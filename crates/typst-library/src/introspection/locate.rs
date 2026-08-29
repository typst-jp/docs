use comemo::Tracked;

use crate::diag::HintedStrResult;
use crate::engine::Engine;
use crate::foundations::{Context, LocatableSelector, func};
use crate::introspection::Location;

<<<<<<< HEAD
/// 文書中の要素のロケーションを特定。
///
/// 厳密に1つだけの要素にマッチしなければならないセレクターを受け取り、要素の[`location`]を返します。
/// このlocationを用いると、特に、物理的な[`page`]($location.page)番号やその要素の[`position`]($location.position)（ページ番号、x座標、y座標）を取得できます。
///
/// # 例
/// 特定の要素のロケーションを特定します。
=======
/// Determines the location of an element in the document.
///
/// Takes a selector that must match exactly one element and returns that
/// element's [`location`]. This location can, in particular, be used to
/// retrieve the physical [`page`]($location.page) number and
/// [`position`]($location.position) (page, x, y) for that element.
///
/// # Examples
/// Locating a specific element:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// #context [
///   Introduction is at: \
///   #locate(<intro>).position()
/// ]
///
/// = Introduction <intro>
/// ```
#[func(contextual)]
pub fn locate(
    engine: &mut Engine,
    context: Tracked<Context>,
<<<<<<< HEAD
    /// 厳密に1つだけの要素にマッチしなければならないセレクター。
    /// その要素のロケーションが決定されます。
    ///
    /// 以下との組み合わせが特に便利です。
    /// - [`here`]と組み合わせた現在のコンテキストでのロケーションの特定
    /// - 何らかのクエリで得られた要素からコンテンツの[`location()`]($content.location)メソッドを用いて取得した[`location`]
=======
    /// A selector that should match exactly one element. This element will be
    /// located.
    ///
    /// Especially useful in combination with
    /// - [`here`] to locate the current context,
    /// - a [`location`] retrieved from some queried element via the
    ///   [`location()`]($content.location) method on content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    selector: LocatableSelector,
) -> HintedStrResult<Location> {
    selector.resolve_unique(engine.introspector, context)
}
