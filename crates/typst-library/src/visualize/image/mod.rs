//! Image handling.

mod raster;
mod svg;

pub use self::raster::{
    ExchangeFormat, PixelEncoding, PixelFormat, RasterFormat, RasterImage,
};
pub use self::svg::SvgImage;

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
    /// 画像ファイルへの[path]($syntax/#paths)、
    /// またはサポートされている[format]($image.format)の画像データの生バイト。
    ///
    /// バイト列を使う場合は、生のピクセルデータを左から右へ、上から下へと並べた
    /// 行優先（row-major）形式で指定します。
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
        })),
        Some(Spanned { v: Smart::Auto, .. }) => Some(Smart::Auto),
        None => None,
    })]
    #[borrowed]
    pub icc: Smart<Derived<DataSource, Bytes>>,
}

#[scope]
#[allow(clippy::too_many_arguments)]
impl ImageElem {
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
        }
        Ok(elem.pack().spanned(span))
    }
}

impl Show for Packed<ImageElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_image)
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

        None
    }
}

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
    /// Webサイトに用いられるベクターフォーマット。
    Svg,
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
    /// バイリニア補間などの平滑化アルゴリズムを用いて拡大縮小します。
    Smooth,
    /// 最近傍補間などのアルゴリズムで拡大縮小し、
    /// ピクセルで構成された画像の見た目を保ちます。
    Pixelated,
}
