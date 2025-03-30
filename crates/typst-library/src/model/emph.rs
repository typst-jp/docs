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
/// コンテンツを強調するには、単にアンダースコア（`_`）で囲みます。ただし、これは単語の区切りにおいてのみ機能します。
/// 単語の一部を強調する場合は、この関数を使用する必要があります。
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
