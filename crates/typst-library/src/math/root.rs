use typst_syntax::Span;

<<<<<<< HEAD
use crate::foundations::{elem, func, Content, NativeElement};
use crate::math::Mathy;

/// 平方根。
=======
use crate::foundations::{Content, NativeElement, elem, func};
use crate::math::Mathy;

/// A square root.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    radicand: Content,
) -> Content {
    RootElem::new(radicand).pack().spanned(span)
}

<<<<<<< HEAD
/// 冪根。
=======
/// A general root.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub radicand: Content,
}
