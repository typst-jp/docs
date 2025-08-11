use comemo::Track;

use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Array, Content, Context, Depth, Func, NativeElement, Packed, Show,
    Smart, StyleChain, Styles, TargetElem, Value,
};
use crate::html::{tag, HtmlElem};
use crate::layout::{BlockElem, Em, Length, VElem};
use crate::model::{ParElem, ParbreakElem};
use crate::text::TextElem;

/// 箇条書きリスト。
///
/// 各項目の先頭にマーカーを付け、
/// 一連の項目を縦に並べて表示します。
///
/// # 例
/// ```example
/// Normal list.
/// - Text
/// - Math
/// - Layout
/// - ...
///
/// Multiple lines.
/// - This list item spans multiple
///   lines because it is indented.
///
/// Function call.
/// #list(
///   [Foundations],
///   [Calculate],
///   [Construct],
///   [Data Loading],
/// )
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
/// 行頭にハイフンとスペースを置くことでリスト項目を作成します。
/// リスト項目には複数の段落や、他のブロックレベルコンテンツを含めることができます。
/// リスト項目の記号よりも深く字下げされた全てのコンテンツは、そのリスト項目の一部になります。
#[elem(scope, title = "Bullet List", Show)]
pub struct ListElem {
    /// リストのデフォルトの[spacing]($list.spacing)を定義します。
    /// これが`{false}`の場合、 項目の間隔は[paragraph spacing]($par.spacing)によって決まります。
    /// `{true}`の場合、代わりに[paragraph leading]($par.leading)が使用されます。
    /// これによりリストがよりコンパクトになり、
    /// 各項目が短い場合に見栄えが良くなります。
    ///
    /// マークアップモードでは、
    /// この引数の値は項目が空行で区切られているかどうかに基づいて決定されます。
    /// 項目間に空行がなく連続している場合、この値は`{true}`に設定されますが、
    /// 項目間が空行で区切られている場合は`{false}`に設定されます。
    /// マークアップで定義された間隔はsetルールで上書きすることはできません。
    ///
    /// ```example
    /// - If a list has a lot of text, and
    ///   maybe other inline content, it
    ///   should not be tight anymore.
    ///
    /// - To make a list wide, simply insert
    ///   a blank line between the items.
    /// ```
    #[default(true)]
    pub tight: bool,

    /// 各項目の先頭に付けるマーカー。
    ///
    /// 単純なコンテンツの代わりに、ネストされたリストに使用する、
    /// 複数のマーカーを持つ配列を渡すこともできます。
    /// リストのネストの深さがマーカーの数を超えた場合、使用されるマーカーは循環します。
    /// 完全に制御したい場合は、
    /// リストのネストの深さ（`{0}`から開始する）に応じて、使用するマーカーを決める関数を渡すこともできます。
    ///
    /// ```example
    /// #set list(marker: [--])
    /// - A more classic list
    /// - With en-dashes
    ///
    /// #set list(marker: ([•], [--]))
    /// - Top-level
    ///   - Nested
    ///   - Items
    /// - Items
    /// ```
    #[borrowed]
    #[default(ListMarker::Content(vec![
        // These are all available in the default font, vertically centered, and
        // roughly of the same size (with the last one having slightly lower
        // weight because it is not filled).
        TextElem::packed('\u{2022}'), // Bullet
        TextElem::packed('\u{2023}'), // Triangular Bullet
        TextElem::packed('\u{2013}'), // En-dash
    ]))]
    pub marker: ListMarker,

    /// 各項目のインデント。
    #[resolve]
    pub indent: Length,

    /// 各項目のマーカーと本文の間隔を指定します。
    #[resolve]
    #[default(Em::new(0.5).into())]
    pub body_indent: Length,

    /// リストの項目同士の間隔を指定します。
    ///
    /// `{auto}`に設定すると、
    /// コンパクトなリストの場合は[`leading`]($par.leading)を、
    /// 幅のある（コンパクトでない）リストの場合は段落の[`spacing`]($par.spacing)を使用します。
    pub spacing: Smart<Length>,

    /// 箇条書きリストの項目。
    ///
    /// list構文を使用する場合、forループのような構造を挟んでも、 
    /// 隣接する項目は自動的にリストとしてまとめられます。
    ///
    /// ```example
    /// #for letter in "ABC" [
    ///   - Letter #letter
    /// ]
    /// ```
    #[variadic]
    pub children: Vec<Packed<ListItem>>,

    /// The nesting depth.
    #[internal]
    #[fold]
    #[ghost]
    pub depth: Depth,
}

#[scope]
impl ListElem {
    #[elem]
    type ListItem;
}

impl Show for Packed<ListElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let tight = self.tight(styles);

        if TargetElem::target_in(styles).is_html() {
            return Ok(HtmlElem::new(tag::ul)
                .with_body(Some(Content::sequence(self.children.iter().map(|item| {
                    // Text in wide lists shall always turn into paragraphs.
                    let mut body = item.body.clone();
                    if !tight {
                        body += ParbreakElem::shared();
                    }
                    HtmlElem::new(tag::li)
                        .with_body(Some(body))
                        .pack()
                        .spanned(item.span())
                }))))
                .pack()
                .spanned(self.span()));
        }

        let mut realized =
            BlockElem::multi_layouter(self.clone(), engine.routines.layout_list)
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

/// 箇条書きリストの項目。
#[elem(name = "item", title = "Bullet List Item")]
pub struct ListItem {
    /// 項目の本文。
    #[required]
    pub body: Content,
}

cast! {
    ListItem,
    v: Content => v.unpack::<Self>().unwrap_or_else(Self::new)
}

/// A list's marker.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ListMarker {
    Content(Vec<Content>),
    Func(Func),
}

impl ListMarker {
    /// Resolve the marker for the given depth.
    pub fn resolve(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        depth: usize,
    ) -> SourceResult<Content> {
        Ok(match self {
            Self::Content(list) => {
                list.get(depth % list.len()).cloned().unwrap_or_default()
            }
            Self::Func(func) => func
                .call(engine, Context::new(None, Some(styles)).track(), [depth])?
                .display(),
        })
    }
}

cast! {
    ListMarker,
    self => match self {
        Self::Content(vec) => if vec.len() == 1 {
            vec.into_iter().next().unwrap().into_value()
        } else {
            vec.into_value()
        },
        Self::Func(func) => func.into_value(),
    },
    v: Content => Self::Content(vec![v]),
    array: Array => {
        if array.is_empty() {
            bail!("array must contain at least one marker");
        }
        Self::Content(array.into_iter().map(Value::display).collect())
    },
    v: Func => Self::Func(v),
}

/// A list, enum, or term list.
pub trait ListLike: NativeElement {
    /// The kind of list item this list is composed of.
    type Item: ListItemLike;

    /// Create this kind of list from its children and tightness.
    fn create(children: Vec<Packed<Self::Item>>, tight: bool) -> Self;
}

/// A list item, enum item, or term list item.
pub trait ListItemLike: NativeElement {
    /// Apply styles to the element's body.
    fn styled(item: Packed<Self>, styles: Styles) -> Packed<Self>;
}

impl ListLike for ListElem {
    type Item = ListItem;

    fn create(children: Vec<Packed<Self::Item>>, tight: bool) -> Self {
        Self::new(children).with_tight(tight)
    }
}

impl ListItemLike for ListItem {
    fn styled(mut item: Packed<Self>, styles: Styles) -> Packed<Self> {
        item.body.style_in_place(styles);
        item
    }
}
