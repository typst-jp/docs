use crate::foundations::{Content, elem};
use crate::layout::{Length, Rel};

<<<<<<< HEAD
/// コンテンツの周囲に空白を追加。
///
/// 空白は各辺を独立に指定するか、位置変数を用いて全辺を一括指定できます。
///
/// # 例
=======
/// Adds spacing around content.
///
/// The spacing can be specified for each side individually, or for all sides at
/// once by specifying a positional argument.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// #set align(center)
///
/// #pad(x: 16pt, image("typing.jpg"))
/// _Typing speeds can be
///  measured in words per minute._
/// ```
#[elem(title = "Padding")]
pub struct PadElem {
<<<<<<< HEAD
    /// 左辺のパディング。
=======
    /// The padding at the left side.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[parse(
        let all = args.named("rest")?.or(args.find()?);
        let x = args.named("x")?.or(all);
        let y = args.named("y")?.or(all);
        args.named("left")?.or(x)
    )]
    pub left: Rel<Length>,

<<<<<<< HEAD
    /// 上辺のパディング。
    #[parse(args.named("top")?.or(y))]
    pub top: Rel<Length>,

    /// 右辺のパディング。
    #[parse(args.named("right")?.or(x))]
    pub right: Rel<Length>,

    /// 下辺のパディング。
    #[parse(args.named("bottom")?.or(y))]
    pub bottom: Rel<Length>,

    /// `left`と`right`を同じ値で設定するための省略記法。
    #[external]
    pub x: Rel<Length>,

    /// `top`と`bottom`を同じ値で設定するための省略記法。
    #[external]
    pub y: Rel<Length>,

    /// 四辺全てを同じ値で設定するための省略記法。
    #[external]
    pub rest: Rel<Length>,

    /// パディングを追加するコンテンツ。
=======
    /// The padding at the top side.
    #[parse(args.named("top")?.or(y))]
    pub top: Rel<Length>,

    /// The padding at the right side.
    #[parse(args.named("right")?.or(x))]
    pub right: Rel<Length>,

    /// The padding at the bottom side.
    #[parse(args.named("bottom")?.or(y))]
    pub bottom: Rel<Length>,

    /// A shorthand to set `left` and `right` to the same value.
    #[external]
    pub x: Rel<Length>,

    /// A shorthand to set `top` and `bottom` to the same value.
    #[external]
    pub y: Rel<Length>,

    /// A shorthand to set all four sides to the same value.
    #[external]
    pub rest: Rel<Length>,

    /// The content to pad at the sides.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}
