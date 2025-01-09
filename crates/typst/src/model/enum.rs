use std::str::FromStr;

use comemo::Track;
use smallvec::{smallvec, SmallVec};

use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Array, Content, Context, NativeElement, Packed, Show, Smart,
    StyleChain, Styles,
};
use crate::introspection::Locator;
use crate::layout::{
    Alignment, Axes, BlockElem, Cell, CellGrid, Em, Fragment, GridLayouter, HAlignment,
    Length, Regions, Sizing, VAlignment, VElem,
};
use crate::model::{ListItemLike, ListLike, Numbering, NumberingPattern, ParElem};
use crate::text::TextElem;

/// 番号付きリスト。
///
/// 一連の項目を縦に並べて表示し、それぞれに連番を付けます。
///
/// # 例
/// ```example
/// Automatically numbered:
/// + Preparations
/// + Analysis
/// + Conclusions
///
/// Manually numbered:
/// 2. What is the first step?
/// 5. I am confused.
/// +  Moving on ...
///
/// Multiple lines:
/// + This enum item has multiple
///   lines because the next line
///   is indented.
///
/// Function call.
/// #enum[First][Second]
/// ```
///
/// setルールを用いることで、
/// すべてのリストを異なるナンバリングスタイルに簡単に切り替えることができます。
/// ```example
/// #set enum(numbering: "a)")
///
/// + Starting off ...
/// + Don't forget step two
/// ```
///
/// また、[`enum.item`]($enum.item)を使用して、
/// リストの各項目の番号を自由にカスタマイズすることもできます。
///
/// ```example
/// #enum(
///   enum.item(1)[First step],
///   enum.item(5)[Fifth step],
///   enum.item(10)[Tenth step]
/// )
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
///
/// - 行の先頭にプラス記号をつけると、
///   自動的にナンバリングされたリスト項目が作成されます。
/// - 行の先頭に数字とピリオドを付けると、
///   明示的に番号を指定したリスト項目が作成されます。
///
/// リストの項目には、複数の段落やその他のブロックレベルのコンテンツを含めることができます。
/// 項目のマーカーよりもインデントが深いコンテンツはすべて、
/// その項目の一部となります。
#[elem(scope, title = "Numbered List", Show)]
pub struct EnumElem {
    /// リストのデフォルトの[spacing]($enum.spacing)を定義します。
    /// これが`{false}`の場合、
    /// 項目の間隔は[paragraph spacing]($par.spacing)によって決まります。
    /// `{true}`の場合、代わりに[paragraph leading]($par.leading)が使用されます。
    /// これによりリストがよりコンパクトになり、各項目が短い場合に見栄えが良くなります。
    ///
    /// マークアップモードでは、
    /// この引数の値は項目が空行で区切られているかどうかに基づいて決定されます。
    /// 項目が直接連続している場合、この値は`{true}`に設定されますが、
    /// 項目が空行で区切られている場合は`{false}`に設定されます。
    /// マークアップで定義された間隔はsetルールで上書きすることは出来ません。
    ///
    /// ```example
    /// + If an enum has a lot of text, and
    ///   maybe other inline content, it
    ///   should not be tight anymore.
    ///
    /// + To make an enum wide, simply
    ///   insert a blank line between the
    ///   items.
    /// ```
    #[default(true)]
    pub tight: bool,

    /// リストをどのようにナンバリングするかを指定します。
    /// [ナンバリングパターンまたは関数]($numbering)を受け付けます。
    ///
    /// ナンバリングのパターンに複数のカウント記号が含まれている場合、
    /// それらはネストされたリストに適用されます。
    /// 関数が指定された場合、`full`が`{false}`の場合は1つの引数を受け取り、`{true}`の場合は複数の引数を受け取ります。
    ///
    /// ```example
    /// #set enum(numbering: "1.a)")
    /// + Different
    /// + Numbering
    ///   + Nested
    ///   + Items
    /// + Style
    ///
    /// #set enum(numbering: n => super[#n])
    /// + Superscript
    /// + Numbering!
    /// ```
    #[default(Numbering::Pattern(NumberingPattern::from_str("1.").unwrap()))]
    #[borrowed]
    pub numbering: Numbering,

    /// リストの開始番号を指定します。
    ///
    /// ```example
    /// #enum(
    ///   start: 3,
    ///   [Skipping],
    ///   [Ahead],
    /// )
    /// ```
    #[default(1)]
    pub start: usize,

    /// 親リストの番号も含めて、
    /// 完全なナンバリングを表示するかどうかを指定します。
    ///
    ///
    /// ```example
    /// #set enum(numbering: "1.a)", full: true)
    /// + Cook
    ///   + Heat water
    ///   + Add ingredients
    /// + Eat
    /// ```
    #[default(false)]
    pub full: bool,

    /// 各項目のインデント。
    #[resolve]
    pub indent: Length,

    /// 各項目のナンバリングと本文の間隔を指定します。
    #[resolve]
    #[default(Em::new(0.5).into())]
    pub body_indent: Length,

    /// リストの項目同士の間隔を指定します。
    ///
    /// `{auto}`に設定すると、
    /// コンパクトなリストの場合は段落の[leading]($par.leading)を、
    /// 幅のある（コンパクトでない）リストの場合は段落の[spacing]($par.spacing)を使用します。
    pub spacing: Smart<Length>,

    /// リストの番号の配置を指定します。
    ///
    /// デフォルトでは、この値は`{end + top}`に設定されており、これはリストの番号を
    /// 現在のテキスト方向の終端（例えば、左から右へ書く文書では、これは`{right}`と同じ）と、
    /// 行の上部に揃えます。
    /// 一般的に、水平方向の番号の配置には`{start}`よりも`{end}`を選択することが推奨されます。
    /// なぜなら、番号がテキストに向かってではなくテキストから離れる方向に伸びることによって、
    /// 特定の視覚的な問題を回避できるからです。
    /// しかし、このオプションを使用することで、この動作を上書きすることができます。
    /// （また、[unordered list]($list)は異なる方法を用いており、直接`marker`コンテンツに配置を指定することで、
    /// これを行っていることに注意してください）
    ///
    /// ````example
    /// #set enum(number-align: start + bottom)
    ///
    /// Here are some powers of two:
    /// 1. One
    /// 2. Two
    /// 4. Four
    /// 8. Eight
    /// 16. Sixteen
    /// 32. Thirty two
    /// ````
    #[default(HAlignment::End + VAlignment::Top)]
    pub number_align: Alignment,

    /// 番号付きリストの項目。
    ///
    /// enum構文を使用する場合、forループのような構造を挟んでも、
    /// 隣接する項目は自動的にリストとしてまとめられます。
    ///
    /// ```example
    /// #for phase in (
    ///    "Launch",
    ///    "Orbit",
    ///    "Descent",
    /// ) [+ #phase]
    /// ```
    #[variadic]
    pub children: Vec<Packed<EnumItem>>,

    /// The numbers of parent items.
    #[internal]
    #[fold]
    #[ghost]
    parents: SmallVec<[usize; 4]>,
}

#[scope]
impl EnumElem {
    #[elem]
    type EnumItem;
}

impl Show for Packed<EnumElem> {
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let mut realized = BlockElem::multi_layouter(self.clone(), layout_enum)
            .pack()
            .spanned(self.span());

        if self.tight(styles) {
            let leading = ParElem::leading_in(styles);
            let spacing =
                VElem::new(leading.into()).with_weak(true).with_attach(true).pack();
            realized = spacing + realized;
        }

        Ok(realized)
    }
}

/// Layout the enumeration.
#[typst_macros::time(span = elem.span())]
fn layout_enum(
    elem: &Packed<EnumElem>,
    engine: &mut Engine,
    locator: Locator,
    styles: StyleChain,
    regions: Regions,
) -> SourceResult<Fragment> {
    let numbering = elem.numbering(styles);
    let indent = elem.indent(styles);
    let body_indent = elem.body_indent(styles);
    let gutter = elem.spacing(styles).unwrap_or_else(|| {
        if elem.tight(styles) {
            ParElem::leading_in(styles).into()
        } else {
            ParElem::spacing_in(styles).into()
        }
    });

    let mut cells = vec![];
    let mut locator = locator.split();
    let mut number = elem.start(styles);
    let mut parents = EnumElem::parents_in(styles);

    let full = elem.full(styles);

    // Horizontally align based on the given respective parameter.
    // Vertically align to the top to avoid inheriting `horizon` or `bottom`
    // alignment from the context and having the number be displaced in
    // relation to the item it refers to.
    let number_align = elem.number_align(styles);

    for item in elem.children() {
        number = item.number(styles).unwrap_or(number);

        let context = Context::new(None, Some(styles));
        let resolved = if full {
            parents.push(number);
            let content = numbering.apply(engine, context.track(), &parents)?.display();
            parents.pop();
            content
        } else {
            match numbering {
                Numbering::Pattern(pattern) => {
                    TextElem::packed(pattern.apply_kth(parents.len(), number))
                }
                other => other.apply(engine, context.track(), &[number])?.display(),
            }
        };

        // Disable overhang as a workaround to end-aligned dots glitching
        // and decreasing spacing between numbers and items.
        let resolved =
            resolved.aligned(number_align).styled(TextElem::set_overhang(false));

        cells.push(Cell::new(Content::empty(), locator.next(&())));
        cells.push(Cell::new(resolved, locator.next(&())));
        cells.push(Cell::new(Content::empty(), locator.next(&())));
        cells.push(Cell::new(
            item.body.clone().styled(EnumElem::set_parents(smallvec![number])),
            locator.next(&item.body.span()),
        ));
        number = number.saturating_add(1);
    }

    let grid = CellGrid::new(
        Axes::with_x(&[
            Sizing::Rel(indent.into()),
            Sizing::Auto,
            Sizing::Rel(body_indent.into()),
            Sizing::Auto,
        ]),
        Axes::with_y(&[gutter.into()]),
        cells,
    );
    let layouter = GridLayouter::new(&grid, regions, styles, elem.span());

    layouter.layout(engine)
}

/// 番号付きリストの項目。
#[elem(name = "item", title = "Numbered List Item")]
pub struct EnumItem {
    /// 項目の番号。
    #[positional]
    pub number: Option<usize>,

    /// 項目の本文。
    #[required]
    pub body: Content,
}

cast! {
    EnumItem,
    array: Array => {
        let mut iter = array.into_iter();
        let (number, body) = match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), None) => (a.cast()?, b.cast()?),
            _ => bail!("array must contain exactly two entries"),
        };
        Self::new(body).with_number(number)
    },
    v: Content => v.unpack::<Self>().unwrap_or_else(Self::new),
}

impl ListLike for EnumElem {
    type Item = EnumItem;

    fn create(children: Vec<Packed<Self::Item>>, tight: bool) -> Self {
        Self::new(children).with_tight(tight)
    }
}

impl ListItemLike for EnumItem {
    fn styled(mut item: Packed<Self>, styles: Styles) -> Packed<Self> {
        item.body.style_in_place(styles);
        item
    }
}
