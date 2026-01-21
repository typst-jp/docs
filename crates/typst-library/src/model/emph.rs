<<<<<<< HEAD
use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
    elem, Content, NativeElement, Packed, Show, StyleChain, TargetElem,
};
use crate::html::{tag, HtmlElem};
use crate::text::{ItalicToggle, TextElem};

/// イタリック体への切り替えによるコンテンツの強調。
///
/// - 現在の[テキストスタイル]($text.style)が`{"normal"}`の場合、これを
///   `{"italic"}`に変更します。
/// - 現在のテキストスタイルが既に`{"italic"}`あるいは`{"oblique"}`の場合、
///   `{"normal"}`に戻します。
///
/// # 例
=======
use crate::foundations::{Content, elem};
use crate::introspection::{Locatable, Tagged};

/// Emphasizes content by toggling italics.
///
/// - If the current [text style]($text.style) is `{"normal"}`, this turns it
///   into `{"italic"}`.
/// - If it is already `{"italic"}` or `{"oblique"}`, it turns it back to
///   `{"normal"}`.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// # 構文
/// この関数には専用の構文もあります。
/// 強調したいコンテンツをアンダースコア（`_`）で囲むだけです。
/// ただし、これは単語の区切りにおいてのみ機能します。
/// 単語の一部を強調したい場合は、関数を使用してください。
#[elem(title = "Emphasis", keywords = ["italic"], Show)]
pub struct EmphElem {
    /// 強調するコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<EmphElem> {
    #[typst_macros::time(name = "emph", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let body = self.body.clone();
        Ok(if TargetElem::target_in(styles).is_html() {
            HtmlElem::new(tag::em)
                .with_body(Some(body))
                .pack()
                .spanned(self.span())
        } else {
            body.styled(TextElem::set_emph(ItalicToggle(true)))
        })
    }
}
=======
/// # Syntax
/// This function also has dedicated syntax: To emphasize content, simply
/// enclose it in underscores (`_`). Note that this only works at word
/// boundaries. To emphasize part of a word, you have to use the function.
#[elem(title = "Emphasis", keywords = ["italic"], Locatable, Tagged)]
pub struct EmphElem {
    /// The content to emphasize.
    #[required]
    pub body: Content,
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
