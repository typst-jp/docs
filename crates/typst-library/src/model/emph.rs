use crate::foundations::{Content, elem};
use crate::introspection::{Locatable, Tagged};

/// イタリック体への切り替えによるコンテンツの強調。
///
/// - 現在の[テキストスタイル]($text.style)が`{"normal"}`の場合、これを
///   `{"italic"}`に変更します。
/// - 現在のテキストスタイルが既に`{"italic"}`あるいは`{"oblique"}`の場合、
///   `{"normal"}`に戻します。
///
/// # 例
/// ```example
/// This is _emphasized._ \
/// This is #emph[too.]
///
/// #show emph: it => {
///   text(blue, it.body)
/// }
///
/// This is _emphasized_ differently.
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
/// 強調したいコンテンツをアンダースコア（`_`）で囲むだけです。
/// ただし、これは単語の区切りにおいてのみ機能します。
/// 単語の一部を強調したい場合は、関数を使用してください。
#[elem(title = "Emphasis", keywords = ["italic"], Locatable, Tagged)]
pub struct EmphElem {
    /// 強調するコンテンツ。
    #[required]
    pub body: Content,
}
