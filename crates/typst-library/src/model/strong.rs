use crate::foundations::{Content, elem};
use crate::introspection::{Locatable, Tagged};

<<<<<<< HEAD
/// フォントの太さを増やすことでコンテンツを強調します。
///
/// 現在のフォントの太さに指定した差分 `delta` を加えます。
///
/// # 例
=======
/// Strongly emphasizes content by increasing the font weight.
///
/// Increases the current font weight by a given `delta`.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// This is *strong.* \
/// This is #strong[too.] \
///
/// #show strong: set text(red)
/// And this is *evermore.*
/// ```
///
<<<<<<< HEAD
/// # 構文
/// この関数には専用の構文もあります。
/// 強調したいコンテンツをアスタリスク（`*`）で囲むだけです。
/// ただし、これは単語の区切りにおいてのみ機能します。
/// 単語の一部を強調したい場合は、関数を使用してください。
#[elem(title = "Strong Emphasis", keywords = ["bold", "weight"], Locatable, Tagged)]
pub struct StrongElem {
    /// フォントの太さに適用する変化量。
=======
/// # Syntax
/// This function also has dedicated syntax: To strongly emphasize content,
/// simply enclose it in stars/asterisks (`*`). Note that this only works at
/// word boundaries. To strongly emphasize part of a word, you have to use the
/// function.
#[elem(title = "Strong Emphasis", keywords = ["bold", "weight"], Locatable, Tagged)]
pub struct StrongElem {
    /// The delta to apply on the font weight.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set strong(delta: 0)
    /// No *effect!*
    /// ```
    #[default(300)]
    pub delta: i64,

<<<<<<< HEAD
    /// 強調するコンテンツ。
=======
    /// The content to strongly emphasize.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}
