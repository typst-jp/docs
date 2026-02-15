use crate::foundations::{Content, elem};
use crate::layout::{Length, Rel};

/// コンテンツの周囲に空白を追加。
///
/// 空白は各辺を独立に指定するか、位置変数を用いて全辺を一括指定できます。
///
/// # 例
/// ```example
/// #set align(center)
///
/// #pad(x: 16pt, image("typing.jpg"))
/// _Typing speeds can be
///  measured in words per minute._
/// ```
#[elem(title = "Padding")]
pub struct PadElem {
    /// 左辺のパディング。
    #[parse(
        let all = args.named("rest")?.or(args.find()?);
        let x = args.named("x")?.or(all);
        let y = args.named("y")?.or(all);
        args.named("left")?.or(x)
    )]
    pub left: Rel<Length>,

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
    #[required]
    pub body: Content,
}
