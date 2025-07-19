use typst_utils::Numeric;

use crate::foundations::{cast, elem, Content};
use crate::layout::{Abs, Em, Fr, Length, Ratio, Rel};

/// パラグラフに水平方向の間隔を挿入。
///
/// 間隔の大きさは絶対的な値、相対的な値、または比率で指定できます。
/// 比率指定の場合は、比率指定されたそれぞれの間隔に、行の残りの間隔がその相対比率に応じて配分されます。
///
/// # 例
/// ```example
/// First #h(1cm) Second \
/// First #h(30%) Second
/// ```
///
/// # 比率指定の間隔
/// 比率指定の間隔を用いると、（[`align`]のように）段落区切りを強制することなく行内の配置が可能です。
/// 要素の大きさが比率で指定された場合、それぞれの要素には、比率の総和に対する自身の比率の割合に応じた間隔が割り当てられます。
///
/// ```example
/// First #h(1fr) Second \
/// First #h(1fr) Second #h(1fr) Third \
/// First #h(2fr) Second #h(1fr) Third
/// ```
///
/// # 数式用の間隔 { #math-spacing }
/// [数式]($category/math)中では、要素間に間隔を挿入するための定数として、`thin`（1/6 em）、`med`（2/9 em）、`thick`（5/18 em）、`quad`（1 em）および`wide`（2 em）も利用可能です。
#[elem(title = "Spacing (H)")]
pub struct HElem {
    /// 挿入する間隔の大きさ。
    #[required]
    pub amount: Spacing,

    /// `{true}`の場合、段落の始まりと終わりの空白は削除されます。
    /// さらに、弱い間隔が隣接していると最大のもの以外は削除されます。
    ///
    /// マークアップ中に弱い間隔があると、挿入された間隔の総量によらず、全ての隣接するマークアップの間隔が削除されます。
    /// 明示的に（通常の間隔の場合は）`[#" "]`と書くか、（改行しない間隔の場合は）`[~]`と書くことで弱い間隔の隣に間隔を強制することができます。
    /// 後者は、マークアップで直前に間隔があったかどうかに関係なく、直前の単語に改行しない間隔を常に1つ追加する構造を作成する際に便利かもしれません。
    ///
    /// ```example
    /// #h(1cm, weak: true)
    /// We identified a group of _weak_
    /// specimens that fail to manifest
    /// in most cases. However, when
    /// #h(8pt, weak: true) supported
    /// #h(8pt, weak: true) on both sides,
    /// they do show up.
    ///
    /// Further #h(0pt, weak: true) more,
    /// even the smallest of them swallow
    /// adjacent markup spaces.
    /// ```
    #[default(false)]
    pub weak: bool,
}

impl HElem {
    /// Zero-width horizontal weak spacing that eats surrounding spaces.
    pub fn hole() -> Self {
        Self::new(Abs::zero().into()).with_weak(true)
    }
}

/// ブロックの流れに垂直方向の間隔を挿入。
///
/// 間隔の大きさは絶対的な値、相対的な値、または比率で指定できます。
/// 比率指定の場合は、比率指定されたそれぞれの間隔に、ページの残りの間隔がその相対比率に応じて配分されます。
///
/// # 例
/// ```example
/// #grid(
///   rows: 3cm,
///   columns: 6,
///   gutter: 1fr,
///   [A #parbreak() B],
///   [A #v(0pt) B],
///   [A #v(10pt) B],
///   [A #v(0pt, weak: true) B],
///   [A #v(40%, weak: true) B],
///   [A #v(1fr) B],
/// )
/// ```
#[elem(title = "Spacing (V)")]
pub struct VElem {
    /// 挿入する間隔の大きさ。
    #[required]
    pub amount: Spacing,

    /// `{true}`の場合、流れの始まりと終わりの空白は削除されます。
    /// さらに、弱い間隔が隣接していると最大のもの以外は削除されます。
    /// たとえ段落間隔の方が大きかったとしても、弱い間隔に隣接する段落間隔は常に削除されます。
    ///
    /// ```example
    /// The following theorem is
    /// foundational to the field:
    /// #v(4pt, weak: true)
    /// $ x^2 + y^2 = r^2 $
    /// #v(4pt, weak: true)
    /// The proof is simple:
    /// ```
    pub weak: bool,

    /// Whether the spacing collapses if not immediately preceded by a
    /// paragraph.
    #[internal]
    #[parse(Some(false))]
    pub attach: bool,
}

cast! {
    VElem,
    v: Content => v.unpack::<Self>().map_err(|_| "expected `v` element")?,
}

/// Kinds of spacing.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Spacing {
    /// Spacing specified in absolute terms and relative to the parent's size.
    Rel(Rel<Length>),
    /// Spacing specified as a fraction of the remaining free space in the
    /// parent.
    Fr(Fr),
}

impl Spacing {
    /// Whether this is fractional spacing.
    pub fn is_fractional(self) -> bool {
        matches!(self, Self::Fr(_))
    }

    /// Whether the spacing is actually no spacing.
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Rel(rel) => rel.is_zero(),
            Self::Fr(fr) => fr.is_zero(),
        }
    }
}

impl From<Abs> for Spacing {
    fn from(abs: Abs) -> Self {
        Self::Rel(abs.into())
    }
}

impl From<Em> for Spacing {
    fn from(em: Em) -> Self {
        Self::Rel(Rel::new(Ratio::zero(), em.into()))
    }
}

impl From<Length> for Spacing {
    fn from(length: Length) -> Self {
        Self::Rel(length.into())
    }
}

impl From<Fr> for Spacing {
    fn from(fr: Fr) -> Self {
        Self::Fr(fr)
    }
}

cast! {
    Spacing,
    self => match self {
        Self::Rel(rel) => {
            if rel.rel.is_zero() {
                rel.abs.into_value()
            } else if rel.abs.is_zero() {
                rel.rel.into_value()
            } else {
                rel.into_value()
            }
        }
        Self::Fr(fr) => fr.into_value(),
    },
    v: Rel<Length> => Self::Rel(v),
    v: Fr => Self::Fr(v),
}
