<<<<<<< HEAD
use base64::Engine;
use ecow::{eco_format, EcoString};
use image::{codecs::png::PngEncoder, ImageEncoder};
use typst_library::foundations::Smart;
use typst_library::layout::{Abs, Axes};
use typst_library::visualize::{
    ExchangeFormat, Image, ImageKind, ImageScaling, RasterFormat,
};

use crate::SVGRenderer;

impl SVGRenderer {
    /// Render an image element.
    pub(super) fn render_image(&mut self, image: &Image, size: &Axes<Abs>) {
        let url = convert_image_to_base64_url(image);
        self.xml.start_element("image");
=======
use std::sync::Arc;

use base64::Engine;
use ecow::{EcoString, eco_format};
use hayro::{FontData, FontQuery, InterpreterSettings, StandardFont};
use image::{ImageEncoder, codecs::png::PngEncoder};
use typst_library::foundations::Smart;
use typst_library::layout::{Abs, Axes};
use typst_library::visualize::{
    ExchangeFormat, Image, ImageKind, ImageScaling, PdfImage, RasterFormat,
};

use crate::{SVGRenderer, State, SvgMatrix};

impl SVGRenderer<'_> {
    /// Render an image element.
    pub(super) fn render_image(
        &mut self,
        state: &State,
        image: &Image,
        size: &Axes<Abs>,
    ) {
        let url = convert_image_to_base64_url(image);
        self.xml.start_element("image");
        if !state.transform.is_identity() {
            self.xml.write_attribute("transform", &SvgMatrix(state.transform));
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        self.xml.write_attribute("xlink:href", &url);
        self.xml.write_attribute("width", &size.x.to_pt());
        self.xml.write_attribute("height", &size.y.to_pt());
        self.xml.write_attribute("preserveAspectRatio", "none");
<<<<<<< HEAD
        match image.scaling() {
            Smart::Auto => {}
            Smart::Custom(ImageScaling::Smooth) => {
                // This is still experimental and not implemented in all major browsers.
                // https://developer.mozilla.org/en-US/docs/Web/CSS/image-rendering#browser_compatibility
                self.xml.write_attribute("style", "image-rendering: smooth")
            }
            Smart::Custom(ImageScaling::Pixelated) => {
                self.xml.write_attribute("style", "image-rendering: pixelated")
            }
=======
        if let Some(value) = convert_image_scaling(image.scaling()) {
            self.xml
                .write_attribute("style", &format_args!("image-rendering: {value}"))
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
        self.xml.end_element();
    }
}

<<<<<<< HEAD
=======
/// Converts an image scaling to a CSS `image-rendering` property value.
pub fn convert_image_scaling(scaling: Smart<ImageScaling>) -> Option<&'static str> {
    match scaling {
        Smart::Auto => None,
        Smart::Custom(ImageScaling::Smooth) => {
            // This is still experimental and not implemented in all major browsers.
            // https://developer.mozilla.org/en-US/docs/Web/CSS/image-rendering#browser_compatibility
            Some("smooth")
        }
        Smart::Custom(ImageScaling::Pixelated) => Some("pixelated"),
    }
}

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// Encode an image into a data URL. The format of the URL is
/// `data:image/{format};base64,`.
#[comemo::memoize]
pub fn convert_image_to_base64_url(image: &Image) -> EcoString {
<<<<<<< HEAD
    let mut buf;
=======
    let (mut buf, strbuf);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let (format, data): (&str, &[u8]) = match image.kind() {
        ImageKind::Raster(raster) => match raster.format() {
            RasterFormat::Exchange(format) => (
                match format {
                    ExchangeFormat::Png => "png",
                    ExchangeFormat::Jpg => "jpeg",
                    ExchangeFormat::Gif => "gif",
<<<<<<< HEAD
=======
                    ExchangeFormat::Webp => "webp",
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                },
                raster.data(),
            ),
            RasterFormat::Pixel(_) => ("png", {
                buf = vec![];
                let mut encoder = PngEncoder::new(&mut buf);
                if let Some(icc_profile) = raster.icc() {
                    encoder.set_icc_profile(icc_profile.to_vec()).ok();
                }
                raster.dynamic().write_with_encoder(encoder).unwrap();
                buf.as_slice()
            }),
        },
        ImageKind::Svg(svg) => ("svg+xml", svg.data()),
<<<<<<< HEAD
=======
        ImageKind::Pdf(pdf) => {
            strbuf = pdf_to_svg(pdf);
            ("svg+xml", strbuf.as_bytes())
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    };

    let mut url = eco_format!("data:image/{format};base64,");
    let data = base64::engine::general_purpose::STANDARD.encode(data);
    url.push_str(&data);
    url
}
<<<<<<< HEAD
=======

// Keep this in sync with `typst-png`!
fn pdf_to_svg(pdf: &PdfImage) -> String {
    let select_standard_font = move |font: StandardFont| -> Option<(FontData, u32)> {
        let bytes = match font {
            StandardFont::Helvetica => typst_assets::pdf::SANS,
            StandardFont::HelveticaBold => typst_assets::pdf::SANS_BOLD,
            StandardFont::HelveticaOblique => typst_assets::pdf::SANS_ITALIC,
            StandardFont::HelveticaBoldOblique => typst_assets::pdf::SANS_BOLD_ITALIC,
            StandardFont::Courier => typst_assets::pdf::FIXED,
            StandardFont::CourierBold => typst_assets::pdf::FIXED_BOLD,
            StandardFont::CourierOblique => typst_assets::pdf::FIXED_ITALIC,
            StandardFont::CourierBoldOblique => typst_assets::pdf::FIXED_BOLD_ITALIC,
            StandardFont::TimesRoman => typst_assets::pdf::SERIF,
            StandardFont::TimesBold => typst_assets::pdf::SERIF_BOLD,
            StandardFont::TimesItalic => typst_assets::pdf::SERIF_ITALIC,
            StandardFont::TimesBoldItalic => typst_assets::pdf::SERIF_BOLD_ITALIC,
            StandardFont::ZapfDingBats => typst_assets::pdf::DING_BATS,
            StandardFont::Symbol => typst_assets::pdf::SYMBOL,
        };
        Some((Arc::new(bytes), 0))
    };

    let interpreter_settings = InterpreterSettings {
        font_resolver: Arc::new(move |query| match query {
            FontQuery::Standard(s) => select_standard_font(*s),
            FontQuery::Fallback(f) => select_standard_font(f.pick_standard_font()),
        }),
        warning_sink: Arc::new(|_| {}),
    };

    hayro_svg::convert(pdf.page(), &interpreter_settings)
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
