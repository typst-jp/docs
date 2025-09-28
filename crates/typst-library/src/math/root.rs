use typst_syntax::Span;

use crate::foundations::{elem, func, Content, NativeElement};
use crate::math::Mathy;

/// 平方根。
///
/// ```example
/// $ sqrt(3 - 2 sqrt(2)) = sqrt(2) - 1 $
/// ```
#[func(title = "Square Root")]
pub fn sqrt(
    span: Span,
    /// 平方根を取る対象の式。
    radicand: Content,
) -> Content {
    RootElem::new(radicand).pack().spanned(span)
}

/// 冪根。
///
/// ```example
/// $ root(3, x) $
/// ```
#[elem(Mathy)]
pub struct RootElem {
    /// 被開方数の何乗根を取るか。
    #[positional]
    pub index: Option<Content>,

    /// 根を取る対象の式。
    #[required]
    pub radicand: Content,
}
