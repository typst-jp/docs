use std::fmt::{self, Debug, Formatter};

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{cast, elem, Content, NativeElement, Packed, Show, StyleChain};
use crate::layout::{BlockElem, Dir, Spacing};

/// コンテンツと間隔を垂直または水平方向に配置。
///
/// スタックは、ある軸に沿ってアイテムのリストを配置し、各アイテム間に任意の間隔を設定します。
///
/// # 例
/// ```example
/// #stack(
///   dir: ttb,
///   rect(width: 40pt),
///   rect(width: 120pt),
///   rect(width: 90pt),
/// )
/// ```
#[elem(Show)]
pub struct StackElem {
    /// アイテムを積み重ねる向き。可能な値は以下の通りです。
    ///
    /// - `{ltr}`: 左から右。
    /// - `{rtl}`: 右から左。
    /// - `{ttb}`: 上から下。
    /// - `{btt}`: 下から上。
    ///
    /// `alignment`と同様に、向きの始点と終点を（それぞれ）取得するために、`start`と`end`メソッドを使用できます。
    /// 向きが`{"horizontal"}`か`{"vertical"}`のどちらに属するかを決定するために`axis`メソッドも使用できます。
    /// `inv`メソッドは逆の向きを返します。
    ///
    /// 例えば、`{ttb.start()}`は`top`、`{ttb.end()}`は`bottom`、`{ttb.axis()}`は`{"vertical"}`となり、`{ttb.inv()}`は`btt`に等しくなります。
    #[default(Dir::TTB)]
    pub dir: Dir,

    /// 明示的に間隔が与えられなかった場合にアイテム間に挿入される間隔。
    pub spacing: Option<Spacing>,

    /// 軸に沿って積み重ねる子要素。
    #[variadic]
    pub children: Vec<StackChild>,
}

impl Show for Packed<StackElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::multi_layouter(self.clone(), engine.routines.layout_stack)
            .pack()
            .spanned(self.span()))
    }
}

/// A child of a stack element.
#[derive(Clone, PartialEq, Hash)]
pub enum StackChild {
    /// Spacing between other children.
    Spacing(Spacing),
    /// Arbitrary block-level content.
    Block(Content),
}

impl Debug for StackChild {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Spacing(kind) => kind.fmt(f),
            Self::Block(block) => block.fmt(f),
        }
    }
}

cast! {
    StackChild,
    self => match self {
        Self::Spacing(spacing) => spacing.into_value(),
        Self::Block(content) => content.into_value(),
    },
    v: Spacing => Self::Spacing(v),
    v: Content => Self::Block(v),
}
