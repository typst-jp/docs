use crate::foundations::{Content, Smart, elem};
use crate::introspection::{Locatable, Tagged};
use crate::layout::{Abs, Corners, Length, Rel, Sides};
use crate::text::{BottomEdge, BottomEdgeMetric, TopEdge, TopEdgeMetric};
use crate::visualize::{Color, FixedStroke, Paint, Stroke};
<<<<<<< HEAD
/// テキスト下部に線を追加。
///
/// # 例
=======

/// Underlines text.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// This is #underline[important].
/// ```
#[elem(Locatable, Tagged)]
pub struct UnderlineElem {
<<<<<<< HEAD
    /// 線の[stroke]をどうするか。
    ///
    /// `{auto}`に設定された場合、現在のテキストフォントで使用されているテキストの太さと色が使用されます。
=======
    /// How to [stroke] the line.
    ///
    /// If set to `{auto}`, takes on the text's color and a thickness defined in
    /// the current font.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// Take #underline(
    ///   stroke: 1.5pt + red,
    ///   offset: 2pt,
    ///   [care],
    /// )
    /// ```
    #[fold]
    pub stroke: Smart<Stroke>,
<<<<<<< HEAD
    /// ベースラインを基準とする線の位置。
    /// `{auto}`の場合、フォントテーブルから読まれます。
=======

    /// The position of the line relative to the baseline, read from the font
    /// tables if `{auto}`.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #underline(offset: 5pt)[
    ///   The Tale Of A Faraway Line I
    /// ]
    /// ```
    pub offset: Smart<Length>,
<<<<<<< HEAD
    /// コンテンツの外側に（負の値のときは内側に）線を左右に拡張する量。
=======

    /// The amount by which to extend the line beyond (or within if negative)
    /// the content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #align(center,
    ///   underline(extent: 2pt)[Chapter 1]
    /// )
    /// ```
    pub extent: Length,
<<<<<<< HEAD
    /// グリフと衝突する線の部分を省略するかどうか。
=======

    /// Whether the line skips sections in which it would collide with the
    /// glyphs.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// This #underline(evade: true)[is great].
    /// This #underline(evade: false)[is less great].
    /// ```
    #[default(true)]
    pub evade: bool,

<<<<<<< HEAD
    /// 線をコンテンツの背後に置くかどうか。
=======
    /// Whether the line is placed behind the content it underlines.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set underline(stroke: (thickness: 1em, paint: maroon, cap: "round"))
    /// #underline(background: true)[This is stylized.] \
    /// #underline(background: false)[This is partially hidden.]
    /// ```
    #[default(false)]
    pub background: bool,

<<<<<<< HEAD
    /// 下部に線を置くコンテンツ。
=======
    /// The content to underline.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// テキスト上部に線を追加。
///
/// # 例
=======
/// Adds a line over text.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// #overline[A line over text.]
/// ```
#[elem(Locatable, Tagged)]
pub struct OverlineElem {
<<<<<<< HEAD
    /// 線の[stroke]をどうするか。
    ///
    /// `{auto}`に設定された場合、現在のテキストフォントで使用されているテキストの太さと色が使用されます。
=======
    /// How to [stroke] the line.
    ///
    /// If set to `{auto}`, takes on the text's color and a thickness defined in
    /// the current font.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set text(fill: olive)
    /// #overline(
    ///   stroke: green.darken(20%),
    ///   offset: -12pt,
    ///   [The Forest Theme],
    /// )
    /// ```
    #[fold]
    pub stroke: Smart<Stroke>,
<<<<<<< HEAD
    /// ベースラインを基準とする線の位置。
    /// `{auto}`の場合、フォントテーブルから読まれます。
=======

    /// The position of the line relative to the baseline. Read from the font
    /// tables if `{auto}`.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #overline(offset: -1.2em)[
    ///   The Tale Of A Faraway Line II
    /// ]
    /// ```
    pub offset: Smart<Length>,
<<<<<<< HEAD
    /// コンテンツの外側に（負の値のときは内側に）線を左右に拡張する量。
=======

    /// The amount by which to extend the line beyond (or within if negative)
    /// the content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set overline(extent: 4pt)
    /// #set underline(extent: 4pt)
    /// #overline(underline[Typography Today])
    /// ```
    pub extent: Length,
<<<<<<< HEAD
    /// グリフと衝突する線の部分を省略するかどうか。
=======

    /// Whether the line skips sections in which it would collide with the
    /// glyphs.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #overline(
    ///   evade: false,
    ///   offset: -7.5pt,
    ///   stroke: 1pt,
    ///   extent: 3pt,
    ///   [Temple],
    /// )
    /// ```
    #[default(true)]
    pub evade: bool,

<<<<<<< HEAD
    /// 線をコンテンツの背後に置くかどうか。
=======
    /// Whether the line is placed behind the content it overlines.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set overline(stroke: (thickness: 1em, paint: maroon, cap: "round"))
    /// #overline(background: true)[This is stylized.] \
    /// #overline(background: false)[This is partially hidden.]
    /// ```
    #[default(false)]
    pub background: bool,

<<<<<<< HEAD
    /// 上部に線を置くコンテンツ。
=======
    /// The content to add a line over.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// テキストの打ち消し。
///
/// # 例
=======
/// Strikes through text.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// This is #strike[not] relevant.
/// ```
#[elem(title = "Strikethrough", Locatable, Tagged)]
pub struct StrikeElem {
<<<<<<< HEAD
    /// 線の[stroke]をどうするか。
    ///
    /// `{auto}`に設定された場合、現在のテキストフォントで使用されているテキストの太さと色が使用されます。
    ///
    /// _注意:_ テキストのコピー・ペーストは依然として可能なため、実際の黒塗りには使用しないでください。
=======
    /// How to [stroke] the line.
    ///
    /// If set to `{auto}`, takes on the text's color and a thickness defined in
    /// the current font.
    ///
    /// _Note:_ Please don't use this for real redaction as you can still copy
    /// paste the text.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// This is #strike(stroke: 1.5pt + red)[very stricken through]. \
    /// This is #strike(stroke: 10pt)[redacted].
    /// ```
    #[fold]
    pub stroke: Smart<Stroke>,
<<<<<<< HEAD
    /// ベースラインを基準とする線の位置。
    /// `{auto}`の場合、フォントテーブルから読まれます。
    ///
    /// これはフォントが提供するオフセットに不満がある場合に便利です。
=======

    /// The position of the line relative to the baseline. Read from the font
    /// tables if `{auto}`.
    ///
    /// This is useful if you are unhappy with the offset your font provides.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set text(font: "Inria Serif")
    /// This is #strike(offset: auto)[low-ish]. \
    /// This is #strike(offset: -3.5pt)[on-top].
    /// ```
    pub offset: Smart<Length>,
<<<<<<< HEAD
    /// コンテンツの外側に（負の値のときは内側に）線を左右に拡張する量。
=======

    /// The amount by which to extend the line beyond (or within if negative)
    /// the content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// This #strike(extent: -2pt)[skips] parts of the word.
    /// This #strike(extent: 2pt)[extends] beyond the word.
    /// ```
    pub extent: Length,
<<<<<<< HEAD
    /// 線をコンテンツの背後に置くかどうか。
=======

    /// Whether the line is placed behind the content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set strike(stroke: red)
    /// #strike(background: true)[This is behind.] \
    /// #strike(background: false)[This is in front.]
    /// ```
    #[default(false)]
    pub background: bool,

<<<<<<< HEAD
    /// 打ち消すコンテンツ。
=======
    /// The content to strike through.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// 背景色によるテキストハイライト。
///
/// # 例
=======
/// Highlights text with a background color.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// This is #highlight[important].
/// ```
#[elem(Locatable, Tagged)]
pub struct HighlightElem {
<<<<<<< HEAD
    /// テキストをハイライトする色。
=======
    /// The color to highlight the text with.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// This is #highlight(
    ///   fill: blue
    /// )[highlighted with blue].
    /// ```
    #[default(Some(Color::from_u8(0xFF, 0xFD, 0x11, 0xA1).into()))]
    pub fill: Option<Paint>,

<<<<<<< HEAD
    /// ハイライトの枠線の色。
    /// 詳細は[rectangleのドキュメント]($rect.stroke)を参照してください。
=======
    /// The highlight's border color. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// This is a #highlight(
    ///   stroke: fuchsia
    /// )[stroked highlighting].
    /// ```
    #[fold]
    pub stroke: Sides<Option<Option<Stroke>>>,
<<<<<<< HEAD
    /// 背景の長方形の上端。
=======

    /// The top end of the background rectangle.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set highlight(top-edge: "ascender")
    /// #highlight[a] #highlight[aib]
    ///
    /// #set highlight(top-edge: "x-height")
    /// #highlight[a] #highlight[aib]
    /// ```
    #[default(TopEdge::Metric(TopEdgeMetric::Ascender))]
    pub top_edge: TopEdge,

<<<<<<< HEAD
    /// 背景の長方形の下端。
=======
    /// The bottom end of the background rectangle.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set highlight(bottom-edge: "descender")
    /// #highlight[a] #highlight[ap]
    ///
    /// #set highlight(bottom-edge: "baseline")
    /// #highlight[a] #highlight[ap]
    /// ```
    #[default(BottomEdge::Metric(BottomEdgeMetric::Descender))]
    pub bottom_edge: BottomEdge,

<<<<<<< HEAD
    /// コンテンツの外側に（負の値のときは内側に）背景を左右に拡張する量。
=======
    /// The amount by which to extend the background to the sides beyond
    /// (or within if negative) the content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// A long #highlight(extent: 4pt)[background].
    /// ```
    pub extent: Length,
<<<<<<< HEAD
    /// 背景の角を丸める量。
    /// 詳細は[rectangleのドキュメント]($rect.radius)を参照してください。
=======

    /// How much to round the highlight's corners. See the
    /// [rectangle's documentation]($rect.radius) for more details.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// Listen #highlight(
    ///   radius: 5pt, extent: 2pt
    /// )[carefully], it will be on the test.
    /// ```
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,
<<<<<<< HEAD
    /// ハイライトされるべきコンテンツ。
=======

    /// The content that should be highlighted.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

/// A text decoration.
///
/// Can be positioned over, under, or on top of text, or highlight the text with
/// a background.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Decoration {
    pub line: DecoLine,
    pub extent: Abs,
}

/// A kind of decorative line.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum DecoLine {
    Underline {
        stroke: Stroke<Abs>,
        offset: Smart<Abs>,
        evade: bool,
        background: bool,
    },
    Strikethrough {
        stroke: Stroke<Abs>,
        offset: Smart<Abs>,
        background: bool,
    },
    Overline {
        stroke: Stroke<Abs>,
        offset: Smart<Abs>,
        evade: bool,
        background: bool,
    },
    Highlight {
        fill: Option<Paint>,
        stroke: Sides<Option<FixedStroke>>,
        top_edge: TopEdge,
        bottom_edge: BottomEdge,
        radius: Corners<Rel<Abs>>,
    },
}
