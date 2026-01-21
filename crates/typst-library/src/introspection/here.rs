use comemo::Tracked;

use crate::diag::HintedStrResult;
<<<<<<< HEAD
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
=======
use crate::foundations::{Context, func};
use crate::introspection::Location;

/// Provides the current location in the document.
///
/// You can think of `here` as a low-level building block that directly extracts
/// the current location from the active [context]. Some other functions use it
/// internally: For instance, `{counter.get()}` is equivalent to
/// `{counter.at(here())}`.
///
/// Within show rules on [locatable]($location/#locatable) elements, `{here()}`
/// will match the location of the shown element.
///
/// If you want to display the current page number, refer to the documentation
/// of the [`counter`] type. While `here` can be used to determine the physical
/// page number, typically you want the logical page number that may, for
/// instance, have been reset after a preface.
///
/// # Examples
/// Determining the current position in the document in combination with the
/// [`position`]($location.position) method:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #context [
///   I am located at
///   #here().position()
/// ]
/// ```
///
<<<<<<< HEAD
/// 現在位置より前にある要素に対して[クエリ]($query)を実行します。
=======
/// Running a [query] for elements before the current position:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// セレクターのbeforeおよびafterに関する詳細は[`selector`]型のドキュメントを参照してください。
=======
/// Refer to the [`selector`] type for more details on before/after selectors.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#[func(contextual)]
pub fn here(context: Tracked<Context>) -> HintedStrResult<Location> {
    context.location()
}
