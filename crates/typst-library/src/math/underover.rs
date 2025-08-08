use crate::foundations::{elem, Content};
use crate::math::Mathy;

/// コンテンツの下にある水平方向の線。
///
/// ```example
/// $ underline(1 + 2 + ... + 5) $
/// ```
#[elem(Mathy)]
pub struct UnderlineElem {
    /// 線の上にあるコンテンツ。
    #[required]
    pub body: Content,
}

/// コンテンツの上にある水平方向の線。
///
/// ```example
/// $ overline(1 + 2 + ... + 5) $
/// ```
#[elem(Mathy)]
pub struct OverlineElem {
    /// 線の下にあるコンテンツ。
    #[required]
    pub body: Content,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}

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
    #[positional]
    pub annotation: Option<Content>,
}
