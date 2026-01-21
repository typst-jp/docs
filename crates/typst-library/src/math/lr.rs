<<<<<<< HEAD
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
=======
use crate::foundations::{Content, NativeElement, SymbolElem, elem, func};
use crate::layout::{Length, Rel};
use crate::math::Mathy;

/// Scales delimiters.
///
/// While matched delimiters scale by default, this can be used to scale
/// unmatched delimiters and to control the delimiter scaling more precisely.
#[elem(title = "Left/Right", Mathy)]
pub struct LrElem {
    /// The size of the brackets, relative to the height of the wrapped content.
    #[default(Rel::one())]
    pub size: Rel<Length>,

    /// The delimited content, including the delimiters.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    #[parse(
        let mut arguments = args.all::<Content>()?.into_iter();
        let mut body = arguments.next().unwrap_or_default();
        arguments.for_each(|arg| body += SymbolElem::packed(',') + arg);
        body
    )]
    pub body: Content,
}

<<<<<<< HEAD
/// 最も近くで囲んでいる`{lr()}`グループに対して、垂直方向に区切り文字を拡大縮小します。
=======
/// Scales delimiters vertically to the nearest surrounding `{lr()}` group.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ { x mid(|) sum_(i=1)^n w_i|f_i (x)| < 1 } $
/// ```
#[elem(Mathy)]
pub struct MidElem {
<<<<<<< HEAD
    /// 拡大縮小させるコンテンツ。
=======
    /// The content to be scaled.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// 式に床関数を作用させます。
=======
/// Floors an expression.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ floor(x/2) $
/// ```
#[func]
pub fn floor(
<<<<<<< HEAD
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 床関数を作用させる式。
=======
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Rel<Length>>,
    /// The expression to floor.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    body: Content,
) -> Content {
    delimited(body, '⌊', '⌋', size)
}

<<<<<<< HEAD
/// 式に天井関数を作用させます。
=======
/// Ceils an expression.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ ceil(x/2) $
/// ```
#[func]
pub fn ceil(
<<<<<<< HEAD
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 天井関数を作用させる式。
=======
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Rel<Length>>,
    /// The expression to ceil.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    body: Content,
) -> Content {
    delimited(body, '⌈', '⌉', size)
}

<<<<<<< HEAD
/// 式を丸めます。
=======
/// Rounds an expression.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ round(x/2) $
/// ```
#[func]
pub fn round(
<<<<<<< HEAD
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 丸める式。
=======
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Rel<Length>>,
    /// The expression to round.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    body: Content,
) -> Content {
    delimited(body, '⌊', '⌉', size)
}

<<<<<<< HEAD
/// 式の絶対値を取ります。
=======
/// Takes the absolute value of an expression.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ abs(x/2) $
/// ```
#[func]
pub fn abs(
<<<<<<< HEAD
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// 絶対値を取る式。
=======
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Rel<Length>>,
    /// The expression to take the absolute value of.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    body: Content,
) -> Content {
    delimited(body, '|', '|', size)
}

<<<<<<< HEAD
/// 式のノルムを取ります。
=======
/// Takes the norm of an expression.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ norm(x/2) $
/// ```
#[func]
pub fn norm(
<<<<<<< HEAD
    /// ラップしたコンテンツの高さを基準とした括弧の大きさ。
    #[named]
    size: Option<Rel<Length>>,
    /// ノルムを取る式。
=======
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Rel<Length>>,
    /// The expression to take the norm of.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
        elem.push_size(size);
=======
        elem.size.set(size);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
    elem.pack().spanned(span)
}
