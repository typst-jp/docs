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
/// ただし、より細かく制御したい場合もあります。
/// 例えば、Typstを使ってブログを生成する場合、
/// この関数を用いると、それぞれの記事を`<article>`タグで囲めます。
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

/// コンテンツをインラインSVGとしてレイアウトする要素。
///
/// TypstのコンテンツにはHTMLへの変換が不適切なものがあります。
/// グラフプロットや、意味を伝えるために位置決めやスタイルに依存するコンテンツが該当します。
///
/// この関数を使用すると、
/// PDF、SVG、およびPNGエクスポートにも使用されるTypstレイアウトエンジンを使用して、
/// 文書の一部を、これらの形式のいずれかでエクスポートした場合に表示されるのとまったく同じようにレンダリングできます。
/// この関数はコンテンツをインラインSVGとして埋め込みます。
#[elem]
pub struct FrameElem {
    ///レイアウト対象のコンテンツ。
    #[positional]
    #[required]
    pub body: Content,
}
