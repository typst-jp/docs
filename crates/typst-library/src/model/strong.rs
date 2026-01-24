use crate::foundations::{Content, elem};
use crate::introspection::{Locatable, Tagged};

/// フォントの太さを増やすことでコンテンツを強調します。
///
/// 現在のフォントの太さに指定した差分 `delta` を加えます。
///
/// # 例
/// ```example
/// This is *strong.* \
/// This is #strong[too.] \
///
/// #show strong: set text(red)
/// And this is *evermore.*
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
/// 強調したいコンテンツをアスタリスク（`*`）で囲むだけです。
/// ただし、これは単語の区切りにおいてのみ機能します。
/// 単語の一部を強調したい場合は、関数を使用してください。
#[elem(title = "Strong Emphasis", keywords = ["bold", "weight"], Locatable, Tagged)]
pub struct StrongElem {
    /// フォントの太さに適用する変化量。
    ///
    /// ```example
    /// #set strong(delta: 0)
    /// No *effect!*
    /// ```
    #[default(300)]
    pub delta: i64,

    /// 強調するコンテンツ。
    #[required]
    pub body: Content,
}
