use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Args, AutoValue, Construct, Content, NativeElement, Packed, Smart,
    StyleChain, Value,
};
use crate::introspection::Locator;
use crate::layout::{
    Abs, Corners, Em, Fr, Fragment, Frame, Length, Region, Regions, Rel, Sides, Size,
    Spacing,
};
use crate::visualize::{Paint, Stroke};

/// コンテンツの大きさを持つインラインレベルのコンテナ。
///
/// インライン数式、テキスト、ボックスを除く全ての要素はブロックレベルであり、[段落]($par)の中に含めることはできません。
/// box関数を用いることで、そのような要素を段落にまとめることができます。
/// ボックスはデフォルトで、受け取ったコンテンツに合わせた大きさになりますが、明示的に大きさを指定することもできます。
///
/// # 例
/// ```example
/// Refer to the docs
/// #box(
///   height: 9pt,
///   image("docs.svg")
/// )
/// for more information.
/// ```
#[elem]
pub struct BoxElem {
    /// ボックスの幅。
    ///
    /// ボックスは以下の例で示すように、[比率]($fraction)を用いて幅を指定できます。
    ///
    /// _注意:_ 現在、パラグラフ内で比率指定が可能なのはボックスおよびその幅のみです。
    /// 比率で指定した大きさを持つ画像や図形などは今後サポートされる可能性があります。
    ///
    /// ```example
    /// Line in #box(width: 1fr, line(length: 100%)) between.
    /// ```
    pub width: Sizing,

    /// ボックスの高さ。
    pub height: Smart<Rel<Length>>,

    /// ボックスのベースラインをシフトさせる量。
    ///
    /// ```example
    /// Image: #box(baseline: 40%, image("tiger.jpg", width: 2cm)).
    /// ```
    #[resolve]
    pub baseline: Rel<Length>,

    /// ボックスの背景色。
    /// 詳細は[rectangleのドキュメント]($rect.fill)を参照してください。
    pub fill: Option<Paint>,

    /// ボックスの枠線の色。
    /// 詳細は[rectangleのドキュメント]($rect.stroke)を参照してください。
    #[resolve]
    #[fold]
    pub stroke: Sides<Option<Option<Stroke>>>,

    /// ボックスの角の丸めの大きさ。
    /// 詳細は[rectangleのドキュメント]($rect.radius)を参照してください。
    #[resolve]
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,

    /// ボックスのコンテンツのパディング量。
    ///
    /// _注意:_ ボックスがテキストを含むとき、その正確な大きさは現在の[テキストの端]($text.top-edge)に依存します。
    ///
    /// ```example
    /// #rect(inset: 0pt)[Tight]
    /// ```
    #[resolve]
    #[fold]
    pub inset: Sides<Option<Rel<Length>>>,

    /// レイアウトに影響を与えずにボックスの大きさを拡大する量。
    ///
    /// これはパディングが行のレイアウトに影響を与えるのを防ぐために便利です。
    /// 以下の例より一般的な場合については、[未加工テキストのblockパラメーター]($raw.block)のドキュメントを参照してください。
    ///
    /// ```example
    /// An inline
    /// #box(
    ///   fill: luma(235),
    ///   inset: (x: 3pt, y: 0pt),
    ///   outset: (y: 3pt),
    ///   radius: 2pt,
    /// )[rectangle].
    /// ```
    #[resolve]
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// ボックスの内側のコンテンツのクリッピングを行うか否か。
    /// クリッピングは、ボックスの境界を超えたコンテンツを隠すため、ボックスのコンテンツがボックス本体よりも大きい場合に便利です。
    ///
    /// ```example
    /// #box(
    ///   width: 50pt,
    ///   height: 50pt,
    ///   clip: true,
    ///   image("tiger.jpg", width: 100pt, height: 100pt)
    /// )
    /// ```
    #[default(false)]
    pub clip: bool,

    /// ボックスのコンテンツ。
    #[positional]
    #[borrowed]
    pub body: Option<Content>,
}

/// An inline-level container that can produce arbitrary items that can break
/// across lines.
#[elem(Construct)]
pub struct InlineElem {
    /// A callback that is invoked with the regions to produce arbitrary
    /// inline items.
    #[required]
    #[internal]
    body: callbacks::InlineCallback,
}

impl Construct for InlineElem {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

impl InlineElem {
    /// Create an inline-level item with a custom layouter.
    #[allow(clippy::type_complexity)]
    pub fn layouter<T: NativeElement>(
        captured: Packed<T>,
        callback: fn(
            content: &Packed<T>,
            engine: &mut Engine,
            locator: Locator,
            styles: StyleChain,
            region: Size,
        ) -> SourceResult<Vec<InlineItem>>,
    ) -> Self {
        Self::new(callbacks::InlineCallback::new(captured, callback))
    }
}

impl Packed<InlineElem> {
    /// Layout the element.
    pub fn layout(
        &self,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Size,
    ) -> SourceResult<Vec<InlineItem>> {
        self.body.call(engine, locator, styles, region)
    }
}

/// Layouted items suitable for placing in a paragraph.
#[derive(Debug, Clone)]
pub enum InlineItem {
    /// Absolute spacing between other items, and whether it is weak.
    Space(Abs, bool),
    /// Layouted inline-level content.
    Frame(Frame),
}

/// ブロックレベルのコンテナ。
///
/// このようなコンテナは、コンテンツを区切り、その大きさを調整し、背景や枠線を付与するために使用できます。
///
/// ブロックは、テキストが段落の一部となるかどうかを制御する主要な方法でもあります。
/// 詳細は[段落のドキュメント]($par/#what-becomes-a-paragraph)を参照してください。
///
/// # 例
/// ブロックを使用すると、複数のページに渡って分割されるコンテンツに背景を与えることができます。
/// ```example
/// #set page(height: 100pt)
/// #block(
///   fill: luma(230),
///   inset: 8pt,
///   radius: 4pt,
///   lorem(30),
/// )
/// ```
///
/// ブロックは、特にshowルールを記述する際、本来インラインとなる要素を強制的にブロックレベルとして扱う場合にも有用です。
/// ```example
/// #show heading: it => it.body
/// = Blockless
/// More text.
///
/// #show heading: it => block(it.body)
/// = Blocky
/// More text.
/// ```
#[elem]
pub struct BlockElem {
    /// ブロックの幅。
    ///
    /// ```example
    /// #set align(center)
    /// #block(
    ///   width: 60%,
    ///   inset: 8pt,
    ///   fill: silver,
    ///   lorem(10),
    /// )
    /// ```
    pub width: Smart<Rel<Length>>,

    /// ブロックの高さ。
    /// 高さがページに残された余白より大きく、[`breakable`]($block.breakable)が`{true}`の場合、 ブロックは残りの高さで次のページに続きます。
    ///
    /// ```example
    /// #set page(height: 80pt)
    /// #set align(center)
    /// #block(
    ///   width: 80%,
    ///   height: 150%,
    ///   fill: aqua,
    /// )
    /// ```
    pub height: Sizing,

    /// ブロックが分割可能で次のページに継続するかどうか。
    ///
    /// ```example
    /// #set page(height: 80pt)
    /// The following block will
    /// jump to its own page.
    /// #block(
    ///   breakable: false,
    ///   lorem(15),
    /// )
    /// ```
    #[default(true)]
    pub breakable: bool,

    /// ブロックの背景色。
    /// 詳細は[rectangleのドキュメント]($rect.fill)を参照してください。
    pub fill: Option<Paint>,

    /// ブロックの枠線の色。
    /// 詳細は[rectangleのドキュメント]($rect.stroke)を参照してください。
    #[resolve]
    #[fold]
    pub stroke: Sides<Option<Option<Stroke>>>,

    /// ブロックの角の丸めの大きさ。
    /// 詳細は[rectangleのドキュメント]($rect.radius)を参照してください。
    #[resolve]
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,

    /// ブロックのコンテンツのパディング量。
    /// 詳細は[boxのドキュメント]($box.inset)を参照してください。
    #[resolve]
    #[fold]
    pub inset: Sides<Option<Rel<Length>>>,

    /// レイアウトに影響を与えずにブロックの大きさを拡大する量。
    /// 詳細は[boxのドキュメント]($box.outset)を参照してください。
    #[resolve]
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// ブロック周りの間隔。`{auto}`の場合、段落の[`spacing`]($par.spacing)を継承します。
    ///
    /// 隣接する2つのブロックについては、最初のブロックの`above`と2番目のブロックの`below`のうち、 大きい方の間隔が優先されます。
    /// また、ブロックの間隔は段落の[`spacing`]($par.spacing)よりも優先されます。
    ///
    /// これは`above`と`below`を同じ値に設定するための短縮記法にすぎないことに注意してください。
    /// `above`と`below`の値は異なる可能性があるため、[context]ブロックでは`{block.above}`と `{block.below}`にのみアクセスでき、`{block.spacing}`に直接アクセスすることはできません。
    ///
    /// このプロパティはshowルールと組み合わせて使用することで、任意のブロックレベル要素の周りの間隔を調整できます。
    ///
    /// ```example
    /// #set align(center)
    /// #show math.equation: set block(above: 8pt, below: 16pt)
    ///
    /// This sum of $x$ and $y$:
    /// $ x + y = z $
    /// A second paragraph.
    /// ```
    #[external]
    #[default(Em::new(1.2).into())]
    pub spacing: Spacing,

    /// このブロックとその前のブロックとの間隔。
    #[parse(
        let spacing = args.named("spacing")?;
        args.named("above")?.or(spacing)
    )]
    pub above: Smart<Spacing>,

    /// このブロックとその後のブロックとの間隔。
    #[parse(args.named("below")?.or(spacing))]
    pub below: Smart<Spacing>,

    /// ブロックの内側のコンテンツのクリッピングを行うか否か。
    ///
    /// クリッピングは、ブロックの境界を超えたコンテンツを隠すため、ブロックのコンテンツがブロック本体よりも大きい場合に便利です。
    ///
    /// ```example
    /// #block(
    ///   width: 50pt,
    ///   height: 50pt,
    ///   clip: true,
    ///   image("tiger.jpg", width: 100pt, height: 100pt)
    /// )
    /// ```
    #[default(false)]
    pub clip: bool,

    /// このブロックが、次のブロックとの間に区切りを入れることなく続ける必要があるかどうか。
    ///
    /// この設定は、ページの下部で見出しが孤立することを防ぐために、見出しブロックに対してデフォルトで適用されています。
    ///
    /// ```example
    /// >>> #set page(height: 140pt)
    /// // Disable stickiness of headings.
    /// #show heading: set block(sticky: false)
    /// #lorem(20)
    ///
    /// = Chapter
    /// #lorem(10)
    /// ```
    #[default(false)]
    pub sticky: bool,

    /// ブロックのコンテンツ。
    #[positional]
    #[borrowed]
    pub body: Option<BlockBody>,
}

impl BlockElem {
    /// Create a block with a custom single-region layouter.
    ///
    /// Such a block must have `breakable: false` (which is set by this
    /// constructor).
    pub fn single_layouter<T: NativeElement>(
        captured: Packed<T>,
        f: fn(
            content: &Packed<T>,
            engine: &mut Engine,
            locator: Locator,
            styles: StyleChain,
            region: Region,
        ) -> SourceResult<Frame>,
    ) -> Self {
        Self::new()
            .with_breakable(false)
            .with_body(Some(BlockBody::SingleLayouter(
                callbacks::BlockSingleCallback::new(captured, f),
            )))
    }

    /// Create a block with a custom multi-region layouter.
    pub fn multi_layouter<T: NativeElement>(
        captured: Packed<T>,
        f: fn(
            content: &Packed<T>,
            engine: &mut Engine,
            locator: Locator,
            styles: StyleChain,
            regions: Regions,
        ) -> SourceResult<Fragment>,
    ) -> Self {
        Self::new().with_body(Some(BlockBody::MultiLayouter(
            callbacks::BlockMultiCallback::new(captured, f),
        )))
    }
}

/// The contents of a block.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum BlockBody {
    /// The block contains normal content.
    Content(Content),
    /// The block contains a layout callback that needs access to just one
    /// base region.
    SingleLayouter(callbacks::BlockSingleCallback),
    /// The block contains a layout callback that needs access to the exact
    /// regions.
    MultiLayouter(callbacks::BlockMultiCallback),
}

impl Default for BlockBody {
    fn default() -> Self {
        Self::Content(Content::default())
    }
}

cast! {
    BlockBody,
    self => match self {
        Self::Content(content) => content.into_value(),
        _ => Value::Auto,
    },
    v: Content => Self::Content(v),
}

/// Defines how to size something along an axis.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Sizing {
    /// A track that fits its item's contents.
    Auto,
    /// A size specified in absolute terms and relative to the parent's size.
    Rel(Rel),
    /// A size specified as a fraction of the remaining free space in the
    /// parent.
    Fr(Fr),
}

impl Sizing {
    /// Whether this is an automatic sizing.
    pub fn is_auto(self) -> bool {
        matches!(self, Self::Auto)
    }

    /// Whether this is fractional sizing.
    pub fn is_fractional(self) -> bool {
        matches!(self, Self::Fr(_))
    }
}

impl Default for Sizing {
    fn default() -> Self {
        Self::Auto
    }
}

impl From<Smart<Rel>> for Sizing {
    fn from(smart: Smart<Rel>) -> Self {
        match smart {
            Smart::Auto => Self::Auto,
            Smart::Custom(rel) => Self::Rel(rel),
        }
    }
}

impl<T: Into<Spacing>> From<T> for Sizing {
    fn from(spacing: T) -> Self {
        match spacing.into() {
            Spacing::Rel(rel) => Self::Rel(rel),
            Spacing::Fr(fr) => Self::Fr(fr),
        }
    }
}

cast! {
    Sizing,
    self => match self {
        Self::Auto => Value::Auto,
        Self::Rel(rel) => rel.into_value(),
        Self::Fr(fr) => fr.into_value(),
    },
    _: AutoValue => Self::Auto,
    v: Rel<Length> => Self::Rel(v),
    v: Fr => Self::Fr(v),
}

/// Manual closure implementations for layout callbacks.
///
/// Normal closures are not `Hash`, so we can't use them.
mod callbacks {
    use super::*;

    macro_rules! callback {
        ($name:ident = ($($param:ident: $param_ty:ty),* $(,)?) -> $ret:ty) => {
            #[derive(Debug, Clone, PartialEq, Hash)]
            pub struct $name {
                captured: Content,
                f: fn(&Content, $($param_ty),*) -> $ret,
            }

            impl $name {
                pub fn new<T: NativeElement>(
                    captured: Packed<T>,
                    f: fn(&Packed<T>, $($param_ty),*) -> $ret,
                ) -> Self {
                    Self {
                        // Type-erased the content.
                        captured: captured.pack(),
                        // Safety: The only difference between the two function
                        // pointer types is the type of the first parameter,
                        // which changes from `&Packed<T>` to `&Content`. This
                        // is safe because:
                        // - `Packed<T>` is a transparent wrapper around
                        //   `Content`, so for any `T` it has the same memory
                        //   representation as `Content`.
                        // - While `Packed<T>` imposes the additional constraint
                        //   that the content is of type `T`, this constraint is
                        //   upheld: It is initially the case because we store a
                        //   `Packed<T>` above. It keeps being the case over the
                        //   lifetime of the closure because `capture` is a
                        //   private field and `Content`'s `Clone` impl is
                        //   guaranteed to retain the type (if it didn't,
                        //   literally everything would break).
                        #[allow(clippy::missing_transmute_annotations)]
                        f: unsafe { std::mem::transmute(f) },
                    }
                }

                pub fn call(&self, $($param: $param_ty),*) -> $ret {
                    (self.f)(&self.captured, $($param),*)
                }
            }
        };
    }

    callback! {
        InlineCallback = (
            engine: &mut Engine,
            locator: Locator,
            styles: StyleChain,
            region: Size,
        ) -> SourceResult<Vec<InlineItem>>
    }

    callback! {
        BlockSingleCallback = (
            engine: &mut Engine,
            locator: Locator,
            styles: StyleChain,
            region: Region,
        ) -> SourceResult<Frame>
    }

    callback! {
        BlockMultiCallback = (
            engine: &mut Engine,
            locator: Locator,
            styles: StyleChain,
            regions: Regions,
        ) -> SourceResult<Fragment>
    }
}
