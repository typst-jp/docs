//! Image handling.

mod raster;
mod svg;

pub use self::raster::{RasterFormat, RasterImage};
pub use self::svg::SvgImage;

use std::ffi::OsStr;
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use comemo::Tracked;
use ecow::EcoString;

use crate::diag::{bail, warning, At, SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, func, scope, Bytes, Cast, Content, NativeElement, Packed, Show, Smart,
    StyleChain,
};
use crate::introspection::Locator;
use crate::layout::{
    Abs, Axes, BlockElem, FixedAlignment, Frame, FrameItem, Length, Point, Region, Rel,
    Size, Sizing,
};
use crate::loading::Readable;
use crate::model::Figurable;
use crate::syntax::{Span, Spanned};
use crate::text::{families, LocalName};
use crate::utils::LazyHash;
use crate::visualize::Path;
use crate::World;

/// ラスターまたはベクター画像。
///
/// 画像を[`figure`]で囲むことで、番号とキャプションを与えることができます。
///
/// ほとんどの要素と同様に、画像はデフォルトでは _ブロックレベル_ であるため、隣接する段落に統合されることはありません。
/// 画像を強制的にインラインにするには、[`box`]の中に入れてください。
///
/// # Example
/// ```example
/// #figure(
///   image("molecular.jpg", width: 80%),
///   caption: [
///     A step in the molecular testing
///     pipeline of our lab.
///   ],
/// )
/// ```
#[elem(scope, Show, LocalName, Figurable)]
pub struct ImageElem {
    /// 画像ファイルのパス。
    ///
    /// より詳細な情報は[パスの章]($syntax/#paths)を参照してください。
    #[required]
    #[parse(
        let Spanned { v: path, span } =
            args.expect::<Spanned<EcoString>>("path to image file")?;
        let id = span.resolve_path(&path).at(span)?;
        let data = engine.world.file(id).at(span)?;
        path
    )]
    #[borrowed]
    pub path: EcoString,

    /// The raw file data.
    #[internal]
    #[required]
    #[parse(Readable::Bytes(data))]
    pub data: Readable,

    /// 画像のフォーマット。デフォルトでは自動的に検出されます。
    ///
    /// サポートされている拡張子は PNG, JPEG, GIF, SVGです。
    /// [PDFの画像はまだサポートされていません。](https://github.com/typst/typst/issues/145)
    pub format: Smart<ImageFormat>,

    /// 画像の幅。
    pub width: Smart<Rel<Length>>,

    /// 画像の高さ。
    pub height: Sizing,

    /// 画像の説明文。
    pub alt: Option<EcoString>,

    /// 与えられた領域に対して、画像をどのように調整するか。
    /// 領域は `width` や `height` フィールドで定義します。
    /// 領域の縦横比が画像の縦横比と同じであれば、`fit` で見た目が変わらないことに注意してください。
    ///
    /// ```example
    /// #set page(width: 300pt, height: 50pt, margin: 10pt)
    /// #image("tiger.jpg", width: 100%, fit: "cover")
    /// #image("tiger.jpg", width: 100%, fit: "contain")
    /// #image("tiger.jpg", width: 100%, fit: "stretch")
    /// ```
    #[default(ImageFit::Cover)]
    pub fit: ImageFit,
}

#[scope]
impl ImageElem {
    /// バイトまたは文字列からラスターまたはベクトル図形をデコードします。
    ///
    /// ```example
    /// #let original = read("diagram.svg")
    /// #let changed = original.replace(
    ///   "#2B80FF", // blue
    ///   green.to-hex(),
    /// )
    ///
    /// #image.decode(original)
    /// #image.decode(changed)
    /// ```
    #[func(title = "Decode Image")]
    pub fn decode(
        /// The call span of this function.
        span: Span,
        /// 画像としてデコードするデータ。SVG の場合は文字列です。
        data: Readable,
        /// 画像のフォーマット。デフォルトでは自動的に検出されます。
        #[named]
        format: Option<Smart<ImageFormat>>,
        /// 画像の幅。
        #[named]
        width: Option<Smart<Rel<Length>>>,
        /// 画像の高さ。
        #[named]
        height: Option<Sizing>,
        /// 画像の説明文。
        #[named]
        alt: Option<Option<EcoString>>,
        /// 与えられた領域に対して、画像をどのように調整するか。
        #[named]
        fit: Option<ImageFit>,
    ) -> StrResult<Content> {
        let mut elem = ImageElem::new(EcoString::new(), data);
        if let Some(format) = format {
            elem.push_format(format);
        }
        if let Some(width) = width {
            elem.push_width(width);
        }
        if let Some(height) = height {
            elem.push_height(height);
        }
        if let Some(alt) = alt {
            elem.push_alt(alt);
        }
        if let Some(fit) = fit {
            elem.push_fit(fit);
        }
        Ok(elem.pack().spanned(span))
    }
}

impl Show for Packed<ImageElem> {
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), layout_image)
            .with_width(self.width(styles))
            .with_height(self.height(styles))
            .pack()
            .spanned(self.span()))
    }
}

impl LocalName for Packed<ImageElem> {
    const KEY: &'static str = "figure";
}

impl Figurable for Packed<ImageElem> {}

/// Layout the image.
#[typst_macros::time(span = elem.span())]
fn layout_image(
    elem: &Packed<ImageElem>,
    engine: &mut Engine,
    _: Locator,
    styles: StyleChain,
    region: Region,
) -> SourceResult<Frame> {
    let span = elem.span();

    // Take the format that was explicitly defined, or parse the extension,
    // or try to detect the format.
    let data = elem.data();
    let format = match elem.format(styles) {
        Smart::Custom(v) => v,
        Smart::Auto => determine_format(elem.path().as_str(), data).at(span)?,
    };

    // Warn the user if the image contains a foreign object. Not perfect
    // because the svg could also be encoded, but that's an edge case.
    if format == ImageFormat::Vector(VectorFormat::Svg) {
        let has_foreign_object =
            data.as_str().is_some_and(|s| s.contains("<foreignObject"));

        if has_foreign_object {
            engine.sink.warn(warning!(
                span,
                "image contains foreign object";
                hint: "SVG images with foreign objects might render incorrectly in typst";
                hint: "see https://github.com/typst/typst/issues/1421 for more information"
            ));
        }
    }

    // Construct the image itself.
    let image = Image::with_fonts(
        data.clone().into(),
        format,
        elem.alt(styles),
        engine.world,
        &families(styles).collect::<Vec<_>>(),
    )
    .at(span)?;

    // Determine the image's pixel aspect ratio.
    let pxw = image.width();
    let pxh = image.height();
    let px_ratio = pxw / pxh;

    // Determine the region's aspect ratio.
    let region_ratio = region.size.x / region.size.y;

    // Find out whether the image is wider or taller than the region.
    let wide = px_ratio > region_ratio;

    // The space into which the image will be placed according to its fit.
    let target = if region.expand.x && region.expand.y {
        // If both width and height are forced, take them.
        region.size
    } else if region.expand.x {
        // If just width is forced, take it.
        Size::new(region.size.x, region.size.y.min(region.size.x / px_ratio))
    } else if region.expand.y {
        // If just height is forced, take it.
        Size::new(region.size.x.min(region.size.y * px_ratio), region.size.y)
    } else {
        // If neither is forced, take the natural image size at the image's
        // DPI bounded by the available space.
        let dpi = image.dpi().unwrap_or(Image::DEFAULT_DPI);
        let natural = Axes::new(pxw, pxh).map(|v| Abs::inches(v / dpi));
        Size::new(
            natural.x.min(region.size.x).min(region.size.y * px_ratio),
            natural.y.min(region.size.y).min(region.size.x / px_ratio),
        )
    };

    // Compute the actual size of the fitted image.
    let fit = elem.fit(styles);
    let fitted = match fit {
        ImageFit::Cover | ImageFit::Contain => {
            if wide == (fit == ImageFit::Contain) {
                Size::new(target.x, target.x / px_ratio)
            } else {
                Size::new(target.y * px_ratio, target.y)
            }
        }
        ImageFit::Stretch => target,
    };

    // First, place the image in a frame of exactly its size and then resize
    // the frame to the target size, center aligning the image in the
    // process.
    let mut frame = Frame::soft(fitted);
    frame.push(Point::zero(), FrameItem::Image(image, fitted, span));
    frame.resize(target, Axes::splat(FixedAlignment::Center));

    // Create a clipping group if only part of the image should be visible.
    if fit == ImageFit::Cover && !target.fits(fitted) {
        frame.clip(Path::rect(frame.size()));
    }

    Ok(frame)
}

/// Determine the image format based on path and data.
fn determine_format(path: &str, data: &Readable) -> StrResult<ImageFormat> {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_lowercase();

    Ok(match ext.as_str() {
        "png" => ImageFormat::Raster(RasterFormat::Png),
        "jpg" | "jpeg" => ImageFormat::Raster(RasterFormat::Jpg),
        "gif" => ImageFormat::Raster(RasterFormat::Gif),
        "svg" | "svgz" => ImageFormat::Vector(VectorFormat::Svg),
        _ => match &data {
            Readable::Str(_) => ImageFormat::Vector(VectorFormat::Svg),
            Readable::Bytes(bytes) => match RasterFormat::detect(bytes) {
                Some(f) => ImageFormat::Raster(f),
                None => bail!("unknown image format"),
            },
        },
    })
}

/// How an image should adjust itself to a given area,
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum ImageFit {
    /// 領域を完全にカバーします。
    /// 水平または垂直方向にのみ画像をトリミングすることで、アスペクト比を保持します。
    /// これがデフォルトです。
    Cover,
    /// 画像は領域内に完全に収まるようにします。
    /// アスペクト比を維持して、画像を切り取らず、1つの寸法は指定より狭くします。
    Contain,
    /// たとえ画像が歪むことになっても、その領域を正確に埋めるように引き伸ばします。
    /// アスペクト比は保たれず、画像は切り取られません。
    Stretch,
}

/// A loaded raster or vector image.
///
/// Values of this type are cheap to clone and hash.
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Image(Arc<LazyHash<Repr>>);

/// The internal representation.
#[derive(Hash)]
struct Repr {
    /// The raw, undecoded image data.
    kind: ImageKind,
    /// A text describing the image.
    alt: Option<EcoString>,
}

/// A kind of image.
#[derive(Hash)]
pub enum ImageKind {
    /// A raster image.
    Raster(RasterImage),
    /// An SVG image.
    Svg(SvgImage),
}

impl Image {
    /// When scaling an image to it's natural size, we default to this DPI
    /// if the image doesn't contain DPI metadata.
    pub const DEFAULT_DPI: f64 = 72.0;

    /// Should always be the same as the default DPI used by usvg.
    pub const USVG_DEFAULT_DPI: f64 = 96.0;

    /// Create an image from a buffer and a format.
    #[comemo::memoize]
    #[typst_macros::time(name = "load image")]
    pub fn new(
        data: Bytes,
        format: ImageFormat,
        alt: Option<EcoString>,
    ) -> StrResult<Image> {
        let kind = match format {
            ImageFormat::Raster(format) => {
                ImageKind::Raster(RasterImage::new(data, format)?)
            }
            ImageFormat::Vector(VectorFormat::Svg) => {
                ImageKind::Svg(SvgImage::new(data)?)
            }
        };

        Ok(Self(Arc::new(LazyHash::new(Repr { kind, alt }))))
    }

    /// Create a possibly font-dependent image from a buffer and a format.
    #[comemo::memoize]
    #[typst_macros::time(name = "load image")]
    pub fn with_fonts(
        data: Bytes,
        format: ImageFormat,
        alt: Option<EcoString>,
        world: Tracked<dyn World + '_>,
        families: &[&str],
    ) -> StrResult<Image> {
        let kind = match format {
            ImageFormat::Raster(format) => {
                ImageKind::Raster(RasterImage::new(data, format)?)
            }
            ImageFormat::Vector(VectorFormat::Svg) => {
                ImageKind::Svg(SvgImage::with_fonts(data, world, families)?)
            }
        };

        Ok(Self(Arc::new(LazyHash::new(Repr { kind, alt }))))
    }

    /// The raw image data.
    pub fn data(&self) -> &Bytes {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.data(),
            ImageKind::Svg(svg) => svg.data(),
        }
    }

    /// The format of the image.
    pub fn format(&self) -> ImageFormat {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.format().into(),
            ImageKind::Svg(_) => VectorFormat::Svg.into(),
        }
    }

    /// The width of the image in pixels.
    pub fn width(&self) -> f64 {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.width() as f64,
            ImageKind::Svg(svg) => svg.width(),
        }
    }

    /// The height of the image in pixels.
    pub fn height(&self) -> f64 {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.height() as f64,
            ImageKind::Svg(svg) => svg.height(),
        }
    }

    /// The image's pixel density in pixels per inch, if known.
    pub fn dpi(&self) -> Option<f64> {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.dpi(),
            ImageKind::Svg(_) => Some(Image::USVG_DEFAULT_DPI),
        }
    }

    /// A text describing the image.
    pub fn alt(&self) -> Option<&str> {
        self.0.alt.as_deref()
    }

    /// The decoded image.
    pub fn kind(&self) -> &ImageKind {
        &self.0.kind
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Image")
            .field("format", &self.format())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("alt", &self.alt())
            .finish()
    }
}

/// A raster or vector image format.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ImageFormat {
    /// A raster graphics format.
    Raster(RasterFormat),
    /// A vector graphics format.
    Vector(VectorFormat),
}

/// A vector graphics format.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum VectorFormat {
    /// Webサイトに用いられるベクターフォーマット。
    Svg,
}

impl From<RasterFormat> for ImageFormat {
    fn from(format: RasterFormat) -> Self {
        Self::Raster(format)
    }
}

impl From<VectorFormat> for ImageFormat {
    fn from(format: VectorFormat) -> Self {
        Self::Vector(format)
    }
}

cast! {
    ImageFormat,
    self => match self {
        Self::Raster(v) => v.into_value(),
        Self::Vector(v) => v.into_value()
    },
    v: RasterFormat => Self::Raster(v),
    v: VectorFormat => Self::Vector(v),
}
