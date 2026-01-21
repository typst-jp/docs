<<<<<<< HEAD
use crate::foundations::{elem, Content};
use crate::math::Mathy;

/// コンテンツの下にある水平方向の線。
=======
use crate::foundations::{Content, elem};
use crate::math::Mathy;

/// A horizontal line under content.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ underline(1 + 2 + ... + 5) $
/// ```
#[elem(Mathy)]
pub struct UnderlineElem {
<<<<<<< HEAD
    /// 線の上にあるコンテンツ。
=======
    /// The content above the line.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の線。
=======
/// A horizontal line over content.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ overline(1 + 2 + ... + 5) $
/// ```
#[elem(Mathy)]
pub struct OverlineElem {
<<<<<<< HEAD
    /// 線の下にあるコンテンツ。
=======
    /// The content below the line.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の波括弧。その下にオプションで注釈ができます。
///
/// ```example
/// $ underbrace(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderbraceElem {
    /// 波括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 波括弧の下にあるオプションのコンテンツ。
=======
/// A horizontal brace under content, with an optional annotation below.
///
/// ```example
/// $ underbrace(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderbraceElem {
    /// The content above the brace.
    #[required]
    pub body: Content,

    /// The optional content below the brace.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の波括弧。その上にオプションで注釈ができます。
///
/// ```example
/// $ overbrace(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverbraceElem {
    /// 波括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 波括弧の上にあるオプションのコンテンツ。
=======
/// A horizontal brace over content, with an optional annotation above.
///
/// ```example
/// $ overbrace(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverbraceElem {
    /// The content below the brace.
    #[required]
    pub body: Content,

    /// The optional content above the brace.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の角括弧。その下にオプションで注釈ができます。
///
/// ```example
/// $ underbracket(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderbracketElem {
    /// 角括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 角括弧の下にあるオプションのコンテンツ。
=======
/// A horizontal bracket under content, with an optional annotation below.
///
/// ```example
/// $ underbracket(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderbracketElem {
    /// The content above the bracket.
    #[required]
    pub body: Content,

    /// The optional content below the bracket.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の角括弧。その上にオプションで注釈ができます。
///
/// ```example
/// $ overbracket(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverbracketElem {
    /// 角括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 角括弧の上にあるオプションのコンテンツ。
=======
/// A horizontal bracket over content, with an optional annotation above.
///
/// ```example
/// $ overbracket(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverbracketElem {
    /// The content below the bracket.
    #[required]
    pub body: Content,

    /// The optional content above the bracket.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の丸括弧。その下にオプションで注釈ができます。
///
/// ```example
/// $ underparen(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderparenElem {
    /// 丸括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 丸括弧の下にあるオプションのコンテンツ。
=======
/// A horizontal parenthesis under content, with an optional annotation below.
///
/// ```example
/// $ underparen(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderparenElem {
    /// The content above the parenthesis.
    #[required]
    pub body: Content,

    /// The optional content below the parenthesis.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の丸括弧。その上にオプションで注釈ができます。
///
/// ```example
/// $ overparen(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverparenElem {
    /// 丸括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 丸括弧の上にあるオプションのコンテンツ。
=======
/// A horizontal parenthesis over content, with an optional annotation above.
///
/// ```example
/// $ overparen(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverparenElem {
    /// The content below the parenthesis.
    #[required]
    pub body: Content,

    /// The optional content above the parenthesis.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の亀甲括弧。その下にオプションで注釈ができます。
///
/// ```example
/// $ undershell(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct UndershellElem {
    /// 亀甲括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 亀甲括弧の下にあるオプションのコンテンツ。
=======
/// A horizontal tortoise shell bracket under content, with an optional
/// annotation below.
///
/// ```example
/// $ undershell(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UndershellElem {
    /// The content above the tortoise shell bracket.
    #[required]
    pub body: Content,

    /// The optional content below the tortoise shell bracket.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の亀甲括弧。その上にオプションで注釈ができます。
///
/// ```example
/// $ overshell(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(Mathy)]
pub struct OvershellElem {
    /// 亀甲括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 亀甲括弧の上にあるオプションのコンテンツ。
=======
/// A horizontal tortoise shell bracket over content, with an optional
/// annotation above.
///
/// ```example
/// $ overshell(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OvershellElem {
    /// The content below the tortoise shell bracket.
    #[required]
    pub body: Content,

    /// The optional content above the tortoise shell bracket.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    pub annotation: Option<Content>,
}
