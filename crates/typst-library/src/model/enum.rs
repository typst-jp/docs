use std::str::FromStr;

use smallvec::SmallVec;

use crate::diag::bail;
use crate::foundations::{Array, Content, Packed, Smart, Styles, cast, elem, scope};
use crate::introspection::{Locatable, Tagged};
use crate::layout::{Alignment, Em, HAlignment, Length, VAlignment};
use crate::model::{ListItemLike, ListLike, Numbering, NumberingPattern};

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
/// 全てのリストを異なる番号付けスタイルに簡単に切り替えることができます。
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
///   自動的に番号付けされたリスト項目が作成されます。
/// - 行の先頭に数字とピリオドを付けると、
///   明示的に番号を指定したリスト項目が作成されます。
///
/// リストの項目には、複数の段落やその他のブロックレベルのコンテンツを含めることができます。
/// 項目のマーカーよりもインデントが深いコンテンツは全て、
/// その項目の一部となります。
#[elem(scope, title = "Numbered List", Locatable, Tagged)]
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

    /// リストをどのように番号付けするかを指定します。
    /// [番号付けパターンまたは関数]($numbering)を受け付けます。
    ///
    /// 番号付けのパターンに複数のカウント記号が含まれている場合、
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
    pub start: Smart<u64>,

    /// 親リストの番号も含めて、
    /// 完全な番号付けを表示するかどうかを指定します。
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

    /// このリストの番号付けを逆順にするかどうかを指定します。
    ///
    /// ```example
    /// #set enum(reversed: true)
    /// + Coffee
    /// + Tea
    /// + Milk
    /// ```
    #[default(false)]
    pub reversed: bool,

    /// 各項目のインデント。
    pub indent: Length,

    /// 各項目の番号付けと本文の間隔を指定します。
    #[default(Em::new(0.5).into())]
    pub body_indent: Length,

    /// リストの項目同士の間隔を指定します。
    ///
    /// `{auto}`に設定すると、
    /// コンパクトなリストの場合は段落の[`leading`]($par.leading)を、
    /// 幅のある（コンパクトでない）リストの場合は段落の[`spacing`]($par.spacing)を使用します。
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
    pub parents: SmallVec<[u64; 4]>,
}

#[scope]
impl EnumElem {
    #[elem]
    type EnumItem;
}

/// 番号付きリストの項目。
#[elem(name = "item", title = "Numbered List Item", Tagged)]
pub struct EnumItem {
    /// 項目の番号。
    #[positional]
    pub number: Smart<u64>,

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
