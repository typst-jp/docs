<<<<<<< HEAD
use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
    elem, Content, NativeElement, Packed, Show, StyleChain, TargetElem,
};
use crate::html::{tag, HtmlElem};
use crate::text::{TextElem, WeightDelta};

/// 太字によるコンテンツの強調。
///
/// 現在のフォントの太さに指定した差分 `delta` を加えます。
///
/// # 例
=======
use crate::foundations::{Content, elem};
use crate::introspection::{Locatable, Tagged};

/// Strongly emphasizes content by increasing the font weight.
///
/// Increases the current font weight by a given `delta`.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
#[elem(title = "Strong Emphasis", keywords = ["bold", "weight"], Show)]
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set strong(delta: 0)
    /// No *effect!*
    /// ```
    #[default(300)]
    pub delta: i64,

<<<<<<< HEAD
    /// 強調するコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<StrongElem> {
    #[typst_macros::time(name = "strong", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let body = self.body.clone();
        Ok(if TargetElem::target_in(styles).is_html() {
            HtmlElem::new(tag::strong)
                .with_body(Some(body))
                .pack()
                .spanned(self.span())
        } else {
            body.styled(TextElem::set_delta(WeightDelta(self.delta(styles))))
        })
    }
}
=======
    /// The content to strongly emphasize.
    #[required]
    pub body: Content,
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
