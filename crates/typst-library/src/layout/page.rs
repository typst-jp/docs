use std::borrow::Cow;
use std::num::NonZeroUsize;
use std::ops::RangeInclusive;
use std::str::FromStr;

use comemo::Track;
use typst_utils::{singleton, NonZeroExt, Scalar};

use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Args, AutoValue, Cast, Construct, Content, Context, Dict, Fold, Func,
    NativeElement, Set, Smart, StyleChain, Value,
};
use crate::introspection::Introspector;
use crate::layout::{
    Abs, Alignment, FlushElem, Frame, HAlignment, Length, OuterVAlignment, Ratio, Rel,
    Sides, SpecificAlignment,
};
use crate::model::{DocumentInfo, Numbering};
use crate::text::LocalName;
use crate::visualize::{Color, Paint};

/// その子要素を単一ページか複数ページにレイアウト。
///
/// この関数は主にsetルールでページのプロパティに影響を与えるために使用されますが、引数を独自のページセットに明示的にレンダリングするためにも使用できます。
///
/// ページでは幅と高さに`{auto}`が設定可能です。
/// この場合、ページは各軸方向においてコンテンツに合わせて大きくなります。
///
/// [ページセットアップガイド]($guides/page-setup-guide)では多くの例とともにこの関数と関連する関数を用いてどのように文書をセットアップするかを説明しています。
///
/// # 例
/// ```example
/// >>> #set page(margin: auto)
/// #set page("us-letter")
///
/// There you go, US friends!
/// ```
#[elem(Construct)]
pub struct PageElem {
    /// 幅と高さを設定するための標準的な紙の大きさ。
    ///
    /// これは`width`と`height`を設定するための単なる省略記法であり、その性質上コンテキスト式からは取得できません。
    #[external]
    #[default(Paper::A4)]
    pub paper: Paper,

    /// ページの幅。
    ///
    /// ```example
    /// #set page(
    ///   width: 3cm,
    ///   margin: (x: 0cm),
    /// )
    ///
    /// #for i in range(3) {
    ///   box(square(width: 1cm))
    /// }
    /// ```
    #[resolve]
    #[parse(
        let paper = args.named_or_find::<Paper>("paper")?;
        args.named("width")?
            .or_else(|| paper.map(|paper| Smart::Custom(paper.width().into())))
    )]
    #[default(Smart::Custom(Paper::A4.width().into()))]
    #[ghost]
    pub width: Smart<Length>,

    /// ページの高さ。
    ///
    /// これが`{auto}`に設定された場合、[改ページ]($pagebreak)は手動で挿入したときのみ発火させることができます。
    /// このドキュメントのほとんどの例では、ページの高さに `{auto}` を指定しており、コンテンツに合わせて動的に伸縮するようにしています。
    #[resolve]
    #[parse(
        args.named("height")?
            .or_else(|| paper.map(|paper| Smart::Custom(paper.height().into())))
    )]
    #[default(Smart::Custom(Paper::A4.height().into()))]
    #[ghost]
    pub height: Smart<Length>,

    /// ページを90度回転させるかどうか。
    ///
    /// ```example
    /// #set page(
    ///   "us-business-card",
    ///   flipped: true,
    ///   fill: rgb("f2e5dd"),
    /// )
    ///
    /// #set align(bottom + end)
    /// #text(14pt)[*Sam H. Richards*] \
    /// _Procurement Manager_
    ///
    /// #set text(10pt)
    /// 17 Main Street \
    /// New York, NY 10001 \
    /// +1 555 555 5555
    /// ```
    #[default(false)]
    #[ghost]
    pub flipped: bool,

    /// ページの余白。
    ///
    /// - `{auto}`: 余白が自動的に短辺の2.5/21倍の大きさに設定されます。
    /// A4のページでは2.5cmの余白になります。
    /// - 単一の長さ指定: 全ての辺に同じ大きさの余白を適用します。
    /// - 辞書指定: 辞書を用いることで余白を個々に設定可能です。
    ///  この辞書に含められるキーは以下のとおりです。リストは優先順位の順に並んでいます。
    ///   - `top`: 上部の余白。
    ///   - `right`: 右側の余白。
    ///   - `bottom`: 下部の余白。
    ///   - `left`: 左側の余白。
    ///   - `inside`: ページの内側の余白([綴じ]($page.binding)側).
    ///   - `outside`: ページの外側の余白([綴じ]($page.binding)の反対側).
    ///   - `x`: 水平方向の余白。
    ///   - `y`: 垂直方向の余白。
    ///   - `rest`: 明示的に指定されていない残り全ての余白。
    ///
    /// `left`と`right`の値は`inside`と`outside`の値と同時に指定できません。
    ///
    /// ```example
    /// #set page(
    ///  width: 3cm,
    ///  height: 4cm,
    ///  margin: (x: 8pt, y: 4pt),
    /// )
    ///
    /// #rect(
    ///   width: 100%,
    ///   height: 100%,
    ///   fill: aqua,
    /// )
    /// ```
    #[fold]
    #[ghost]
    pub margin: Margin,

    /// ページのどちら側を綴じるか。
    ///
    /// - `{auto}`: [テキストの向き]($text.dir)が左から右なら`left`と等価となり、右から左なら`right`と等価となります。
    /// - `left`: 左側を綴じます。
    /// - `right`: 右側を綴じます。
    ///
    /// これは余白の`inside`および`outside`オプションの意味に影響します。
    #[ghost]
    pub binding: Smart<Binding>,

    /// ぺージの段数。
    ///
    /// ページや他のコンテナに段を挿入したい場合は[`columns`関数]($columns)も使用できます。
    ///
    /// ```example:single
    /// #set page(columns: 2, height: 4.8cm)
    /// Climate change is one of the most
    /// pressing issues of our time, with
    /// the potential to devastate
    /// communities, ecosystems, and
    /// economies around the world. It's
    /// clear that we need to take urgent
    /// action to reduce our carbon
    /// emissions and mitigate the impacts
    /// of a rapidly changing climate.
    /// ```
    #[default(NonZeroUsize::ONE)]
    #[ghost]
    pub columns: NonZeroUsize,

    /// ページ背景の塗り潰し。
    ///
    /// これを非透明なものに設定すると、プリンターにページ全体を着色するよう指示します。
    /// 印刷規模が大きい場合は、このプロパティを設定せず、あらかじめ染色された用紙を調達したほうが環境にも優しくコスト削減もできるでしょう。
    ///
    /// `{none}`に設定すると背景は透明になります。
    /// ビューアーではPDFのページに（通常は白色の）背景があるように見えますが、実際は透明であることに注意してください（印刷時には背景に色が使用されません）。
    ///
    /// デフォルト値の`{auto}`は、PDF出力の場合は`{none}`となり、PNGおよびSVG出力の場合は`{white}`になります。
    ///
    /// ```example
    /// #set page(fill: rgb("444352"))
    /// #set text(fill: rgb("fdfdfd"))
    /// *Dark mode enabled.*
    /// ```
    #[borrowed]
    #[ghost]
    pub fill: Smart<Option<Paint>>,

    /// ページ[番号]($numbering)の付け方。
    ///
    /// `footer`（もしくは上部配置番号付けの場合は`header`）が明示的に与えられた場合、numberingは無視されます。
    ///
    /// ```example
    /// #set page(
    ///   height: 100pt,
    ///   margin: (top: 16pt, bottom: 24pt),
    ///   numbering: "1 / 1",
    /// )
    ///
    /// #lorem(48)
    /// ```
    #[borrowed]
    #[ghost]
    pub numbering: Option<Numbering>,

    /// ページの補足語。
    ///
    /// ページ参照では、これはページ番号の前に追加されます。
    ///
    /// ```example
    /// #set page(numbering: "1.", supplement: [p.])
    ///
    /// = Introduction <intro>
    /// We are on #ref(<intro>, form: "page")!
    /// ```
    #[ghost]
    pub supplement: Smart<Option<Content>>,

    /// ページ番号の配置。
    ///
    /// 垂直成分が`top`の場合は、番号はヘッダー内に配置され、`bottom`の場合はフッター内に配置されます。
    /// 水平方向には配置できません。
    /// 対応する`header`や`footer`が明示的に与えられた場合、numberingは無視されます。
    ///
    /// ```example
    /// #set page(
    ///   margin: (top: 16pt, bottom: 24pt),
    ///   numbering: "1",
    ///   number-align: right,
    /// )
    ///
    /// #lorem(30)
    /// ```
    #[default(SpecificAlignment::Both(HAlignment::Center, OuterVAlignment::Bottom))]
    #[ghost]
    pub number_align: SpecificAlignment<HAlignment, OuterVAlignment>,

    /// ページのヘッダー。
    /// ページの上部余白を埋めます。
    ///
    /// - コンテンツが与えられた場合: ヘッダーとしてコンテンツを表示します。
    /// - `{auto}`: `numbering`が設定されており、`number-align`が`top`の場合ページ番号を表示します。
    /// - `{none}`: ヘッダーを表示しません。
    ///
    /// ```example
    /// #set par(justify: true)
    /// #set page(
    ///   margin: (top: 32pt, bottom: 20pt),
    ///   header: [
    ///     #set text(8pt)
    ///     #smallcaps[Typst Academy]
    ///     #h(1fr) _Exercise Sheet 3_
    ///   ],
    /// )
    ///
    /// #lorem(19)
    /// ```
    #[borrowed]
    #[ghost]
    pub header: Smart<Option<Content>>,

    /// 上部余白方向のヘッダーの上昇量。
    #[resolve]
    #[default(Ratio::new(0.3).into())]
    #[ghost]
    pub header_ascent: Rel<Length>,

    /// ページのフッター。
    /// ページの下部余白を埋めます。
    ///
    /// - コンテンツが与えられた場合: フッターとしてコンテンツを表示します。
    /// - `{auto}`: `numbering`が設定されており、`number-align`が`bottom`の場合ページ番号を表示します。
    /// - `{none}`: フッターを表示しません。
    ///
    /// 単純なページ番号を使用する場合は、一般的に`numbering`プロパティで十分です。
    /// カスタムフッターを作成したい場合でページ番号を表示したいときは、[ページカウンター]($counter)に直接アクセスできます。
    ///
    /// ```example
    /// #set par(justify: true)
    /// #set page(
    ///   height: 100pt,
    ///   margin: 20pt,
    ///   footer: context [
    ///     #set align(right)
    ///     #set text(8pt)
    ///     #counter(page).display(
    ///       "1 of I",
    ///       both: true,
    ///     )
    ///   ]
    /// )
    ///
    /// #lorem(48)
    /// ```
    #[borrowed]
    #[ghost]
    pub footer: Smart<Option<Content>>,

    /// 下部余白方向のフッターの下降量。
    #[resolve]
    #[default(Ratio::new(0.3).into())]
    #[ghost]
    pub footer_descent: Rel<Length>,

    /// ページ背景のコンテンツ。
    ///
    /// このコンテンツはページ本文の背後に配置されます。
    /// これは背景画像や透かしに使用できます。
    ///
    /// ```example
    /// #set page(background: rotate(24deg,
    ///   text(18pt, fill: rgb("FFCBC4"))[
    ///     *CONFIDENTIAL*
    ///   ]
    /// ))
    ///
    /// = Typst's secret plans
    /// In the year 2023, we plan to take
    /// over the world (of typesetting).
    /// ```
    #[borrowed]
    #[ghost]
    pub background: Option<Content>,

    /// ページ前景のコンテンツ。
    ///
    /// このコンテンツはページ本文の上に重なって表示されます。
    ///
    /// ```example
    /// #set page(foreground: text(24pt)[🥸])
    ///
    /// Reviewer 2 has marked our paper
    /// "Weak Reject" because they did
    /// not understand our approach...
    /// ```
    #[borrowed]
    #[ghost]
    pub foreground: Option<Content>,

    /// ページの本文。
    ///
    /// コンテンツが単一ページに収まらなかった場合は複数ページが作成されます。
    /// 関数呼び出し前に設定されていたページプロパティを適用した新しいページが本文組版後に作成されます。
    #[external]
    #[required]
    pub body: Content,
}

impl Construct for PageElem {
    fn construct(engine: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        // The page constructor is special: It doesn't create a page element.
        // Instead, it just ensures that the passed content lives in a separate
        // page and styles it.
        let styles = Self::set(engine, args)?;
        let body = args.expect::<Content>("body")?;
        Ok(Content::sequence([
            PagebreakElem::shared_weak().clone(),
            // We put an effectless, invisible non-tag element on the page.
            // This has two desirable consequences:
            // - The page is kept even if the body is empty
            // - The page doesn't inherit shared styles from the body
            FlushElem::new().pack(),
            body,
            PagebreakElem::shared_boundary().clone(),
        ])
        .styled_with_map(styles))
    }
}

impl LocalName for PageElem {
    const KEY: &'static str = "page";
}

/// 手動の改ページ。
///
/// いかなるコンテナ内でも使用してはいけません。
///
/// # 例
/// ```example
/// The next page contains
/// more details on compound theory.
/// #pagebreak()
///
/// == Compound Theory
/// In 1984, the first ...
/// ```
#[elem(title = "Page Break")]
pub struct PagebreakElem {
    /// `{true}`の場合、現在のページが既に空のとき改ページは行われません。
    #[default(false)]
    pub weak: bool,

    /// 設定された場合、必要に応じて空ページを追加して次のページが偶数/奇数ページになるようにします。
    ///
    /// ```example
    /// #set page(height: 30pt)
    ///
    /// First.
    /// #pagebreak(to: "odd")
    /// Third.
    /// ```
    pub to: Option<Parity>,

    /// Whether this pagebreak designates an end boundary of a page run. This is
    /// an even weaker version of pagebreak `weak` because it not only doesn't
    /// force an empty page, but also doesn't force its initial styles onto a
    /// staged empty page.
    #[internal]
    #[parse(None)]
    #[default(false)]
    pub boundary: bool,
}

impl PagebreakElem {
    /// Get the globally shared weak pagebreak element.
    pub fn shared_weak() -> &'static Content {
        singleton!(Content, PagebreakElem::new().with_weak(true).pack())
    }

    /// Get the globally shared boundary pagebreak element.
    pub fn shared_boundary() -> &'static Content {
        singleton!(
            Content,
            PagebreakElem::new().with_weak(true).with_boundary(true).pack()
        )
    }
}

/// A finished document with metadata and page frames.
#[derive(Debug, Default, Clone)]
pub struct PagedDocument {
    /// The document's finished pages.
    pub pages: Vec<Page>,
    /// Details about the document.
    pub info: DocumentInfo,
    /// Provides the ability to execute queries on the document.
    pub introspector: Introspector,
}

/// A finished page.
#[derive(Debug, Clone)]
pub struct Page {
    /// The frame that defines the page.
    pub frame: Frame,
    /// How the page is filled.
    ///
    /// - When `None`, the background is transparent.
    /// - When `Auto`, the background is transparent for PDF and white
    ///   for raster and SVG targets.
    ///
    /// Exporters should access the resolved value of this property through
    /// `fill_or_transparent()` or `fill_or_white()`.
    pub fill: Smart<Option<Paint>>,
    /// The page's numbering.
    pub numbering: Option<Numbering>,
    /// The page's supplement.
    pub supplement: Content,
    /// The logical page number (controlled by `counter(page)` and may thus not
    /// match the physical number).
    pub number: usize,
}

impl Page {
    /// Get the configured background or `None` if it is `Auto`.
    ///
    /// This is used in PDF export.
    pub fn fill_or_transparent(&self) -> Option<Paint> {
        self.fill.clone().unwrap_or(None)
    }

    /// Get the configured background or white if it is `Auto`.
    ///
    /// This is used in raster and SVG export.
    pub fn fill_or_white(&self) -> Option<Paint> {
        self.fill.clone().unwrap_or_else(|| Some(Color::WHITE.into()))
    }
}

/// Specification of the page's margins.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Margin {
    /// The margins for each side.
    pub sides: Sides<Option<Smart<Rel<Length>>>>,
    /// Whether to swap `left` and `right` to make them `inside` and `outside`
    /// (when to swap depends on the binding).
    pub two_sided: Option<bool>,
}

impl Margin {
    /// Create an instance with four equal components.
    pub fn splat(value: Option<Smart<Rel<Length>>>) -> Self {
        Self { sides: Sides::splat(value), two_sided: None }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            sides: Sides::splat(Some(Smart::Auto)),
            two_sided: None,
        }
    }
}

impl Fold for Margin {
    fn fold(self, outer: Self) -> Self {
        Margin {
            sides: self.sides.fold(outer.sides),
            two_sided: self.two_sided.fold(outer.two_sided),
        }
    }
}

cast! {
    Margin,
    self => {
        let two_sided = self.two_sided.unwrap_or(false);
        if !two_sided && self.sides.is_uniform() {
            if let Some(left) = self.sides.left {
                return left.into_value();
            }
        }

        let mut dict = Dict::new();
        let mut handle = |key: &str, component: Option<Smart<Rel<Length>>>| {
            if let Some(c) = component {
                dict.insert(key.into(), c.into_value());
            }
        };

        handle("top", self.sides.top);
        handle("bottom", self.sides.bottom);
        if two_sided {
            handle("inside", self.sides.left);
            handle("outside", self.sides.right);
        } else {
            handle("left", self.sides.left);
            handle("right", self.sides.right);
        }

        Value::Dict(dict)
    },
    _: AutoValue => Self::splat(Some(Smart::Auto)),
    v: Rel<Length> => Self::splat(Some(Smart::Custom(v))),
    mut dict: Dict => {
        let mut take = |key| dict.take(key).ok().map(Value::cast).transpose();

        let rest = take("rest")?;
        let x = take("x")?.or(rest);
        let y = take("y")?.or(rest);
        let top = take("top")?.or(y);
        let bottom = take("bottom")?.or(y);
        let outside = take("outside")?;
        let inside = take("inside")?;
        let left = take("left")?;
        let right = take("right")?;

        let implicitly_two_sided = outside.is_some() || inside.is_some();
        let implicitly_not_two_sided = left.is_some() || right.is_some();
        if implicitly_two_sided && implicitly_not_two_sided {
            bail!("`inside` and `outside` are mutually exclusive with `left` and `right`");
        }

        // - If 'implicitly_two_sided' is false here, then
        //   'implicitly_not_two_sided' will be guaranteed to be true
        //    due to the previous two 'if' conditions.
        // - If both are false, this means that this margin change does not
        //   affect lateral margins, and thus shouldn't make a difference on
        //   the 'two_sided' attribute of this margin.
        let two_sided = (implicitly_two_sided || implicitly_not_two_sided)
            .then_some(implicitly_two_sided);

        dict.finish(&[
            "left", "top", "right", "bottom", "outside", "inside", "x", "y", "rest",
        ])?;

        Margin {
            sides: Sides {
                left: inside.or(left).or(x),
                top,
                right: outside.or(right).or(x),
                bottom,
            },
            two_sided,
        }
    }
}

/// Specification of the page's binding.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Binding {
    /// Bound on the left, as customary in LTR languages.
    Left,
    /// Bound on the right, as customary in RTL languages.
    Right,
}

impl Binding {
    /// Whether to swap left and right margin for the page with this number.
    pub fn swap(self, number: NonZeroUsize) -> bool {
        match self {
            // Left-bound must swap on even pages
            // (because it is correct on the first page).
            Self::Left => number.get() % 2 == 0,
            // Right-bound must swap on odd pages
            // (because it is wrong on the first page).
            Self::Right => number.get() % 2 == 1,
        }
    }
}

cast! {
    Binding,
    self => match self {
        Self::Left => Alignment::LEFT.into_value(),
        Self::Right => Alignment::RIGHT.into_value(),
    },
    v: Alignment => match v {
        Alignment::LEFT => Self::Left,
        Alignment::RIGHT => Self::Right,
        _ => bail!("must be `left` or `right`"),
    },
}

/// A header, footer, foreground or background definition.
#[derive(Debug, Clone, Hash)]
pub enum Marginal {
    /// Bare content.
    Content(Content),
    /// A closure mapping from a page number to content.
    Func(Func),
}

impl Marginal {
    /// Resolve the marginal based on the page number.
    pub fn resolve(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        page: usize,
    ) -> SourceResult<Cow<'_, Content>> {
        Ok(match self {
            Self::Content(content) => Cow::Borrowed(content),
            Self::Func(func) => Cow::Owned(
                func.call(engine, Context::new(None, Some(styles)).track(), [page])?
                    .display(),
            ),
        })
    }
}

cast! {
    Marginal,
    self => match self {
        Self::Content(v) => v.into_value(),
        Self::Func(v) => v.into_value(),
    },
    v: Content => Self::Content(v),
    v: Func => Self::Func(v),
}

/// A list of page ranges to be exported.
#[derive(Debug, Clone)]
pub struct PageRanges(Vec<PageRange>);

/// A range of pages to export.
///
/// The range is one-indexed. For example, `1..=3` indicates the first, second
/// and third pages should be exported.
pub type PageRange = RangeInclusive<Option<NonZeroUsize>>;

impl PageRanges {
    /// Create new page ranges.
    pub fn new(ranges: Vec<PageRange>) -> Self {
        Self(ranges)
    }

    /// Check if a page, given its number, should be included when exporting the
    /// document while restricting the exported pages to these page ranges.
    /// This is the one-indexed version of 'includes_page_index'.
    pub fn includes_page(&self, page: NonZeroUsize) -> bool {
        self.includes_page_index(page.get() - 1)
    }

    /// Check if a page, given its index, should be included when exporting the
    /// document while restricting the exported pages to these page ranges.
    /// This is the zero-indexed version of 'includes_page'.
    pub fn includes_page_index(&self, page: usize) -> bool {
        let page = NonZeroUsize::try_from(page + 1).unwrap();
        self.0.iter().any(|range| match (range.start(), range.end()) {
            (Some(start), Some(end)) => (start..=end).contains(&&page),
            (Some(start), None) => (start..).contains(&&page),
            (None, Some(end)) => (..=end).contains(&&page),
            (None, None) => true,
        })
    }
}

/// Whether something should be even or odd.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Parity {
    /// 次のページが偶数ページになります。
    Even,
    /// 次のページが奇数ページになります。
    Odd,
}

impl Parity {
    /// Whether the given number matches the parity.
    pub fn matches(self, number: usize) -> bool {
        match self {
            Self::Even => number % 2 == 0,
            Self::Odd => number % 2 == 1,
        }
    }
}

/// Specification of a paper.
#[derive(Debug, Copy, Clone, Hash)]
pub struct Paper {
    /// The name of the paper.
    name: &'static str,
    /// The width of the paper in millimeters.
    width: Scalar,
    /// The height of the paper in millimeters.
    height: Scalar,
}

impl Paper {
    /// The width of the paper.
    pub fn width(self) -> Abs {
        Abs::mm(self.width.get())
    }

    /// The height of the paper.
    pub fn height(self) -> Abs {
        Abs::mm(self.height.get())
    }
}

/// Defines paper constants and a paper parsing implementation.
macro_rules! papers {
    ($(($var:ident: $width:expr, $height: expr, $name:literal))*) => {
        /// Predefined papers.
        ///
        /// Each paper is parsable from its name in kebab-case.
        impl Paper {
            $(pub const $var: Self = Self {
                name: $name,
                width: Scalar::new($width),
                height: Scalar::new($height),
            };)*
        }

        impl FromStr for Paper {
            type Err = &'static str;

            fn from_str(name: &str) -> Result<Self, Self::Err> {
                match name.to_lowercase().as_str() {
                    $($name => Ok(Self::$var),)*
                    _ => Err("unknown paper size"),
                }
            }
        }

        cast! {
            Paper,
            self => self.name.into_value(),
            $(
                /// 指定された大きさの用紙を生成。
                $name => Self::$var,
            )*
        }
    };
}

// All paper sizes in mm.
//
// Resources:
// - https://papersizes.io/
// - https://en.wikipedia.org/wiki/Paper_size
// - https://www.theedkins.co.uk/jo/units/oldunits/print.htm
// - https://vintagepaper.co/blogs/news/traditional-paper-sizes
papers! {
    // ---------------------------------------------------------------------- //
    // ISO 216 A Series
    (A0:  841.0, 1189.0, "a0")
    (A1:  594.0,  841.0, "a1")
    (A2:  420.0,  594.0, "a2")
    (A3:  297.0,  420.0, "a3")
    (A4:  210.0,  297.0, "a4")
    (A5:  148.0,  210.0, "a5")
    (A6:  105.0,  148.0, "a6")
    (A7:   74.0,  105.0, "a7")
    (A8:   52.0,   74.0, "a8")
    (A9:   37.0,   52.0, "a9")
    (A10:  26.0,   37.0, "a10")
    (A11:  18.0,   26.0, "a11")

    // ISO 216 B Series
    (ISO_B1: 707.0, 1000.0, "iso-b1")
    (ISO_B2: 500.0,  707.0,  "iso-b2")
    (ISO_B3: 353.0,  500.0,  "iso-b3")
    (ISO_B4: 250.0,  353.0,  "iso-b4")
    (ISO_B5: 176.0,  250.0,  "iso-b5")
    (ISO_B6: 125.0,  176.0,  "iso-b6")
    (ISO_B7:  88.0,  125.0,  "iso-b7")
    (ISO_B8:  62.0,   88.0,  "iso-b8")

    // ISO 216 C Series
    (ISO_C3: 324.0, 458.0, "iso-c3")
    (ISO_C4: 229.0, 324.0, "iso-c4")
    (ISO_C5: 162.0, 229.0, "iso-c5")
    (ISO_C6: 114.0, 162.0, "iso-c6")
    (ISO_C7:  81.0, 114.0, "iso-c7")
    (ISO_C8:  57.0,  81.0, "iso-c8")

    // DIN D Series (extension to ISO)
    (DIN_D3: 272.0, 385.0, "din-d3")
    (DIN_D4: 192.0, 272.0, "din-d4")
    (DIN_D5: 136.0, 192.0, "din-d5")
    (DIN_D6:  96.0, 136.0, "din-d6")
    (DIN_D7:  68.0,  96.0, "din-d7")
    (DIN_D8:  48.0,  68.0, "din-d8")

    // SIS (used in academia)
    (SIS_G5: 169.0, 239.0, "sis-g5")
    (SIS_E5: 115.0, 220.0, "sis-e5")

    // ANSI Extensions
    (ANSI_A: 216.0,  279.0, "ansi-a")
    (ANSI_B: 279.0,  432.0, "ansi-b")
    (ANSI_C: 432.0,  559.0, "ansi-c")
    (ANSI_D: 559.0,  864.0, "ansi-d")
    (ANSI_E: 864.0, 1118.0, "ansi-e")

    // ANSI Architectural Paper
    (ARCH_A:  229.0,  305.0, "arch-a")
    (ARCH_B:  305.0,  457.0, "arch-b")
    (ARCH_C:  457.0,  610.0, "arch-c")
    (ARCH_D:  610.0,  914.0, "arch-d")
    (ARCH_E1: 762.0, 1067.0, "arch-e1")
    (ARCH_E:  914.0, 1219.0, "arch-e")

    // JIS B Series
    (JIS_B0:  1030.0, 1456.0, "jis-b0")
    (JIS_B1:   728.0, 1030.0, "jis-b1")
    (JIS_B2:   515.0,  728.0, "jis-b2")
    (JIS_B3:   364.0,  515.0, "jis-b3")
    (JIS_B4:   257.0,  364.0, "jis-b4")
    (JIS_B5:   182.0,  257.0, "jis-b5")
    (JIS_B6:   128.0,  182.0, "jis-b6")
    (JIS_B7:    91.0,  128.0, "jis-b7")
    (JIS_B8:    64.0,   91.0, "jis-b8")
    (JIS_B9:    45.0,   64.0, "jis-b9")
    (JIS_B10:   32.0,   45.0, "jis-b10")
    (JIS_B11:   22.0,   32.0, "jis-b11")

    // SAC D Series
    (SAC_D0: 764.0, 1064.0, "sac-d0")
    (SAC_D1: 532.0,  760.0, "sac-d1")
    (SAC_D2: 380.0,  528.0, "sac-d2")
    (SAC_D3: 264.0,  376.0, "sac-d3")
    (SAC_D4: 188.0,  260.0, "sac-d4")
    (SAC_D5: 130.0,  184.0, "sac-d5")
    (SAC_D6:  92.0,  126.0, "sac-d6")

    // ISO 7810 ID
    (ISO_ID_1: 85.6, 53.98, "iso-id-1")
    (ISO_ID_2: 74.0, 105.0, "iso-id-2")
    (ISO_ID_3: 88.0, 125.0, "iso-id-3")

    // ---------------------------------------------------------------------- //
    // Asia
    (ASIA_F4: 210.0, 330.0, "asia-f4")

    // Japan
    (JP_SHIROKU_BAN_4: 264.0, 379.0, "jp-shiroku-ban-4")
    (JP_SHIROKU_BAN_5: 189.0, 262.0, "jp-shiroku-ban-5")
    (JP_SHIROKU_BAN_6: 127.0, 188.0, "jp-shiroku-ban-6")
    (JP_KIKU_4:        227.0, 306.0, "jp-kiku-4")
    (JP_KIKU_5:        151.0, 227.0, "jp-kiku-5")
    (JP_BUSINESS_CARD:  91.0,  55.0, "jp-business-card")

    // China
    (CN_BUSINESS_CARD: 90.0, 54.0, "cn-business-card")

    // Europe
    (EU_BUSINESS_CARD: 85.0, 55.0, "eu-business-card")

    // French Traditional (AFNOR)
    (FR_TELLIERE:          340.0, 440.0, "fr-tellière")
    (FR_COURONNE_ECRITURE: 360.0, 460.0, "fr-couronne-écriture")
    (FR_COURONNE_EDITION:  370.0, 470.0, "fr-couronne-édition")
    (FR_RAISIN:            500.0, 650.0, "fr-raisin")
    (FR_CARRE:             450.0, 560.0, "fr-carré")
    (FR_JESUS:             560.0, 760.0, "fr-jésus")

    // United Kingdom Imperial
    (UK_BRIEF:    406.4, 342.9, "uk-brief")
    (UK_DRAFT:    254.0, 406.4, "uk-draft")
    (UK_FOOLSCAP: 203.2, 330.2, "uk-foolscap")
    (UK_QUARTO:   203.2, 254.0, "uk-quarto")
    (UK_CROWN:    508.0, 381.0, "uk-crown")
    (UK_BOOK_A:   111.0, 178.0, "uk-book-a")
    (UK_BOOK_B:   129.0, 198.0, "uk-book-b")

    // Unites States
    (US_LETTER:         215.9,  279.4, "us-letter")
    (US_LEGAL:          215.9,  355.6, "us-legal")
    (US_TABLOID:        279.4,  431.8, "us-tabloid")
    (US_EXECUTIVE:      84.15,  266.7, "us-executive")
    (US_FOOLSCAP_FOLIO: 215.9,  342.9, "us-foolscap-folio")
    (US_STATEMENT:      139.7,  215.9, "us-statement")
    (US_LEDGER:         431.8,  279.4, "us-ledger")
    (US_OFICIO:         215.9, 340.36, "us-oficio")
    (US_GOV_LETTER:     203.2,  266.7, "us-gov-letter")
    (US_GOV_LEGAL:      215.9,  330.2, "us-gov-legal")
    (US_BUSINESS_CARD:   88.9,   50.8, "us-business-card")
    (US_DIGEST:         139.7,  215.9, "us-digest")
    (US_TRADE:          152.4,  228.6, "us-trade")

    // ---------------------------------------------------------------------- //
    // Other
    (NEWSPAPER_COMPACT:    280.0,    430.0, "newspaper-compact")
    (NEWSPAPER_BERLINER:   315.0,    470.0, "newspaper-berliner")
    (NEWSPAPER_BROADSHEET: 381.0,    578.0, "newspaper-broadsheet")
    (PRESENTATION_16_9:    297.0, 167.0625, "presentation-16-9")
    (PRESENTATION_4_3:     280.0,    210.0, "presentation-4-3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paged_document_is_send_and_sync() {
        fn ensure_send_and_sync<T: Send + Sync>() {}
        ensure_send_and_sync::<PagedDocument>();
    }
}
