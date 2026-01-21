<<<<<<< HEAD
use comemo::Track;
use typst_syntax::Span;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
    dict, elem, func, Content, Context, Func, NativeElement, Packed, Show, StyleChain,
};
use crate::introspection::Locatable;
use crate::layout::{BlockElem, Size};

/// 現在の外側のコンテナ（存在しなければページ）の寸法（幅と高さ）へのアクセスを提供します。
///
/// `width`と`height`という[`length`]型のキーを持つ辞書を単一の引数として受け取る関数を受け付けます。
/// 関数には[context]が渡されるため、`context`キーワードと組み合わせて使用する必要はありません。
/// これが以下の例で[`measure`]が呼び出せる理由です。
=======
use typst_syntax::Span;

use crate::foundations::{Content, Func, NativeElement, elem, func};
use crate::introspection::Locatable;

/// Provides access to the current outer container's (or page's, if none)
/// dimensions (width and height).
///
/// Accepts a function that receives a single parameter, which is a dictionary
/// with keys `width` and `height`, both of type [`length`]. The function is
/// provided [context], meaning you don't need to use it in combination with the
/// `context` keyword. This is why [`measure`] can be called in the example
/// below.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #let text = lorem(30)
/// #layout(size => [
///   #let (height,) = measure(
<<<<<<< HEAD
///     block(width: size.width, text),
=======
///     width: size.width,
///     text,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///   )
///   This text is #height high with
///   the current page width: \
///   #text
/// ])
/// ```
///
<<<<<<< HEAD
/// `layout`関数はコンテンツに[ブロック]($block)レベルのコンテナであることを強制するため、そのコンテナ内部でページを基準とした配置や改ページはできないことに注意してください。
///
/// 例えば、幅が`{800pt}`で高さが`{400pt}`のボックスの中から`layout`が呼び出されたとすると、指定された関数には`{(width: 800pt, height: 400pt)}`という引数が与えられます。
/// ページに直接置かれた場合は、ページの寸法からそのマージンを引いたものを受け取ります。
/// これは主に[測定]($measure)と組み合わせるときに便利です。
///
/// この関数は、[ratio]を固定長に変換するためにも利用できます。
/// これは独自のレイアウト抽象化を構築する際に重宝するかもしれません。
=======
/// Note that the `layout` function forces its contents into a [block]-level
/// container, so placement relative to the page or pagebreaks are not possible
/// within it.
///
/// If the `layout` call is placed inside a box with a width of `{800pt}` and a
/// height of `{400pt}`, then the specified function will be given the argument
/// `{(width: 800pt, height: 400pt)}`. If it is placed directly into the page, it
/// receives the page's dimensions minus its margins. This is mostly useful in
/// combination with [measurement]($measure).
///
/// To retrieve the _remaining_ height of the page rather than its full size,
/// you can wrap your `layout` call in a `{block(height: 1fr)}`. This works
/// because the block automatically grows to fill the remaining space (see the
/// [fraction] documentation for more details).
///
/// ```example
/// #set page(height: 150pt)
///
/// #lorem(20)
///
/// #block(height: 1fr, layout(size => [
///   Remaining height: #size.height
/// ]))
/// ```
///
/// You can also use this function to resolve a [`ratio`] to a fixed length.
/// This might come in handy if you're building your own layout abstractions.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #layout(size => {
///   let half = 50% * size.width
///   [Half a page is #half wide.]
/// })
/// ```
///
<<<<<<< HEAD
/// `layout`が提供する幅と高さは、対象ページの寸法が`{auto}`に設定されていると無限大になりうることに注意してください。
#[func]
pub fn layout(
    span: Span,
    /// 外側のコンテナの大きさと一緒に呼び出す関数。
    /// 戻り値は文書に表示されます。
    ///
    /// コンテナの大きさは`width`と`height`のキーを持つ[dictionary]として与えられます。
    ///
    /// この関数は、`layout` によって返されるコンテンツが文書中に現れるたびに呼び出されます。
    /// これによりそのコンテナの寸法に依存するコンテンツの生成が可能になります。
=======
/// Note that the width or height provided by `layout` will be infinite if the
/// corresponding page dimension is set to `{auto}`.
#[func]
pub fn layout(
    span: Span,
    /// A function to call with the outer container's size. Its return value is
    /// displayed in the document.
    ///
    /// The container's size is given as a [dictionary] with the keys `width`
    /// and `height`.
    ///
    /// This function is called once for each time the content returned by
    /// `layout` appears in the document. This makes it possible to generate
    /// content that depends on the dimensions of its container.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    func: Func,
) -> Content {
    LayoutElem::new(func).pack().spanned(span)
}

/// Executes a `layout` call.
<<<<<<< HEAD
#[elem(Locatable, Show)]
struct LayoutElem {
    /// The function to call with the outer container's (or page's) size.
    #[required]
    func: Func,
}

impl Show for Packed<LayoutElem> {
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::multi_layouter(
            self.clone(),
            |elem, engine, locator, styles, regions| {
                // Gets the current region's base size, which will be the size of the
                // outer container, or of the page if there is no such container.
                let Size { x, y } = regions.base();
                let loc = elem.location().unwrap();
                let context = Context::new(Some(loc), Some(styles));
                let result = elem
                    .func
                    .call(
                        engine,
                        context.track(),
                        [dict! { "width" => x, "height" => y }],
                    )?
                    .display();
                (engine.routines.layout_fragment)(
                    engine, &result, locator, styles, regions,
                )
            },
        )
        .pack()
        .spanned(self.span()))
    }
=======
#[elem(Locatable)]
pub struct LayoutElem {
    /// The function to call with the outer container's (or page's) size.
    #[required]
    pub func: Func,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}
