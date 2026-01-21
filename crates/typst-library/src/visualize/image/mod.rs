//! Image handling.

<<<<<<< HEAD
mod raster;
mod svg;

=======
mod pdf;
mod raster;
mod svg;

pub use self::pdf::PdfImage;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub use self::raster::{
    ExchangeFormat, PixelEncoding, PixelFormat, RasterFormat, RasterImage,
};
pub use self::svg::SvgImage;

<<<<<<< HEAD
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use ecow::EcoString;
use typst_syntax::{Span, Spanned};
use typst_utils::LazyHash;

use crate::diag::{SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, func, scope, Bytes, Cast, Content, Derived, NativeElement, Packed, Show,
    Smart, StyleChain,
};
use crate::layout::{BlockElem, Length, Rel, Sizing};
use crate::loading::{DataSource, Load, Readable};
use crate::model::Figurable;
use crate::text::LocalName;

/// ラスターまたはベクター画像。
///
/// 画像を[`figure`]で囲むことで、番号とキャプションを与えることができます。
///
/// ほとんどの要素と同様に、画像はデフォルトでは _ブロックレベル_ であるため、隣接する段落に統合されることはありません。
/// 画像を強制的にインラインにするには、[`box`]の中に入れてください。
///
/// # 例
=======
use std::ffi::OsStr;
use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroUsize;
use std::sync::Arc;

use ecow::EcoString;
use hayro_syntax::LoadPdfError;
use typst_syntax::{Span, Spanned};
use typst_utils::{LazyHash, NonZeroExt};

use crate::diag::{At, LoadedWithin, SourceResult, StrResult, bail, warning};
use crate::engine::Engine;
use crate::foundations::{
    Bytes, Cast, Content, Derived, NativeElement, Packed, Smart, StyleChain, Synthesize,
    cast, elem, func, scope,
};
use crate::introspection::{Locatable, Tagged};
use crate::layout::{Length, Rel, Sizing};
use crate::loading::{DataSource, Load, LoadSource, Loaded, Readable};
use crate::model::Figurable;
use crate::text::{LocalName, Locale, families};
use crate::visualize::image::pdf::PdfDocument;

/// A raster or vector graphic.
///
/// You can wrap the image in a [`figure`] to give it a number and caption.
///
/// Like most elements, images are _block-level_ by default and thus do not
/// integrate themselves into adjacent paragraphs. To force an image to become
/// inline, put it into a [`box`].
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #figure(
///   image("molecular.jpg", width: 80%),
///   caption: [
///     A step in the molecular testing
///     pipeline of our lab.
///   ],
/// )
/// ```
<<<<<<< HEAD
#[elem(scope, Show, LocalName, Figurable)]
pub struct ImageElem {
    /// 画像ファイルへの[path]($syntax/#paths)、
    /// またはサポートされている[format]($image.format)の画像データの生バイト。
    ///
    /// バイト列を使う場合は、生のピクセルデータを左から右へ、上から下へと並べた
    /// 行優先（row-major）形式で指定します。
=======
#[elem(scope, Locatable, Tagged, Synthesize, LocalName, Figurable)]
pub struct ImageElem {
    /// A [path]($syntax/#paths) to an image file or raw bytes making up an
    /// image in one of the supported [formats]($image.format).
    ///
    /// Bytes can be used to specify raw pixel data in a row-major,
    /// left-to-right, top-to-bottom format.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #let original = read("diagram.svg")
    /// #let changed = original.replace(
    ///   "#2B80FF", // blue
    ///   green.to-hex(),
    /// )
    ///
    /// #image(bytes(original))
    /// #image(bytes(changed))
    /// ```
    #[required]
    #[parse(
        let source = args.expect::<Spanned<DataSource>>("source")?;
<<<<<<< HEAD
        let data = source.load(engine.world)?;
        Derived::new(source.v, data)
    )]
    pub source: Derived<DataSource, Bytes>,

    /// 画像のフォーマット。
    ///
    /// デフォルトでは、フォーマットは自動的に検出されます。
    /// そのため、通常は生のバイト列を[`source`]($image.source)として提供する場合にのみこの指定が必要です
    /// （それでもTypstは自動でフォーマットを判別しようとしますが、
    /// 必ずしも成功するとは限りません）。
    ///
    /// 生のピクセルデータと同様にサポートされている拡張子は`{"png"}`、`{"jpg"}`、`{"gif"}`、`{"svg"}`です。
    /// [PDFの画像はまだサポートされていません。](https://github.com/typst/typst/issues/145)
    ///
    /// 生のピクセルデータを`source`として提供する場合、
    /// `format`には次のキーを持つ辞書を指定する必要があります。
    /// - `encoding` ([str]): ピクセルデータのエンコーディング。以下のいずれかを指定します。
    ///   - `{"rgb8"}` （3つの8ビットチャンネル: 赤（red）、緑（green）、青（blue））
    ///   - `{"rgba8"}` （4つの8ビットチャンネル: 赤（red）、緑（green）、青（blue）、透明度（alpha））
    ///   - `{"luma8"}` （1つの8ビットチャンネル）
    ///   - `{"lumaa8"}` （2つの8ビットチャンネル: 輝度（luma）と透明度（alpha））
    /// - `width` ([int]): 画像の幅のピクセル数。
    /// - `height` ([int]): 画像の高さのピクセル数。
    ///
    /// 幅のピクセル数、高さのピクセル数、指定したエンコーディングにおけるチャンネル数をかけ合わせたものが
    /// `source`のデータと一致しなければなりません。
=======
        let loaded = source.load(engine.world)?;
        Derived::new(source.v, loaded)
    )]
    pub source: Derived<DataSource, Loaded>,

    /// The image's format.
    ///
    /// By default, the format is detected automatically. Typically, you thus
    /// only need to specify this when providing raw bytes as the
    /// [`source`]($image.source) (even then, Typst will try to figure out the
    /// format automatically, but that's not always possible).
    ///
    /// Supported formats are `{"png"}`, `{"jpg"}`, `{"gif"}`, `{"svg"}`,
    /// `{"pdf"}`, `{"webp"}` as well as raw pixel data.
    ///
    /// Note that several restrictions apply when using PDF files as images:
    ///
    /// - When exporting to PDF, any PDF image file used must have a version
    ///   equal to or lower than the [export target PDF
    ///   version]($pdf/#pdf-versions).
    /// - PDF files as images are currently not supported when exporting with a
    ///   specific PDF standard, like PDF/A-3 or PDF/UA-1. In these cases, you
    ///   can instead use SVGs to embed vector images.
    /// - The image file must not be password-protected.
    /// - Tags in your PDF image will not be preserved. Instead, you must
    ///   provide an [alternative description]($image.alt) to make the image
    ///   accessible.
    ///
    /// When providing raw pixel data as the `source`, you must specify a
    /// dictionary with the following keys as the `format`:
    /// - `encoding` ([str]): The encoding of the pixel data. One of:
    ///   - `{"rgb8"}` (three 8-bit channels: red, green, blue)
    ///   - `{"rgba8"}` (four 8-bit channels: red, green, blue, alpha)
    ///   - `{"luma8"}` (one 8-bit channel)
    ///   - `{"lumaa8"}` (two 8-bit channels: luma and alpha)
    /// - `width` ([int]): The pixel width of the image.
    /// - `height` ([int]): The pixel height of the image.
    ///
    /// The pixel width multiplied by the height multiplied by the channel count
    /// for the specified encoding must then match the `source` data.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #image(
    ///   read(
    ///     "tetrahedron.svg",
    ///     encoding: none,
    ///   ),
    ///   format: "svg",
    ///   width: 2cm,
    /// )
    ///
    /// #image(
    ///   bytes(range(16).map(x => x * 16)),
    ///   format: (
    ///     encoding: "luma8",
    ///     width: 4,
    ///     height: 4,
    ///   ),
    ///   width: 2cm,
    /// )
    /// ```
    pub format: Smart<ImageFormat>,

<<<<<<< HEAD
    /// 画像の幅。
    pub width: Smart<Rel<Length>>,

    /// 画像の高さ。
    pub height: Sizing,

    /// 画像の説明文。
    pub alt: Option<EcoString>,

    /// 与えられた領域に対して、画像をどのように調整するか。
    /// 領域は `width` や `height` フィールドで定義します。
    /// 領域の縦横比が画像の縦横比と同じであれば、`fit` で見た目が変わらないことに注意してください。
=======
    /// The width of the image.
    pub width: Smart<Rel<Length>>,

    /// The height of the image.
    pub height: Sizing,

    /// An alternative description of the image.
    ///
    /// This text is used by Assistive Technology (AT) like screen readers to
    /// describe the image to users with visual impairments.
    ///
    /// When the image is wrapped in a [`figure`]($figure), use this parameter
    /// rather than the [figure's `alt` parameter]($figure.alt) to describe the
    /// image. The only exception to this rule is when the image and the other
    /// contents in the figure form a single semantic unit. In this case, use
    /// the figure's `alt` parameter to describe the entire composition and do
    /// not use this parameter.
    ///
    /// You can learn how to write good alternative descriptions in the
    /// [Accessibility Guide]($guides/accessibility/#textual-representations).
    pub alt: Option<EcoString>,

    /// The page number that should be embedded as an image. This attribute only
    /// has an effect for PDF files.
    #[default(NonZeroUsize::ONE)]
    pub page: NonZeroUsize,

    /// How the image should adjust itself to a given area (the area is defined
    /// by the `width` and `height` fields). Note that `fit` doesn't visually
    /// change anything if the area's aspect ratio is the same as the image's
    /// one.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set page(width: 300pt, height: 50pt, margin: 10pt)
    /// #image("tiger.jpg", width: 100%, fit: "cover")
    /// #image("tiger.jpg", width: 100%, fit: "contain")
    /// #image("tiger.jpg", width: 100%, fit: "stretch")
    /// ```
    #[default(ImageFit::Cover)]
    pub fit: ImageFit,

<<<<<<< HEAD
    /// ビューアーに対して、画像をどのように拡大縮小すべきかを示すヒント。
    ///
    /// `{auto}`に設定した場合、デフォルトの動作はビューアーに委ねられます。
    /// PNGエクスポートの場合、TypstはほとんどのPDFビューアーやSVGビューアーと同様に、
    /// スムーズな拡大縮小をデフォルトとして設定します。
    ///
    /// _注意:_ PDFビューアーによっては正確な見た目が異なる場合があります。
    pub scaling: Smart<ImageScaling>,

    /// 画像用のICCプロファイル。
    ///
    /// ICCプロファイルは、画像の色をどのように解釈するかを定義するものです。
    /// `{auto}`に設定した場合、Typstは画像からICCプロファイルを抽出しようとします。
    #[parse(match args.named::<Spanned<Smart<DataSource>>>("icc")? {
        Some(Spanned { v: Smart::Custom(source), span }) => Some(Smart::Custom({
            let data = Spanned::new(&source, span).load(engine.world)?;
            Derived::new(source, data)
=======
    /// A hint to viewers how they should scale the image.
    ///
    /// When set to `{auto}`, the default is left up to the viewer. For PNG
    /// export, Typst will default to smooth scaling, like most PDF and SVG
    /// viewers.
    ///
    /// _Note:_ The exact look may differ across PDF viewers.
    pub scaling: Smart<ImageScaling>,

    /// An ICC profile for the image.
    ///
    /// ICC profiles define how to interpret the colors in an image. When set
    /// to `{auto}`, Typst will try to extract an ICC profile from the image.
    #[parse(match args.named::<Spanned<Smart<DataSource>>>("icc")? {
        Some(Spanned { v: Smart::Custom(source), span }) => Some(Smart::Custom({
            let loaded = Spanned::new(&source, span).load(engine.world)?;
            Derived::new(source, loaded.data)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        })),
        Some(Spanned { v: Smart::Auto, .. }) => Some(Smart::Auto),
        None => None,
    })]
<<<<<<< HEAD
    #[borrowed]
    pub icc: Smart<Derived<DataSource, Bytes>>,
=======
    pub icc: Smart<Derived<DataSource, Bytes>>,

    /// The locale of this element (used for the alternative description).
    #[internal]
    #[synthesized]
    pub locale: Locale,
}

impl Synthesize for Packed<ImageElem> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        self.locale = Some(Locale::get_in(styles));
        Ok(())
    }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

#[scope]
#[allow(clippy::too_many_arguments)]
impl ImageElem {
<<<<<<< HEAD
    /// バイト列または文字列からラスター画像またはベクター画像をデコードする。
    #[func(title = "Decode Image")]
    #[deprecated = "`image.decode`は非推奨です。代わりにバイト列を直接`image`に渡してください。"]
    pub fn decode(
        span: Span,
        /// 画像としてデコードするデータ。SVGの場合は文字列です。
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
        /// ビューアーがどのように拡大縮小すべきかを示すヒント。
        #[named]
        scaling: Option<Smart<ImageScaling>>,
    ) -> StrResult<Content> {
        let bytes = data.into_bytes();
        let source = Derived::new(DataSource::Bytes(bytes.clone()), bytes);
        let mut elem = ImageElem::new(source);
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
        if let Some(scaling) = scaling {
            elem.push_scaling(scaling);
=======
    /// Decode a raster or vector graphic from bytes or a string.
    #[func(title = "Decode Image")]
    #[deprecated(
        message = "`image.decode` is deprecated, directly pass bytes to `image` instead",
        until = "0.15.0"
    )]
    pub fn decode(
        span: Span,
        /// The data to decode as an image. Can be a string for SVGs.
        data: Spanned<Readable>,
        /// The image's format. Detected automatically by default.
        #[named]
        format: Option<Smart<ImageFormat>>,
        /// The width of the image.
        #[named]
        width: Option<Smart<Rel<Length>>>,
        /// The height of the image.
        #[named]
        height: Option<Sizing>,
        /// A text describing the image.
        #[named]
        alt: Option<Option<EcoString>>,
        /// How the image should adjust itself to a given area.
        #[named]
        fit: Option<ImageFit>,
        /// A hint to viewers how they should scale the image.
        #[named]
        scaling: Option<Smart<ImageScaling>>,
    ) -> StrResult<Content> {
        let bytes = data.v.into_bytes();
        let loaded =
            Loaded::new(Spanned::new(LoadSource::Bytes, data.span), bytes.clone());
        let source = Derived::new(DataSource::Bytes(bytes), loaded);
        let mut elem = ImageElem::new(source);
        if let Some(format) = format {
            elem.format.set(format);
        }
        if let Some(width) = width {
            elem.width.set(width);
        }
        if let Some(height) = height {
            elem.height.set(height);
        }
        if let Some(alt) = alt {
            elem.alt.set(alt);
        }
        if let Some(fit) = fit {
            elem.fit.set(fit);
        }
        if let Some(scaling) = scaling {
            elem.scaling.set(scaling);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
        Ok(elem.pack().spanned(span))
    }
}

<<<<<<< HEAD
impl Show for Packed<ImageElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_image)
            .with_width(self.width(styles))
            .with_height(self.height(styles))
            .pack()
            .spanned(self.span()))
=======
impl Packed<ImageElem> {
    /// Decodes the image.
    pub fn decode(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Image> {
        let span = self.span();
        let loaded = &self.source.derived;
        let format = self.determine_format(styles).at(span)?;

        // Construct the image itself.
        let kind = match format {
            ImageFormat::Raster(format) => ImageKind::Raster(
                RasterImage::new(
                    loaded.data.clone(),
                    format,
                    self.icc.get_ref(styles).as_ref().map(|icc| icc.derived.clone()),
                )
                .at(span)?,
            ),
            ImageFormat::Vector(VectorFormat::Svg) => {
                // Warn the user if the image contains a foreign object. Not
                // perfect because the svg could also be encoded, but that's an
                // edge case.
                if memchr::memmem::find(&loaded.data, b"<foreignObject").is_some() {
                    engine.sink.warn(warning!(
                        span,
                        "image contains foreign object";
                        hint: "SVG images with foreign objects might render incorrectly in Typst";
                        hint: "see https://github.com/typst/typst/issues/1421 for more information"
                    ));
                }

                // Identify the SVG file in case contained hrefs need to be resolved.
                let svg_file = match self.source.source {
                    DataSource::Path(ref path) => span.resolve_path(path).ok(),
                    DataSource::Bytes(_) => span.id(),
                };
                ImageKind::Svg(
                    SvgImage::with_fonts_images(
                        loaded.data.clone(),
                        engine.world,
                        &families(styles).map(|f| f.as_str()).collect::<Vec<_>>(),
                        svg_file,
                    )
                    .within(loaded)?,
                )
            }
            ImageFormat::Vector(VectorFormat::Pdf) => {
                let document = match PdfDocument::new(loaded.data.clone()) {
                    Ok(doc) => doc,
                    Err(e) => match e {
                        // TODO: the `DecyptionError` is currently not public
                        LoadPdfError::Decryption(_) => {
                            bail!(
                                span,
                                "the PDF is encrypted or password-protected";
                                hint: "such PDFs are currently not supported";
                                hint: "preprocess the PDF to remove the encryption"
                            );
                        }
                        LoadPdfError::Invalid => {
                            bail!(
                                span,
                                "the PDF could not be loaded";
                                hint: "perhaps the PDF file is malformed"
                            );
                        }
                    },
                };

                // See https://github.com/LaurenzV/hayro/issues/141.
                if document.pdf().xref().has_optional_content_groups() {
                    engine.sink.warn(warning!(
                        span,
                        "PDF contains optional content groups";
                        hint: "the image might display incorrectly in PDF export";
                        hint: "preprocess the PDF to flatten or remove optional content groups"
                    ));
                }

                // The user provides the page number start from 1, but further
                // down the pipeline, page numbers are 0-based.
                let page_num = self.page.get(styles).get();
                let page_idx = page_num - 1;
                let num_pages = document.num_pages();

                let Some(pdf_image) = PdfImage::new(document, page_idx) else {
                    let s = if num_pages == 1 { "" } else { "s" };
                    bail!(
                        span,
                        "page {page_num} does not exist";
                        hint: "the document only has {num_pages} page{s}"
                    );
                };

                ImageKind::Pdf(pdf_image)
            }
        };

        Ok(Image::new(kind, self.alt.get_cloned(styles), self.scaling.get(styles)))
    }

    /// Tries to determine the image format based on the format that was
    /// explicitly defined, or else the extension, or else the data.
    fn determine_format(&self, styles: StyleChain) -> StrResult<ImageFormat> {
        if let Smart::Custom(v) = self.format.get(styles) {
            return Ok(v);
        };

        let Derived { source, derived: loaded } = &self.source;
        if let DataSource::Path(path) = source
            && let Some(format) = determine_format_from_path(path.as_str())
        {
            return Ok(format);
        }

        Ok(ImageFormat::detect(&loaded.data).ok_or("unknown image format")?)
    }
}

/// Derive the image format from the file extension of a path.
fn determine_format_from_path(path: &str) -> Option<ImageFormat> {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_lowercase();

    match ext.as_str() {
        // Raster formats
        "png" => Some(ExchangeFormat::Png.into()),
        "jpg" | "jpeg" => Some(ExchangeFormat::Jpg.into()),
        "gif" => Some(ExchangeFormat::Gif.into()),
        "webp" => Some(ExchangeFormat::Webp.into()),
        // Vector formats
        "svg" | "svgz" => Some(VectorFormat::Svg.into()),
        "pdf" => Some(VectorFormat::Pdf.into()),
        _ => None,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

impl LocalName for Packed<ImageElem> {
    const KEY: &'static str = "figure";
}

impl Figurable for Packed<ImageElem> {}

/// How an image should adjust itself to a given area,
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum ImageFit {
<<<<<<< HEAD
    /// 領域を完全にカバーします。
    /// 水平または垂直方向にのみ画像をトリミングすることで、アスペクト比を保持します。
    /// これがデフォルトです。
    Cover,
    /// 画像は領域内に完全に収まるようにします。
    /// アスペクト比を維持して、画像を切り取らず、1つの寸法は指定より狭くします。
    Contain,
    /// たとえ画像が歪むことになっても、その領域を正確に埋めるように引き伸ばします。
    /// アスペクト比は保たれず、画像は切り取られません。
=======
    /// The image should completely cover the area (preserves aspect ratio by
    /// cropping the image only horizontally or vertically). This is the
    /// default.
    Cover,
    /// The image should be fully contained in the area (preserves aspect
    /// ratio; doesn't crop the image; one dimension can be narrower than
    /// specified).
    Contain,
    /// The image should be stretched so that it exactly fills the area, even if
    /// this means that the image will be distorted (doesn't preserve aspect
    /// ratio and doesn't crop the image).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Stretch,
}

/// A loaded raster or vector image.
///
/// Values of this type are cheap to clone and hash.
<<<<<<< HEAD
#[derive(Clone, Hash, Eq, PartialEq)]
=======
#[derive(Clone, Eq, PartialEq, Hash)]
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub struct Image(Arc<LazyHash<Repr>>);

/// The internal representation.
#[derive(Hash)]
struct Repr {
    /// The raw, undecoded image data.
    kind: ImageKind,
    /// A text describing the image.
    alt: Option<EcoString>,
    /// The scaling algorithm to use.
    scaling: Smart<ImageScaling>,
}

impl Image {
    /// When scaling an image to it's natural size, we default to this DPI
    /// if the image doesn't contain DPI metadata.
    pub const DEFAULT_DPI: f64 = 72.0;

    /// Should always be the same as the default DPI used by usvg.
    pub const USVG_DEFAULT_DPI: f64 = 96.0;

    /// Create an image from a `RasterImage` or `SvgImage`.
    pub fn new(
        kind: impl Into<ImageKind>,
        alt: Option<EcoString>,
        scaling: Smart<ImageScaling>,
    ) -> Self {
        Self::new_impl(kind.into(), alt, scaling)
    }

    /// Create an image with optional properties set to the default.
    pub fn plain(kind: impl Into<ImageKind>) -> Self {
        Self::new(kind, None, Smart::Auto)
    }

    /// The internal, non-generic implementation. This is memoized to reuse
    /// the `Arc` and `LazyHash`.
    #[comemo::memoize]
    fn new_impl(
        kind: ImageKind,
        alt: Option<EcoString>,
        scaling: Smart<ImageScaling>,
    ) -> Image {
        Self(Arc::new(LazyHash::new(Repr { kind, alt, scaling })))
    }

    /// The format of the image.
    pub fn format(&self) -> ImageFormat {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.format().into(),
            ImageKind::Svg(_) => VectorFormat::Svg.into(),
<<<<<<< HEAD
=======
            ImageKind::Pdf(_) => VectorFormat::Pdf.into(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }

    /// The width of the image in pixels.
    pub fn width(&self) -> f64 {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.width() as f64,
            ImageKind::Svg(svg) => svg.width(),
<<<<<<< HEAD
=======
            ImageKind::Pdf(pdf) => pdf.width() as f64,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }

    /// The height of the image in pixels.
    pub fn height(&self) -> f64 {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.height() as f64,
            ImageKind::Svg(svg) => svg.height(),
<<<<<<< HEAD
=======
            ImageKind::Pdf(pdf) => pdf.height() as f64,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }

    /// The image's pixel density in pixels per inch, if known.
    pub fn dpi(&self) -> Option<f64> {
        match &self.0.kind {
            ImageKind::Raster(raster) => raster.dpi(),
            ImageKind::Svg(_) => Some(Image::USVG_DEFAULT_DPI),
<<<<<<< HEAD
=======
            ImageKind::Pdf(_) => Some(Image::DEFAULT_DPI),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }

    /// A text describing the image.
    pub fn alt(&self) -> Option<&str> {
        self.0.alt.as_deref()
    }

    /// The image scaling algorithm to use for this image.
    pub fn scaling(&self) -> Smart<ImageScaling> {
        self.0.scaling
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
            .field("scaling", &self.scaling())
            .finish()
    }
}

/// A kind of image.
#[derive(Clone, Hash)]
pub enum ImageKind {
    /// A raster image.
    Raster(RasterImage),
    /// An SVG image.
    Svg(SvgImage),
<<<<<<< HEAD
=======
    /// A PDF image.
    Pdf(PdfImage),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

impl From<RasterImage> for ImageKind {
    fn from(image: RasterImage) -> Self {
        Self::Raster(image)
    }
}

impl From<SvgImage> for ImageKind {
    fn from(image: SvgImage) -> Self {
        Self::Svg(image)
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

impl ImageFormat {
    /// Try to detect the format of an image from data.
    pub fn detect(data: &[u8]) -> Option<Self> {
        if let Some(format) = ExchangeFormat::detect(data) {
            return Some(Self::Raster(RasterFormat::Exchange(format)));
        }

        if is_svg(data) {
            return Some(Self::Vector(VectorFormat::Svg));
        }

<<<<<<< HEAD
=======
        if is_pdf(data) {
            return Some(Self::Vector(VectorFormat::Pdf));
        }

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        None
    }
}

<<<<<<< HEAD
=======
/// Checks whether the data looks like a PDF file.
fn is_pdf(data: &[u8]) -> bool {
    let head = &data[..data.len().min(2048)];
    memchr::memmem::find(head, b"%PDF-").is_some()
}

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// Checks whether the data looks like an SVG or a compressed SVG.
fn is_svg(data: &[u8]) -> bool {
    // Check for the gzip magic bytes. This check is perhaps a bit too
    // permissive as other formats than SVGZ could use gzip.
    if data.starts_with(&[0x1f, 0x8b]) {
        return true;
    }

    // If the first 2048 bytes contain the SVG namespace declaration, we assume
    // that it's an SVG. Note that, if the SVG does not contain a namespace
    // declaration, usvg will reject it.
    let head = &data[..data.len().min(2048)];
    memchr::memmem::find(head, b"http://www.w3.org/2000/svg").is_some()
}

/// A vector graphics format.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum VectorFormat {
<<<<<<< HEAD
    /// Webサイトに用いられるベクターフォーマット。
    Svg,
=======
    /// The vector graphics format of the web.
    Svg,
    /// High-fidelity document and graphics format, with focus on exact
    /// reproduction in print.
    Pdf,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

impl<R> From<R> for ImageFormat
where
    R: Into<RasterFormat>,
{
    fn from(format: R) -> Self {
        Self::Raster(format.into())
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
        Self::Vector(v) => v.into_value(),
    },
    v: RasterFormat => Self::Raster(v),
    v: VectorFormat => Self::Vector(v),
}

/// The image scaling algorithm a viewer should use.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum ImageScaling {
<<<<<<< HEAD
    /// バイリニア補間などの平滑化アルゴリズムを用いて拡大縮小します。
    Smooth,
    /// 最近傍補間などのアルゴリズムで拡大縮小し、
    /// ピクセルで構成された画像の見た目を保ちます。
=======
    /// Scale with a smoothing algorithm such as bilinear interpolation.
    Smooth,
    /// Scale with nearest neighbor or a similar algorithm to preserve the
    /// pixelated look of the image.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Pixelated,
}
