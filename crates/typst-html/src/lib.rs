//! Typst's HTML exporter.

mod attr;
mod charsets;
mod convert;
mod css;
mod document;
mod dom;
mod encode;
mod fragment;
mod link;
mod rules;
mod tag;
mod typed;

pub use self::document::html_document;
pub use self::dom::*;
pub use self::encode::html;
pub use self::rules::{html_span_filled, register};

use ecow::EcoString;
use typst_library::Category;
use typst_library::foundations::{Content, Module, Scope};
use typst_library::introspection::Location;
use typst_macros::elem;

/// Creates the module with all HTML definitions.
pub fn module() -> Module {
    let mut html = Scope::deduplicating();
    html.start_category(Category::Html);
    html.define_elem::<HtmlElem>();
    html.define_elem::<FrameElem>();
    crate::typed::define(&mut html);
    Module::new("html", html)
}

<<<<<<< HEAD
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
/// 将来的に、この機能に対してさらに多くのチェックを追加する可能性があるため、
/// この関数を使用する際は有効なHTMLを生成していることを確認してください。
///
/// 通常、Typstは`html`、`head`、および`body`タグを生成します。
/// 代わりにこの関数でそれらを作成した場合、Typstは自身の生成するタグを省略します。
=======
/// An HTML element that can contain Typst content.
///
/// Typst's HTML export automatically generates the appropriate tags for most
/// elements. However, sometimes, it is desirable to retain more control. For
/// example, when using Typst to generate your blog, you could use this function
/// to wrap each article in an `<article>` tag.
///
/// Typst is aware of what is valid HTML. A tag and its attributes must form
/// syntactically valid HTML. Some tags, like `meta` do not accept content.
/// Hence, you must not provide a body for them. We may add more checks in the
/// future, so be sure that you are generating valid HTML when using this
/// function.
///
/// Normally, Typst will generate `html`, `head`, and `body` tags for you. If
/// you instead create them with this function, Typst will omit its own tags.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```typ
/// #html.elem("div", attrs: (style: "background: aqua"))[
///   A div with _Typst content_ inside!
/// ]
/// ```
#[elem(name = "elem")]
pub struct HtmlElem {
<<<<<<< HEAD
    /// 要素のタグ。
    #[required]
    pub tag: HtmlTag,

    /// 要素のHTML属性。
    pub attrs: HtmlAttrs,

    /// HTML要素の内容。
    ///
    /// 本文には任意のTypstコンテンツを指定できます。
=======
    /// The element's tag.
    #[required]
    pub tag: HtmlTag,

    /// The element's HTML attributes.
    pub attrs: HtmlAttrs,

    /// The contents of the HTML element.
    ///
    /// The body can be arbitrary Typst content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    pub body: Option<Content>,

    /// The element's logical parent, if any.
    #[internal]
    #[synthesized]
    pub parent: Location,

<<<<<<< HEAD
    /// 役割は、最上位のスタイル設定されたHTML要素に適用されるべきですが、
    /// その子孫には適用されません。スタイルセットルールが今後追加されて、
    /// サブツリーではなく特定の要素に適用される場合は、
    /// これを置き換えることができます。`class`のような他のことについて
    /// 同じメカニズムが必要な場合は、これは潜在的に任意の属性にも拡張できます。
    /// 現在は最小限です。
    ///
    /// これは`<p>`要素に対して無視されます。
    /// そうしないと、要素をグループ化した結果として生じる段落に
    /// 意図せずに付与されてしまう傾向があります。
    /// これは多少のハックですが、`role`プロパティは純粋に内部的なものであり、
    /// その使用方法は私たちが制御しているため、十分です。
=======
    /// A role that should be applied to the top-level styled HTML element, but
    /// not its descendants. If we ever get set rules that apply to a specific
    /// element instead of a subtree, they could supplant this. If we need the
    /// same mechanism for things like `class`, this could potentially also be
    /// extended to arbitrary attributes. It's minimal for now.
    ///
    /// This is ignored for `<p>` elements as it otherwise tends to
    /// unintentionally attach to paragraphs resulting from grouping of a single
    /// element instead of attaching to that element. This is a bit of a hack,
    /// but good enough for now as the `role` property is purely internal and
    /// we control what it is used for.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[internal]
    #[ghost]
    pub role: Option<EcoString>,
}

impl HtmlElem {
    /// Add an attribute to the element.
    pub fn with_attr(mut self, attr: HtmlAttr, value: impl Into<EcoString>) -> Self {
        self.attrs
            .as_option_mut()
            .get_or_insert_with(Default::default)
            .push(attr, value);
        self
    }

    /// Adds the attribute to the element if value is not `None`.
    pub fn with_optional_attr(
        self,
        attr: HtmlAttr,
        value: Option<impl Into<EcoString>>,
    ) -> Self {
        if let Some(value) = value { self.with_attr(attr, value) } else { self }
    }

    /// Adds CSS styles to an element.
    fn with_styles(self, properties: css::Properties) -> Self {
        if let Some(value) = properties.into_inline_styles() {
            self.with_attr(attr::style, value)
        } else {
            self
        }
    }

    /// Checks whether the given element is an inline-level HTML element.
    fn is_inline(elem: &Content) -> bool {
        elem.to_packed::<HtmlElem>()
            .is_some_and(|elem| tag::is_inline_by_default(elem.tag))
    }
}

<<<<<<< HEAD
/// コンテンツをインラインSVGとしてレイアウトする要素。
///
/// TypstのコンテンツにはHTMLへの変換に不適切なものがあります。
/// グラフプロットや、意味を伝えるために位置決めやスタイルに依存するコンテンツが該当します。
///
/// この関数を使用すると、
/// PDF、SVG、およびPNGエクスポートにも使用されるTypstレイアウトエンジンを使用して、
/// 文書の一部を、これらの形式のいずれかでエクスポートした場合に表示されるのとまったく同じようにレンダリングできます。
/// この関数はコンテンツをインラインSVGとして埋め込みます。
#[elem]
pub struct FrameElem {
    /// レイアウト対象のコンテンツ。
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[positional]
    #[required]
    pub body: Content,
}
