<<<<<<< HEAD
use std::collections::HashMap;
use std::io::Cursor;

use ecow::eco_format;
use image::{DynamicImage, GenericImageView, Rgba};
use pdf_writer::{Chunk, Filter, Finish, Ref};
use typst_library::diag::{At, SourceResult, StrResult};
use typst_library::foundations::Smart;
use typst_library::visualize::{
    ColorSpace, ExchangeFormat, Image, ImageKind, ImageScaling, RasterFormat,
    RasterImage, SvgImage,
};
use typst_utils::Deferred;

use crate::{color, deflate, PdfChunk, WithGlobalRefs};

/// Embed all used images into the PDF.
#[typst_macros::time(name = "write images")]
pub fn write_images(
    context: &WithGlobalRefs,
) -> SourceResult<(PdfChunk, HashMap<Image, Ref>)> {
    let mut chunk = PdfChunk::new();
    let mut out = HashMap::new();
    context.resources.traverse(&mut |resources| {
        for (i, image) in resources.images.items().enumerate() {
            if out.contains_key(image) {
                continue;
            }

            let (handle, span) = resources.deferred_images.get(&i).unwrap();
            let encoded = handle.wait().as_ref().map_err(Clone::clone).at(*span)?;

            match encoded {
                EncodedImage::Raster {
                    data,
                    filter,
                    color_space,
                    bits_per_component,
                    width,
                    height,
                    compressed_icc,
                    alpha,
                    interpolate,
                } => {
                    let image_ref = chunk.alloc();
                    out.insert(image.clone(), image_ref);

                    let mut image = chunk.chunk.image_xobject(image_ref, data);
                    image.filter(*filter);
                    image.width(*width as i32);
                    image.height(*height as i32);
                    image.bits_per_component(i32::from(*bits_per_component));
                    image.interpolate(*interpolate);

                    let mut icc_ref = None;
                    let space = image.color_space();
                    if compressed_icc.is_some() {
                        let id = chunk.alloc.bump();
                        space.icc_based(id);
                        icc_ref = Some(id);
                    } else {
                        color::write(
                            *color_space,
                            space,
                            &context.globals.color_functions,
                        );
                    }

                    // Add a second gray-scale image containing the alpha values if
                    // this image has an alpha channel.
                    if let Some((alpha_data, alpha_filter)) = alpha {
                        let mask_ref = chunk.alloc.bump();
                        image.s_mask(mask_ref);
                        image.finish();

                        let mut mask = chunk.image_xobject(mask_ref, alpha_data);
                        mask.filter(*alpha_filter);
                        mask.width(*width as i32);
                        mask.height(*height as i32);
                        mask.color_space().device_gray();
                        mask.bits_per_component(i32::from(*bits_per_component));
                        mask.interpolate(*interpolate);
                    } else {
                        image.finish();
                    }

                    if let (Some(compressed_icc), Some(icc_ref)) =
                        (compressed_icc, icc_ref)
                    {
                        let mut stream = chunk.icc_profile(icc_ref, compressed_icc);
                        stream.filter(Filter::FlateDecode);
                        match color_space {
                            ColorSpace::Srgb => {
                                stream.n(3);
                                stream.alternate().srgb();
                            }
                            ColorSpace::D65Gray => {
                                stream.n(1);
                                stream.alternate().d65_gray();
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                EncodedImage::Svg(svg_chunk, id) => {
                    let mut map = HashMap::new();
                    svg_chunk.renumber_into(&mut chunk.chunk, |old| {
                        *map.entry(old).or_insert_with(|| chunk.alloc.bump())
                    });
                    out.insert(image.clone(), map[id]);
                }
            }
        }

        Ok(())
    })?;

    Ok((chunk, out))
}

/// Creates a new PDF image from the given image.
///
/// Also starts the deferred encoding of the image.
#[comemo::memoize]
pub fn deferred_image(
    image: Image,
    pdfa: bool,
) -> (Deferred<StrResult<EncodedImage>>, Option<ColorSpace>) {
    let color_space = match image.kind() {
        ImageKind::Raster(raster) if raster.icc().is_none() => {
            Some(to_color_space(raster.dynamic().color()))
        }
        _ => None,
    };

    // PDF/A does not appear to allow interpolation.
    // See https://github.com/typst/typst/issues/2942.
    let interpolate = !pdfa && image.scaling() == Smart::Custom(ImageScaling::Smooth);

    let deferred = Deferred::new(move || match image.kind() {
        ImageKind::Raster(raster) => Ok(encode_raster_image(raster, interpolate)),
        ImageKind::Svg(svg) => {
            let (chunk, id) = encode_svg(svg, pdfa)
                .map_err(|err| eco_format!("failed to convert SVG to PDF: {err}"))?;
            Ok(EncodedImage::Svg(chunk, id))
        }
    });

    (deferred, color_space)
}

/// Encode an image with a suitable filter.
#[typst_macros::time(name = "encode raster image")]
fn encode_raster_image(image: &RasterImage, interpolate: bool) -> EncodedImage {
    let dynamic = image.dynamic();
    let color_space = to_color_space(dynamic.color());

    let (filter, data, bits_per_component) =
        if image.format() == RasterFormat::Exchange(ExchangeFormat::Jpg) {
            let mut data = Cursor::new(vec![]);
            dynamic.write_to(&mut data, image::ImageFormat::Jpeg).unwrap();
            (Filter::DctDecode, data.into_inner(), 8)
        } else {
            // TODO: Encode flate streams with PNG-predictor?
            let (data, bits_per_component) = match (dynamic, color_space) {
                // RGB image.
                (DynamicImage::ImageRgb8(rgb), _) => (deflate(rgb.as_raw()), 8),
                // Grayscale image
                (DynamicImage::ImageLuma8(luma), _) => (deflate(luma.as_raw()), 8),
                (_, ColorSpace::D65Gray) => (deflate(dynamic.to_luma8().as_raw()), 8),
                // Anything else
                _ => (deflate(dynamic.to_rgb8().as_raw()), 8),
            };
            (Filter::FlateDecode, data, bits_per_component)
        };

    let compressed_icc = image.icc().map(|data| deflate(data));
    let alpha = dynamic.color().has_alpha().then(|| encode_alpha(dynamic));

    EncodedImage::Raster {
        data,
        filter,
        color_space,
        bits_per_component,
        width: image.width(),
        height: image.height(),
        compressed_icc,
        alpha,
        interpolate,
    }
}

/// Encode an image's alpha channel if present.
#[typst_macros::time(name = "encode alpha")]
fn encode_alpha(image: &DynamicImage) -> (Vec<u8>, Filter) {
    let pixels: Vec<_> = image.pixels().map(|(_, _, Rgba([_, _, _, a]))| a).collect();
    (deflate(&pixels), Filter::FlateDecode)
}

/// Encode an SVG into a chunk of PDF objects.
#[typst_macros::time(name = "encode svg")]
fn encode_svg(
    svg: &SvgImage,
    pdfa: bool,
) -> Result<(Chunk, Ref), svg2pdf::ConversionError> {
    svg2pdf::to_chunk(
        svg.tree(),
        svg2pdf::ConversionOptions { pdfa, ..Default::default() },
    )
}

/// A pre-encoded image.
pub enum EncodedImage {
    /// A pre-encoded rasterized image.
    Raster {
        /// The raw, pre-deflated image data.
        data: Vec<u8>,
        /// The filter to use for the image.
        filter: Filter,
        /// Which color space this image is encoded in.
        color_space: ColorSpace,
        /// How many bits of each color component are stored.
        bits_per_component: u8,
        /// The image's width.
        width: u32,
        /// The image's height.
        height: u32,
        /// The image's ICC profile, deflated, if any.
        compressed_icc: Option<Vec<u8>>,
        /// The alpha channel of the image, pre-deflated, if any.
        alpha: Option<(Vec<u8>, Filter)>,
        /// Whether image interpolation should be enabled.
        interpolate: bool,
    },
    /// A vector graphic.
    ///
    /// The chunk is the SVG converted to PDF objects.
    Svg(Chunk, Ref),
}

/// Matches an [`image::ColorType`] to [`ColorSpace`].
fn to_color_space(color: image::ColorType) -> ColorSpace {
    use image::ColorType::*;
    match color {
        L8 | La8 | L16 | La16 => ColorSpace::D65Gray,
        Rgb8 | Rgba8 | Rgb16 | Rgba16 | Rgb32F | Rgba32F => ColorSpace::Srgb,
        _ => unimplemented!(),
=======
use std::hash::{Hash, Hasher};
use std::sync::{Arc, OnceLock};

use image::{DynamicImage, EncodableLayout, GenericImageView, Rgba};
use krilla::image::{BitsPerComponent, CustomImage, ImageColorspace};
use krilla::pdf::PdfDocument;
use krilla::surface::Surface;
use krilla_svg::{SurfaceExt, SvgSettings};
use typst_library::diag::{SourceResult, bail};
use typst_library::foundations::Smart;
use typst_library::layout::{Abs, Angle, Ratio, Size, Transform};
use typst_library::visualize::{
    ExchangeFormat, Image, ImageKind, ImageScaling, PdfImage, RasterFormat, RasterImage,
};
use typst_syntax::Span;
use typst_utils::defer;

use crate::convert::{FrameContext, GlobalContext};
use crate::tags;
use crate::util::{SizeExt, TransformExt};

#[typst_macros::time(name = "handle image")]
pub(crate) fn handle_image(
    gc: &mut GlobalContext,
    fc: &mut FrameContext,
    image: &Image,
    size: Size,
    surface: &mut Surface,
    span: Span,
) -> SourceResult<()> {
    surface.push_transform(&fc.state().transform().to_krilla());
    surface.set_location(span.into_raw());
    let mut surface = defer(surface, |s| {
        s.pop();
        s.reset_location();
    });

    let interpolate = image.scaling() == Smart::Custom(ImageScaling::Smooth);

    gc.image_spans.insert(span);

    let mut handle = tags::image(gc, fc, &mut surface, image, size);
    let surface = handle.surface();

    match image.kind() {
        ImageKind::Raster(raster) => {
            let (exif_transform, new_size) = exif_transform(raster, size);
            surface.push_transform(&exif_transform.to_krilla());
            let mut surface = defer(surface, |s| s.pop());

            let image = match convert_raster(raster.clone(), interpolate) {
                None => bail!(span, "failed to process image"),
                Some(i) => i,
            };

            if !gc.image_to_spans.contains_key(&image) {
                gc.image_to_spans.insert(image.clone(), span);
            }

            if let Some(size) = new_size.to_krilla() {
                surface.draw_image(image, size);
            }
        }
        ImageKind::Svg(svg) => {
            if let Some(size) = size.to_krilla() {
                surface.draw_svg(
                    svg.tree(),
                    size,
                    SvgSettings { embed_text: true, ..Default::default() },
                );
            }
        }
        ImageKind::Pdf(pdf) => {
            if let Some(size) = size.to_krilla() {
                surface.draw_pdf_page(&convert_pdf(pdf), size, pdf.page_index());
            }
        }
    }

    Ok(())
}

struct Repr {
    /// The original, underlying raster image.
    raster: RasterImage,
    /// The alpha channel of the raster image, if existing.
    alpha_channel: OnceLock<Option<Vec<u8>>>,
    /// A (potentially) converted version of the dynamic image stored `raster` that is
    /// guaranteed to either be in luma8 or rgb8, and thus can be used for the
    /// `color_channel` method of `CustomImage`.
    actual_dynamic: OnceLock<Arc<DynamicImage>>,
}

/// A wrapper around `RasterImage` so that we can implement `CustomImage`.
#[derive(Clone)]
struct PdfRasterImage(Arc<Repr>);

impl PdfRasterImage {
    pub fn new(raster: RasterImage) -> Self {
        Self(Arc::new(Repr {
            raster,
            alpha_channel: OnceLock::new(),
            actual_dynamic: OnceLock::new(),
        }))
    }
}

impl Hash for PdfRasterImage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // `alpha_channel` and `actual_dynamic` are generated from the underlying `RasterImage`,
        // so this is enough. Since `raster` is prehashed, this is also very cheap.
        self.0.raster.hash(state);
    }
}

impl CustomImage for PdfRasterImage {
    fn color_channel(&self) -> &[u8] {
        self.0
            .actual_dynamic
            .get_or_init(|| {
                let dynamic = self.0.raster.dynamic();
                let channel_count = dynamic.color().channel_count();

                match (dynamic.as_ref(), channel_count) {
                    // Pure luma8 or rgb8 image, can use it directly.
                    (DynamicImage::ImageLuma8(_), _) => dynamic.clone(),
                    (DynamicImage::ImageRgb8(_), _) => dynamic.clone(),
                    // Grey-scale image, convert to luma8.
                    (_, 1 | 2) => Arc::new(DynamicImage::ImageLuma8(dynamic.to_luma8())),
                    // Anything else, convert to rgb8.
                    _ => Arc::new(DynamicImage::ImageRgb8(dynamic.to_rgb8())),
                }
            })
            .as_bytes()
    }

    fn alpha_channel(&self) -> Option<&[u8]> {
        self.0
            .alpha_channel
            .get_or_init(|| {
                self.0.raster.dynamic().color().has_alpha().then(|| {
                    self.0
                        .raster
                        .dynamic()
                        .pixels()
                        .map(|(_, _, Rgba([_, _, _, a]))| a)
                        .collect()
                })
            })
            .as_ref()
            .map(|v| &**v)
    }

    fn bits_per_component(&self) -> BitsPerComponent {
        BitsPerComponent::Eight
    }

    fn size(&self) -> (u32, u32) {
        (self.0.raster.width(), self.0.raster.height())
    }

    fn icc_profile(&self) -> Option<&[u8]> {
        if matches!(
            self.0.raster.dynamic().as_ref(),
            DynamicImage::ImageLuma8(_)
                | DynamicImage::ImageLumaA8(_)
                | DynamicImage::ImageRgb8(_)
                | DynamicImage::ImageRgba8(_)
        ) {
            self.0.raster.icc().map(|b| b.as_bytes())
        } else {
            // In all other cases, the dynamic will be converted into RGB8 or LUMA8, so the ICC
            // profile may become invalid, and thus we don't include it.
            None
        }
    }

    fn color_space(&self) -> ImageColorspace {
        // Remember that we convert all images to either RGB or luma.
        if self.0.raster.dynamic().color().has_color() {
            ImageColorspace::Rgb
        } else {
            ImageColorspace::Luma
        }
    }
}

#[comemo::memoize]
fn convert_raster(
    raster: RasterImage,
    interpolate: bool,
) -> Option<krilla::image::Image> {
    if let RasterFormat::Exchange(ExchangeFormat::Jpg) = raster.format() {
        let image_data: Arc<dyn AsRef<[u8]> + Send + Sync> =
            Arc::new(raster.data().clone());
        let icc_profile = raster.icc().map(|i| {
            let i: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(i.clone());
            i
        });

        krilla::image::Image::from_jpeg_with_icc(
            image_data.into(),
            icc_profile.map(|i| i.into()),
            interpolate,
        )
    } else {
        krilla::image::Image::from_custom(PdfRasterImage::new(raster), interpolate)
    }
}

#[comemo::memoize]
fn convert_pdf(pdf: &PdfImage) -> PdfDocument {
    PdfDocument::new(pdf.document().pdf().clone())
}

fn exif_transform(image: &RasterImage, size: Size) -> (Transform, Size) {
    // For JPEGs, we want to apply the EXIF orientation as a transformation
    // because we don't recode them. For other formats, the transform is already
    // baked into the dynamic image data.
    if image.format() != RasterFormat::Exchange(ExchangeFormat::Jpg) {
        return (Transform::identity(), size);
    }

    let base = |hp: bool, vp: bool, mut base_ts: Transform, size: Size| {
        if hp {
            // Flip horizontally in-place.
            base_ts = base_ts.pre_concat(
                Transform::scale(-Ratio::one(), Ratio::one())
                    .pre_concat(Transform::translate(-size.x, Abs::zero())),
            )
        }

        if vp {
            // Flip vertically in-place.
            base_ts = base_ts.pre_concat(
                Transform::scale(Ratio::one(), -Ratio::one())
                    .pre_concat(Transform::translate(Abs::zero(), -size.y)),
            )
        }

        base_ts
    };

    let no_flipping =
        |hp: bool, vp: bool| (base(hp, vp, Transform::identity(), size), size);

    let with_flipping = |hp: bool, vp: bool| {
        let base_ts = Transform::rotate_at(Angle::deg(90.0), Abs::zero(), Abs::zero())
            .pre_concat(Transform::scale(Ratio::one(), -Ratio::one()));
        let inv_size = Size::new(size.y, size.x);
        (base(hp, vp, base_ts, inv_size), inv_size)
    };

    match image.exif_rotation() {
        Some(2) => no_flipping(true, false),
        Some(3) => no_flipping(true, true),
        Some(4) => no_flipping(false, true),
        Some(5) => with_flipping(false, false),
        Some(6) => with_flipping(false, true),
        Some(7) => with_flipping(true, true),
        Some(8) => with_flipping(true, false),
        _ => no_flipping(false, false),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}
