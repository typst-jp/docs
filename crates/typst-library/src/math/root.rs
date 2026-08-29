use typst_syntax::Span;

use crate::foundations::{Content, NativeElement, elem, func};
use crate::math::Mathy;

<<<<<<< HEAD
/// 平方根。
=======
/// A square root.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ sqrt(3 - 2 sqrt(2)) = sqrt(2) - 1 $
/// ```
#[func(title = "Square Root")]
pub fn sqrt(
    span: Span,
<<<<<<< HEAD
    /// 平方根を取る対象の式。
=======
    /// The expression to take the square root of.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    radicand: Content,
) -> Content {
    RootElem::new(radicand).pack().spanned(span)
}

<<<<<<< HEAD
/// 冪根。
=======
/// A general root.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ root(3, x) $
/// ```
#[elem(Mathy)]
pub struct RootElem {
<<<<<<< HEAD
    /// 被開方数の何乗根を取るか。
    #[positional]
    pub index: Option<Content>,

    /// 根を取る対象の式。
=======
    /// Which root of the radicand to take.
    #[positional]
    pub index: Option<Content>,

    /// The expression to take the root of.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub radicand: Content,
}
