use crate::foundations::{elem, func, Content, NativeElement, SymbolElem};
use crate::layout::{Length, Rel};
use crate::math::Mathy;

/// 区切り文字の拡大縮小。
///
/// 対応が取れている区切り文字はデフォルトで拡大縮小しますが、これは対応が取れていない区切り文字を拡大縮小させたり、区切り文字の拡大縮小をより正確に制御するのに便利です。
#[elem(title = "Left/Right", Mathy)]
pub struct LrElem {
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[resolve]
    #[default(Rel::one())]
    pub size: Rel<Length>,

    /// 区切り文字を含めた、区切られるコンテンツ。
    #[required]
    #[parse(
        let mut arguments = args.all::<Content>()?.into_iter();
        let mut body = arguments.next().unwrap_or_default();
        arguments.for_each(|arg| body += SymbolElem::packed(',') + arg);
        body
    )]
    pub body: Content,
}

/// 最も近くで囲んでいる`{lr()}`グループに対して、垂直方向に区切り文字を拡大縮小します。
///
/// ```example
/// $ { x mid(|) sum_(i=1)^n w_i|f_i (x)| < 1 } $
/// ```
#[elem(Mathy)]
pub struct MidElem {
    /// 拡大縮小させるコンテンツ。
    #[required]
    pub body: Content,
}

/// 式に床関数を作用させます。
///
/// ```example
/// $ floor(x/2) $
/// ```
#[func]
pub fn floor(
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 床関数を作用させる式。
    body: Content,
) -> Content {
    delimited(body, '⌊', '⌋', size)
}

/// 式に天井関数を作用させます。
///
/// ```example
/// $ ceil(x/2) $
/// ```
#[func]
pub fn ceil(
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 天井関数を作用させる式。
    body: Content,
) -> Content {
    delimited(body, '⌈', '⌉', size)
}

/// 式を丸めます。
///
/// ```example
/// $ round(x/2) $
/// ```
#[func]
pub fn round(
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 丸める式。
    body: Content,
) -> Content {
    delimited(body, '⌊', '⌉', size)
}

/// 式の絶対値を取ります。
///
/// ```example
/// $ abs(x/2) $
/// ```
#[func]
pub fn abs(
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 絶対値を取る式。
    body: Content,
) -> Content {
    delimited(body, '|', '|', size)
}

/// 式のノルムを取ります。
///
/// ```example
/// $ norm(x/2) $
/// ```
#[func]
pub fn norm(
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// ノルムを取る式。
    body: Content,
) -> Content {
    delimited(body, '‖', '‖', size)
}

fn delimited(
    body: Content,
    left: char,
    right: char,
    size: Option<Rel<Length>>,
) -> Content {
    let span = body.span();
    let mut elem = LrElem::new(Content::sequence([
        SymbolElem::packed(left),
        body,
        SymbolElem::packed(right),
    ]));
    // Push size only if size is provided
    if let Some(size) = size {
        elem.push_size(size);
    }
    elem.pack().spanned(span)
}
