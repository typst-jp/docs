use smallvec::smallvec;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, Packed, Show, Smart, StyleChain};
use crate::layout::{Abs, Corners, Length, Rel, Sides};
use crate::text::{BottomEdge, BottomEdgeMetric, TextElem, TopEdge, TopEdgeMetric};
use crate::visualize::{Color, FixedStroke, Paint, Stroke};

/// テキスト下部に線を追加。
///
/// # 例
/// ```example
/// This is #underline[important].
/// ```
#[elem(Show)]
pub struct UnderlineElem {
    /// 線の[stroke]をどうするか。
    ///
    /// `{auto}`に設定された場合、現在のテキストフォントで使用されているテキストの太さと色が使用されます。
    ///
    /// ```example
    /// Take #underline(
    ///   stroke: 1.5pt + red,
    ///   offset: 2pt,
    ///   [care],
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<Stroke>,

    /// ベースラインを基準とする線の位置。
    /// `{auto}`の場合、フォントテーブルから読まれます。
    ///
    /// ```example
    /// #underline(offset: 5pt)[
    ///   The Tale Of A Faraway Line I
    /// ]
    /// ```
    #[resolve]
    pub offset: Smart<Length>,

    /// コンテンツの外側に（負の値のときは内側に）線を左右に拡張する量。
    ///
    /// ```example
    /// #align(center,
    ///   underline(extent: 2pt)[Chapter 1]
    /// )
    /// ```
    #[resolve]
    pub extent: Length,

    /// 字形と衝突する線の部分を省略するかどうか。
    ///
    /// ```example
    /// This #underline(evade: true)[is great].
    /// This #underline(evade: false)[is less great].
    /// ```
    #[default(true)]
    pub evade: bool,

    /// 線をコンテンツの背後に置くかどうか。
    ///
    /// ```example
    /// #set underline(stroke: (thickness: 1em, paint: maroon, cap: "round"))
    /// #underline(background: true)[This is stylized.] \
    /// #underline(background: false)[This is partially hidden.]
    /// ```
    #[default(false)]
    pub background: bool,

    /// 下部に線を置くコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<UnderlineElem> {
    #[typst_macros::time(name = "underline", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone().styled(TextElem::set_deco(smallvec![Decoration {
            line: DecoLine::Underline {
                stroke: self.stroke(styles).unwrap_or_default(),
                offset: self.offset(styles),
                evade: self.evade(styles),
                background: self.background(styles),
            },
            extent: self.extent(styles),
        }])))
    }
}

/// テキスト上部に線を追加。
///
/// # 例
/// ```example
/// #overline[A line over text.]
/// ```
#[elem(Show)]
pub struct OverlineElem {
    /// 線の[stroke]をどうするか。
    ///
    /// `{auto}`に設定された場合、現在のテキストフォントで使用されているテキストの太さと色が使用されます。
    ///
    /// ```example
    /// #set text(fill: olive)
    /// #overline(
    ///   stroke: green.darken(20%),
    ///   offset: -12pt,
    ///   [The Forest Theme],
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<Stroke>,

    /// ベースラインを基準とする線の位置。
    /// `{auto}`の場合、フォントテーブルから読まれます。
    ///
    /// ```example
    /// #overline(offset: -1.2em)[
    ///   The Tale Of A Faraway Line II
    /// ]
    /// ```
    #[resolve]
    pub offset: Smart<Length>,

    /// コンテンツの外側に（負の値のときは内側に）線を左右に拡張する量。
    ///
    /// ```example
    /// #set overline(extent: 4pt)
    /// #set underline(extent: 4pt)
    /// #overline(underline[Typography Today])
    /// ```
    #[resolve]
    pub extent: Length,

    /// 字形と衝突する線の部分を省略するかどうか。
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

    /// 線をコンテンツの背後に置くかどうか。
    ///
    /// ```example
    /// #set overline(stroke: (thickness: 1em, paint: maroon, cap: "round"))
    /// #overline(background: true)[This is stylized.] \
    /// #overline(background: false)[This is partially hidden.]
    /// ```
    #[default(false)]
    pub background: bool,

    /// 上部に線を置くコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<OverlineElem> {
    #[typst_macros::time(name = "overline", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone().styled(TextElem::set_deco(smallvec![Decoration {
            line: DecoLine::Overline {
                stroke: self.stroke(styles).unwrap_or_default(),
                offset: self.offset(styles),
                evade: self.evade(styles),
                background: self.background(styles),
            },
            extent: self.extent(styles),
        }])))
    }
}

/// テキストの打ち消し。
///
/// # 例
/// ```example
/// This is #strike[not] relevant.
/// ```
#[elem(title = "Strikethrough", Show)]
pub struct StrikeElem {
    /// 線の[stroke]をどうするか。
    ///
    /// `{auto}`に設定された場合、現在のテキストフォントで使用されているテキストの太さと色が使用されます。
    ///
    /// _注意:_ テキストのコピー・ペーストは依然として可能なため、実際の黒塗りには使用しないでください。
    ///
    /// ```example
    /// This is #strike(stroke: 1.5pt + red)[very stricken through]. \
    /// This is #strike(stroke: 10pt)[redacted].
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<Stroke>,

    /// ベースラインを基準とする線の位置。
    /// `{auto}`の場合、フォントテーブルから読まれます。
    ///
    /// これはフォントが提供するオフセットに不満がある場合に便利です。
    ///
    /// ```example
    /// #set text(font: "Inria Serif")
    /// This is #strike(offset: auto)[low-ish]. \
    /// This is #strike(offset: -3.5pt)[on-top].
    /// ```
    #[resolve]
    pub offset: Smart<Length>,

    /// コンテンツの外側に（負の値のときは内側に）線を左右に拡張する量。
    ///
    /// ```example
    /// This #strike(extent: -2pt)[skips] parts of the word.
    /// This #strike(extent: 2pt)[extends] beyond the word.
    /// ```
    #[resolve]
    pub extent: Length,

    /// 線をコンテンツの背後に置くかどうか。
    ///
    /// ```example
    /// #set strike(stroke: red)
    /// #strike(background: true)[This is behind.] \
    /// #strike(background: false)[This is in front.]
    /// ```
    #[default(false)]
    pub background: bool,

    /// 打ち消すコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<StrikeElem> {
    #[typst_macros::time(name = "strike", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone().styled(TextElem::set_deco(smallvec![Decoration {
            // Note that we do not support evade option for strikethrough.
            line: DecoLine::Strikethrough {
                stroke: self.stroke(styles).unwrap_or_default(),
                offset: self.offset(styles),
                background: self.background(styles),
            },
            extent: self.extent(styles),
        }])))
    }
}

/// 背景色によるテキストハイライト。
///
/// # 例
/// ```example
/// This is #highlight[important].
/// ```
#[elem(Show)]
pub struct HighlightElem {
    /// テキストをハイライトする色。
    ///
    /// ```example
    /// This is #highlight(
    ///   fill: blue
    /// )[highlighted with blue].
    /// ```
    #[default(Some(Color::from_u8(0xFF, 0xFD, 0x11, 0xA1).into()))]
    pub fill: Option<Paint>,

    /// ハイライトの枠線の色。
    /// 詳細は[rectangleのドキュメント]($rect.stroke)を参照してください。
    ///
    /// ```example
    /// This is a #highlight(
    ///   stroke: fuchsia
    /// )[stroked highlighting].
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Sides<Option<Option<Stroke>>>,

    /// 背景の長方形の上端。
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

    /// 背景の長方形の下端。
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

    /// コンテンツの外側に（負の値のときは内側に）背景を左右に拡張する量。
    ///
    /// ```example
    /// A long #highlight(extent: 4pt)[background].
    /// ```
    #[resolve]
    pub extent: Length,

    /// 背景の角を丸める量。
    /// 詳細は[rectangleのドキュメント]($rect.radius)を参照してください。
    ///
    /// ```example
    /// Listen #highlight(
    ///   radius: 5pt, extent: 2pt
    /// )[carefully], it will be on the test.
    /// ```
    #[resolve]
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,

    /// ハイライトされるべきコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<HighlightElem> {
    #[typst_macros::time(name = "highlight", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone().styled(TextElem::set_deco(smallvec![Decoration {
            line: DecoLine::Highlight {
                fill: self.fill(styles),
                stroke: self
                    .stroke(styles)
                    .unwrap_or_default()
                    .map(|stroke| stroke.map(Stroke::unwrap_or_default)),
                top_edge: self.top_edge(styles),
                bottom_edge: self.bottom_edge(styles),
                radius: self.radius(styles).unwrap_or_default(),
            },
            extent: self.extent(styles),
        }])))
    }
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
