use comemo::Tracked;

use crate::diag::HintedStrResult;
use crate::engine::Engine;
use crate::foundations::{func, Context, LocatableSelector};
use crate::introspection::Location;

/// 文書中の要素のロケーションを特定。
///
/// 厳密に1つだけの要素にマッチしなければならないセレクターを受け取り、要素の[`location`]を返します。
/// このlocationを用いると、特に、物理的な[`page`]($location.page)番号やその要素の[`position`]($location.position)（ページ番号、x座標、y座標）を取得できます。
///
/// # 例
/// 特定の要素のロケーションを特定します。
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
    /// 厳密に1つだけの要素にマッチしなければならないセレクター。
    /// その要素のロケーションが決定されます。
    ///
    /// 以下との組み合わせが特に便利です。
    /// - [`here`]と組み合わせた現在のコンテキストでのロケーションの特定
    /// - 何らかのクエリで得られた要素からコンテンツの[`location()`]($content.location)メソッドを用いて取得した[`location`]
    selector: LocatableSelector,
) -> HintedStrResult<Location> {
    selector.resolve_unique(engine.introspector, context)
}
