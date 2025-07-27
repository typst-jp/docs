use comemo::Tracked;
use typst_syntax::Span;

use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{dict, func, Content, Context, Dict, Resolve, Smart};
use crate::introspection::{Locator, LocatorLink};
use crate::layout::{Abs, Axes, Length, Region, Size};

/// レイアウトされたコンテンツの測定。
///
/// `measure`関数を用いるとレイアウトされたコンテンツの大きさを測定できます。
/// デフォルトでは無限のスペースが想定されているため、測定された寸法は必ずしもコンテンツの最終的な寸法に一致するとは限りません。
/// 現在のレイアウトの寸法が測定したい場合は、`measure`と[`layout`]を組み合わせることができます。
///
/// # 例
/// 同じコンテンツでも置く場所の[context]によって異なる大きさになることがあります。
/// 以下の例では、フォントサイズを大きくすると`[#content]`は必然的に大きくなります。
///
/// ```example
/// #let content = [Hello!]
/// #content
/// #set text(14pt)
/// #content
/// ```
///
/// この理由から、測定が可能なのはコンテキストが利用可能な場合に限ります。
///
/// ```example
/// #let thing(body) = context {
///   let size = measure(body)
///   [Width of "#body" is #size.width]
/// }
///
/// #thing[Hey] \
/// #thing[Welcome]
/// ```
///
/// measure関数はキーが`width`と`height`で、その値がいずれも[`length`]型の辞書を返します。
#[func(contextual)]
pub fn measure(
    engine: &mut Engine,
    context: Tracked<Context>,
    span: Span,
    /// コンテンツをレイアウトするのに利用可能な幅。
    ///
    /// これを`{auto}`に設定すると無限大の幅が利用可能であると見なされます。
    ///
    /// この関数の`width`および`height`パラメーターを用いることは、大きさを持ち、コンテンツを有する[`block`]を測定することとは異なることに注意してください。
    /// 以下の例では、前者はブロックの寸法ではなく、内側のコンテンツの寸法を取得します。
    ///
    /// ```example
    /// #context measure(lorem(100), width: 400pt)
    ///
    /// #context measure(block(lorem(100), width: 400pt))
    /// ```
    #[named]
    #[default(Smart::Auto)]
    width: Smart<Length>,
    /// コンテンツをレイアウトするのに利用可能な高さ。
    ///
    /// これを`{auto}`に設定すると無限大の高さが利用可能であると見なされます。
    #[named]
    #[default(Smart::Auto)]
    height: Smart<Length>,
    /// 大きさを測定するコンテンツ。
    content: Content,
) -> SourceResult<Dict> {
    // Create a pod region with the available space.
    let styles = context.styles().at(span)?;
    let pod = Region::new(
        Axes::new(
            width.resolve(styles).unwrap_or(Abs::inf()),
            height.resolve(styles).unwrap_or(Abs::inf()),
        ),
        Axes::splat(false),
    );

    // We put the locator into a special "measurement mode" to ensure that
    // introspection-driven features within the content continue to work. Read
    // the "Dealing with measurement" section of the [`Locator`] docs for more
    // details.
    let here = context.location().at(span)?;
    let link = LocatorLink::measure(here);
    let locator = Locator::link(&link);

    let frame = (engine.routines.layout_frame)(engine, &content, locator, styles, pod)?;
    let Size { x, y } = frame.size();
    Ok(dict! { "width" => x, "height" => y })
}
