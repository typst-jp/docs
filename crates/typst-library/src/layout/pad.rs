<<<<<<< HEAD
use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, NativeElement, Packed, Show, StyleChain};
use crate::layout::{BlockElem, Length, Rel};

/// コンテンツの周囲に空白を追加。
///
/// 空白は各辺を独立に指定するか、位置変数を用いて全辺を一括指定できます。
///
/// # 例
=======
use crate::foundations::{Content, elem};
use crate::layout::{Length, Rel};

/// Adds spacing around content.
///
/// The spacing can be specified for each side individually, or for all sides at
/// once by specifying a positional argument.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #set align(center)
///
/// #pad(x: 16pt, image("typing.jpg"))
/// _Typing speeds can be
///  measured in words per minute._
/// ```
<<<<<<< HEAD
#[elem(title = "Padding", Show)]
pub struct PadElem {
    /// 左辺のパディング。
=======
#[elem(title = "Padding")]
pub struct PadElem {
    /// The padding at the left side.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
    #[required]
    pub body: Content,
}

impl Show for Packed<PadElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::multi_layouter(self.clone(), engine.routines.layout_pad)
            .pack()
            .spanned(self.span()))
    }
}
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
    #[required]
    pub body: Content,
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
