//! HTML output.

mod dom;

pub use self::dom::*;

use ecow::EcoString;

use crate::foundations::{elem, Content, Module, Scope};

/// Create a module with all HTML definitions.
pub fn module() -> Module {
    let mut html = Scope::deduplicating();
    html.start_category(crate::Category::Html);
    html.define_elem::<HtmlElem>();
    html.define_elem::<FrameElem>();
    Module::new("html", html)
}

/// Typstのコンテンツを含むことができるHTML要素。
///
/// TypstのHTMLエクスポートは、ほとんどの要素に対して適切なタグを自動的に生成します。
/// ただし、場合によっては、より強いコントロールを手元に残しておくことが望ましい場合があります。
/// 例えば、Typstを使ってブログを生成する場合、
/// それぞれの記事を`<article>`タグでラップするためにこの関数を使用できます。
///
/// Typstは有効なHTMLが何であるかを認識しています。
/// タグとその属性は、構文的に有効なHTMLを構成していなければなりません。
/// `meta`のようないくつかのタグはコンテンツを受け付けません。
/// したがって、それらに対して本文を提供してはいけません。
/// 将来的に、この機能に対して更に多くのチェックを追加する可能性があるため、この関数を使用する際は有効なHTMLを生成していることを確認してください。
///
/// 通常、Typstは`html`、`head`、および`body`タグを生成します。
/// 代わりにこの関数でそれらを作成した場合、Typstは自身の生成するタグを省略します。
///
/// ```typ
/// #html.elem("div", attrs: (style: "background: aqua"))[
///   A div with _Typst content_ inside!
/// ]
/// ```
#[elem(name = "elem")]
pub struct HtmlElem {
    /// 要素のタグ。
    #[required]
    pub tag: HtmlTag,

    /// 要素のHTML属性。
    #[borrowed]
    pub attrs: HtmlAttrs,

    /// HTML要素の内容。
    ///
    /// 本文には任意のTypstコンテンツを指定できます。
    #[positional]
    #[borrowed]
    pub body: Option<Content>,
}

impl HtmlElem {
    /// Add an attribute to the element.
    pub fn with_attr(mut self, attr: HtmlAttr, value: impl Into<EcoString>) -> Self {
        self.attrs.get_or_insert_with(Default::default).push(attr, value);
        self
    }
}

/// An element that lays out its content as an inline SVG.
///
/// Sometimes, converting Typst content to HTML is not desirable. This can be
/// the case for plots and other content that relies on positioning and styling
/// to convey its message.
///
/// This function allows you to use the Typst layout engine that would also be
/// used for PDF, SVG, and PNG export to render a part of your document exactly
/// how it would appear when exported in one of these formats. It embeds the
/// content as an inline SVG.
#[elem]
pub struct FrameElem {
    /// The content that shall be laid out.
    #[positional]
    #[required]
    pub body: Content,
}
