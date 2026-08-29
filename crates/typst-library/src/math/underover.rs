use crate::foundations::{Content, elem};
use crate::math::Mathy;

<<<<<<< HEAD
/// コンテンツの下にある水平方向の線。
=======
/// A horizontal line under content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の線。
=======
/// A horizontal line over content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の波括弧。その下にオプションで注釈ができます。
=======
/// A horizontal brace under content, with an optional annotation below.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ underbrace(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderbraceElem {
<<<<<<< HEAD
    /// 波括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 波括弧の下にあるオプションのコンテンツ。
=======
    /// The content above the brace.
    #[required]
    pub body: Content,

    /// The optional content below the brace.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の波括弧。その上にオプションで注釈ができます。
=======
/// A horizontal brace over content, with an optional annotation above.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ overbrace(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverbraceElem {
<<<<<<< HEAD
    /// 波括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 波括弧の上にあるオプションのコンテンツ。
=======
    /// The content below the brace.
    #[required]
    pub body: Content,

    /// The optional content above the brace.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の角括弧。その下にオプションで注釈ができます。
=======
/// A horizontal bracket under content, with an optional annotation below.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ underbracket(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderbracketElem {
<<<<<<< HEAD
    /// 角括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 角括弧の下にあるオプションのコンテンツ。
=======
    /// The content above the bracket.
    #[required]
    pub body: Content,

    /// The optional content below the bracket.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の角括弧。その上にオプションで注釈ができます。
=======
/// A horizontal bracket over content, with an optional annotation above.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ overbracket(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverbracketElem {
<<<<<<< HEAD
    /// 角括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 角括弧の上にあるオプションのコンテンツ。
=======
    /// The content below the bracket.
    #[required]
    pub body: Content,

    /// The optional content above the bracket.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の丸括弧。その下にオプションで注釈ができます。
=======
/// A horizontal parenthesis under content, with an optional annotation below.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ underparen(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UnderparenElem {
<<<<<<< HEAD
    /// 丸括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 丸括弧の下にあるオプションのコンテンツ。
=======
    /// The content above the parenthesis.
    #[required]
    pub body: Content,

    /// The optional content below the parenthesis.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの上にある水平方向の丸括弧。その上にオプションで注釈ができます。
=======
/// A horizontal parenthesis over content, with an optional annotation above.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ overparen(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OverparenElem {
<<<<<<< HEAD
    /// 丸括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 丸括弧の上にあるオプションのコンテンツ。
=======
    /// The content below the parenthesis.
    #[required]
    pub body: Content,

    /// The optional content above the parenthesis.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}

<<<<<<< HEAD
/// コンテンツの下にある水平方向の亀甲括弧。その下にオプションで注釈ができます。
=======
/// A horizontal tortoise shell bracket under content, with an optional
/// annotation below.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ undershell(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct UndershellElem {
<<<<<<< HEAD
    /// 亀甲括弧の上にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 亀甲括弧の下にあるオプションのコンテンツ。
    #[positional]
    pub annotation: Option<Content>,
}
/// コンテンツの上にある水平方向の亀甲括弧。その上にオプションで注釈ができます。
=======
    /// The content above the tortoise shell bracket.
    #[required]
    pub body: Content,

    /// The optional content below the tortoise shell bracket.
    #[positional]
    pub annotation: Option<Content>,
}

/// A horizontal tortoise shell bracket over content, with an optional
/// annotation above.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ overshell(0 + 1 + dots.c + n, n + 1 "numbers") $
/// ```
#[elem(Mathy)]
pub struct OvershellElem {
<<<<<<< HEAD
    /// 亀甲括弧の下にあるコンテンツ。
    #[required]
    pub body: Content,

    /// 亀甲括弧の上にあるオプションのコンテンツ。
=======
    /// The content below the tortoise shell bracket.
    #[required]
    pub body: Content,

    /// The optional content above the tortoise shell bracket.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub annotation: Option<Content>,
}
