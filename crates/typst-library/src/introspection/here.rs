use comemo::Tracked;

use crate::diag::HintedStrResult;
use crate::foundations::{func, Context};
use crate::introspection::Location;

/// 文書中における現在のロケーションを提供。
///
/// `here`はアクティブな[コンテキスト]($context)から現在のロケーションを直接取得する低レベルな構成要素と考えることができます。
/// いくつかの他の関数が内部で使用しています。
/// 例えば、`{counter.get()}`は`{counter.at(here())}`と等価です。
///
/// [ロケータブル]($location/#locatable)要素に対するshowルールにおいて、`{here()}`は表示する要素のロケーションにマッチします。
///
/// 現在のページ番号を表示したい場合は、[`counter`]型のドキュメントを参照してください。
/// `here`は物理的なページ番号を決定できますが、通常は、前書きの後にリセットされるような、論理的なページ番号が必要でしょう。
///
/// # 例
/// [`position`]($location.position)メソッドと組み合わせて文書中での現在位置を決定します。
/// ```example
/// #context [
///   I am located at
///   #here().position()
/// ]
/// ```
///
/// 現在位置より前にある要素に対して[クエリ]($query)を実行します。
/// ```example
/// = Introduction
/// = Background
///
/// There are
/// #context query(
///   selector(heading).before(here())
/// ).len()
/// headings before me.
///
/// = Conclusion
/// ```
/// セレクターのbeforeおよびafterに関する詳細は[`selector`]型のドキュメントを参照してください。
#[func(contextual)]
pub fn here(context: Tracked<Context>) -> HintedStrResult<Location> {
    context.location()
}
