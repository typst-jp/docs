use typst_syntax::Span;

use crate::foundations::{Content, Func, NativeElement, elem, func};
use crate::introspection::Locatable;

/// 現在の外側のコンテナ（存在しなければページ）の寸法（幅と高さ）へのアクセスを提供します。
///
/// `width`と`height`という[`length`]型のキーを持つ辞書を単一の引数として受け取る関数を受け付けます。
/// 関数には[context]が渡されるため、`context`キーワードと組み合わせて使用する必要はありません。
/// これが以下の例で[`measure`]が呼び出せる理由です。
///
/// ```example
/// #let text = lorem(30)
/// #layout(size => [
///   #let (height,) = measure(
///     width: size.width,
///     text,
///   )
///   This text is #height high with
///   the current page width: \
///   #text
/// ])
/// ```
///
/// `layout`関数はコンテンツに[ブロック]($block)レベルのコンテナであることを強制するため、そのコンテナ内部でページを基準とした配置や改ページはできないことに注意してください。
///
/// 例えば、幅が`{800pt}`で高さが`{400pt}`のボックスの中から`layout`が呼び出されたとすると、指定された関数には`{(width: 800pt, height: 400pt)}`という引数が与えられます。
/// ページに直接置かれた場合は、ページの寸法からそのマージンを引いたものを受け取ります。
/// これは主に[測定]($measure)と組み合わせるときに便利です。
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
/// この関数は、[`ratio`]を固定長に変換するためにも利用できます。
/// これは独自のレイアウト抽象化を構築する際に重宝するかもしれません。
///
/// ```example
/// #layout(size => {
///   let half = 50% * size.width
///   [Half a page is #half wide.]
/// })
/// ```
///
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
    func: Func,
) -> Content {
    LayoutElem::new(func).pack().spanned(span)
}

/// Executes a `layout` call.
#[elem(Locatable)]
pub struct LayoutElem {
    /// The function to call with the outer container's (or page's) size.
    #[required]
    pub func: Func,
}
