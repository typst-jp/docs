use std::str::FromStr;

<<<<<<< HEAD
use ecow::eco_format;
use smallvec::SmallVec;

use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Array, Content, NativeElement, Packed, Show, Smart, StyleChain,
    Styles, TargetElem,
};
use crate::html::{attr, tag, HtmlElem};
use crate::layout::{Alignment, BlockElem, Em, HAlignment, Length, VAlignment, VElem};
use crate::model::{
    ListItemLike, ListLike, Numbering, NumberingPattern, ParElem, ParbreakElem,
};

/// 番号付きリスト。
///
/// 一連の項目を縦に並べて表示し、それぞれに連番を付けます。
///
/// # 例
=======
use smallvec::SmallVec;

use crate::diag::bail;
use crate::foundations::{Array, Content, Packed, Smart, Styles, cast, elem, scope};
use crate::introspection::{Locatable, Tagged};
use crate::layout::{Alignment, Em, HAlignment, Length, VAlignment};
use crate::model::{ListItemLike, ListLike, Numbering, NumberingPattern};

/// A numbered list.
///
/// Displays a sequence of items vertically and numbers them consecutively.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// setルールを用いることで、
/// 全てのリストを異なる番号付けスタイルに簡単に切り替えることができます。
=======
/// You can easily switch all your enumerations to a different numbering style
/// with a set rule.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #set enum(numbering: "a)")
///
/// + Starting off ...
/// + Don't forget step two
/// ```
///
<<<<<<< HEAD
/// また、[`enum.item`]($enum.item)を使用して、
/// リストの各項目の番号を自由にカスタマイズすることもできます。
=======
/// You can also use [`enum.item`] to programmatically customize the number of
/// each item in the enumeration:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #enum(
///   enum.item(1)[First step],
///   enum.item(5)[Fifth step],
///   enum.item(10)[Tenth step]
/// )
/// ```
///
<<<<<<< HEAD
/// # 構文
/// この関数には専用の構文もあります。
///
/// - 行の先頭にプラス記号をつけると、
///   自動的に番号付けされたリスト項目が作成されます。
/// - 行の先頭に数字とピリオドを付けると、
///   明示的に番号を指定したリスト項目が作成されます。
///
/// リストの項目には、複数の段落やその他のブロックレベルのコンテンツを含めることができます。
/// 項目のマーカーよりもインデントが深いコンテンツは全て、
/// その項目の一部となります。
#[elem(scope, title = "Numbered List", Show)]
pub struct EnumElem {
    /// リストのデフォルトの[spacing]($enum.spacing)を定義します。
    /// これが`{false}`の場合、
    /// 項目の間隔は[paragraph spacing]($par.spacing)によって決まります。
    /// `{true}`の場合、代わりに[paragraph leading]($par.leading)が使用されます。
    /// これによりリストがよりコンパクトになり、各項目が短い場合に見栄えがよくなります。
    ///
    /// マークアップモードでは、
    /// この引数の値は項目が空行で区切られているかどうかに基づいて決定されます。
    /// 項目間に空行がなく連続している場合、この値は`{true}`に設定されますが、
    /// 項目間が空行で区切られている場合は`{false}`に設定されます。
    /// マークアップで定義された間隔はsetルールで上書きすることは出来ません。
=======
/// # Syntax
/// This functions also has dedicated syntax:
///
/// - Starting a line with a plus sign creates an automatically numbered
///   enumeration item.
/// - Starting a line with a number followed by a dot creates an explicitly
///   numbered enumeration item.
///
/// Enumeration items can contain multiple paragraphs and other block-level
/// content. All content that is indented more than an item's marker becomes
/// part of that item.
#[elem(scope, title = "Numbered List", Locatable, Tagged)]
pub struct EnumElem {
    /// Defines the default [spacing]($enum.spacing) of the enumeration. If it
    /// is `{false}`, the items are spaced apart with
    /// [paragraph spacing]($par.spacing). If it is `{true}`, they use
    /// [paragraph leading]($par.leading) instead. This makes the list more
    /// compact, which can look better if the items are short.
    ///
    /// In markup mode, the value of this parameter is determined based on
    /// whether items are separated with a blank line. If items directly follow
    /// each other, this is set to `{true}`; if items are separated by a blank
    /// line, this is set to `{false}`. The markup-defined tightness cannot be
    /// overridden with set rules.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// リストをどのように番号付けするかを指定します。
    /// [番号付けパターンまたは関数]($numbering)を受け付けます。
    ///
    /// 番号付けのパターンに複数のカウント記号が含まれている場合、
    /// それらはネストされたリストに適用されます。
    /// 関数が指定された場合、`full`が`{false}`の場合は1つの引数を受け取り、`{true}`の場合は複数の引数を受け取ります。
=======
    /// How to number the enumeration. Accepts a
    /// [numbering pattern or function]($numbering).
    ///
    /// If the numbering pattern contains multiple counting symbols, they apply
    /// to nested enums. If given a function, the function receives one argument
    /// if `full` is `{false}` and multiple arguments if `full` is `{true}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
    #[borrowed]
    pub numbering: Numbering,

    /// リストの開始番号を指定します。
=======
    pub numbering: Numbering,

    /// Which number to start the enumeration with.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #enum(
    ///   start: 3,
    ///   [Skipping],
    ///   [Ahead],
    /// )
    /// ```
<<<<<<< HEAD
    pub start: Smart<usize>,

    /// 親リストの番号も含めて、
    /// 完全な番号付けを表示するかどうかを指定します。
=======
    pub start: Smart<u64>,

    /// Whether to display the full numbering, including the numbers of
    /// all parent enumerations.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// このリストの番号付けを逆順にするかどうかを指定します。
=======
    /// Whether to reverse the numbering for this enumeration.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set enum(reversed: true)
    /// + Coffee
    /// + Tea
    /// + Milk
    /// ```
    #[default(false)]
    pub reversed: bool,

<<<<<<< HEAD
    /// 各項目のインデント。
    #[resolve]
    pub indent: Length,

    /// 各項目の番号付けと本文の間隔を指定します。
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
=======
    /// The indentation of each item.
    pub indent: Length,

    /// The space between the numbering and the body of each item.
    #[default(Em::new(0.5).into())]
    pub body_indent: Length,

    /// The spacing between the items of the enumeration.
    ///
    /// If set to `{auto}`, uses paragraph [`leading`]($par.leading) for tight
    /// enumerations and paragraph [`spacing`]($par.spacing) for wide
    /// (non-tight) enumerations.
    pub spacing: Smart<Length>,

    /// The alignment that enum numbers should have.
    ///
    /// By default, this is set to `{end + top}`, which aligns enum numbers
    /// towards end of the current text direction (in left-to-right script,
    /// for example, this is the same as `{right}`) and at the top of the line.
    /// The choice of `{end}` for horizontal alignment of enum numbers is
    /// usually preferred over `{start}`, as numbers then grow away from the
    /// text instead of towards it, avoiding certain visual issues. This option
    /// lets you override this behaviour, however. (Also to note is that the
    /// [unordered list]($list) uses a different method for this, by giving the
    /// `marker` content an alignment directly.).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// 番号付きリストの項目。
    ///
    /// enum構文を使用する場合、forループのような構造を挟んでも、
    /// 隣接する項目は自動的にリストとしてまとめられます。
=======
    /// The numbered list's items.
    ///
    /// When using the enum syntax, adjacent items are automatically collected
    /// into enumerations, even through constructs like for loops.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
    pub parents: SmallVec<[usize; 4]>,
=======
    pub parents: SmallVec<[u64; 4]>,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

#[scope]
impl EnumElem {
    #[elem]
    type EnumItem;
}

<<<<<<< HEAD
impl Show for Packed<EnumElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let tight = self.tight(styles);

        if TargetElem::target_in(styles).is_html() {
            let mut elem = HtmlElem::new(tag::ol);
            if self.reversed(styles) {
                elem = elem.with_attr(attr::reversed, "reversed");
            }
            if let Some(n) = self.start(styles).custom() {
                elem = elem.with_attr(attr::start, eco_format!("{n}"));
            }
            let body = Content::sequence(self.children.iter().map(|item| {
                let mut li = HtmlElem::new(tag::li);
                if let Some(nr) = item.number(styles) {
                    li = li.with_attr(attr::value, eco_format!("{nr}"));
                }
                // Text in wide enums shall always turn into paragraphs.
                let mut body = item.body.clone();
                if !tight {
                    body += ParbreakElem::shared();
                }
                li.with_body(Some(body)).pack().spanned(item.span())
            }));
            return Ok(elem.with_body(Some(body)).pack().spanned(self.span()));
        }

        let mut realized =
            BlockElem::multi_layouter(self.clone(), engine.routines.layout_enum)
                .pack()
                .spanned(self.span());

        if tight {
            let leading = ParElem::leading_in(styles);
            let spacing =
                VElem::new(leading.into()).with_weak(true).with_attach(true).pack();
            realized = spacing + realized;
        }

        Ok(realized)
    }
}

/// 番号付きリストの項目。
#[elem(name = "item", title = "Numbered List Item")]
pub struct EnumItem {
    /// 項目の番号。
    #[positional]
    pub number: Option<usize>,

    /// 項目の本文。
=======
/// An enumeration item.
#[elem(name = "item", title = "Numbered List Item", Tagged)]
pub struct EnumItem {
    /// The item's number.
    #[positional]
    pub number: Smart<u64>,

    /// The item's body.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
