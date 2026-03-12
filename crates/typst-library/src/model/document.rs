use ecow::EcoString;

use crate::diag::{HintedStrResult, SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Args, Array, Construct, Content, Datetime, OneOrMultiple, Smart, StyleChain, Styles,
    Value, cast, elem,
};
use crate::text::{Locale, TextElem};

/// 文書とそのメタデータのルート要素。
///
/// 全ての文書は、自動的に`document`（文書）要素でラップされます。
/// この文書要素は自分で作成することはできません。
/// この関数は、[setルール]($styling/#set-rules)と組み合わせて文書のメタデータを指定する場合にのみ使用されます。
/// setルールは、レイアウトコンテナの内部に置いてはいけません。
///
/// ```example
/// #set document(title: [Hello])
///
/// This has no visible output, but
/// embeds metadata into the PDF!
/// ```
///
/// この関数で設定したメタデータは、文書内には表示されません。
/// 代わりに、コンパイルされたPDFファイル内に埋め込まれます。
#[elem(Construct)]
pub struct DocumentElem {
    /// 文書のタイトル。
    /// これはPDFビューアーのウィンドウタイトルやブラウザタブに表示されます。
    ///
    /// タイトルの設定はアクセシビリティ上重要であり、他の文書の中から識別しやすくなります。
    /// PDF/UAへのエクスポート時にはタイトルが必須です。
    ///
    /// これはコンテンツで指定可能ですが、PDFビューアーがプレーンテキストのタイトルしかサポートしないため、
    /// 変換時に情報が失われる可能性があります。
    #[ghost]
    pub title: Option<Content>,

    /// 文書の著者。
    #[ghost]
    pub author: OneOrMultiple<EcoString>,

    /// 文書の説明。
    #[ghost]
    pub description: Option<Content>,

    /// 文書のキーワード。
    #[ghost]
    pub keywords: OneOrMultiple<EcoString>,

    /// 文書の作成日。
    ///
    /// これを`{auto}`（デフォルト設定）とすると、Typstは現在の日時を使用します。
    /// `{none}`とすると、PDFメタデータに作成日時を埋め込まなくなります。
    ///
    /// PDFに埋め込むためには、yearの値が0以上でなくてはなりません。
    ///
    /// バイト単位で同一に再現できるPDFを出力したい場合には、`{auto}`以外の値を設定してください。
    #[ghost]
    pub date: Smart<Option<Datetime>>,
}

impl Construct for DocumentElem {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "can only be used in set rules")
    }
}

/// A list of authors.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Author(Vec<EcoString>);

cast! {
    Author,
    self => self.0.into_value(),
    v: EcoString => Self(vec![v]),
    v: Array => Self(v.into_iter().map(Value::cast).collect::<HintedStrResult<_>>()?),
}

/// A list of keywords.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Keywords(Vec<EcoString>);

cast! {
    Keywords,
    self => self.0.into_value(),
    v: EcoString => Self(vec![v]),
    v: Array => Self(v.into_iter().map(Value::cast).collect::<HintedStrResult<_>>()?),
}

/// Details about the document.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct DocumentInfo {
    /// The document's title.
    pub title: Option<EcoString>,
    /// The document's author(s).
    pub author: Vec<EcoString>,
    /// The document's description.
    pub description: Option<EcoString>,
    /// The document's keywords.
    pub keywords: Vec<EcoString>,
    /// The document's creation date.
    pub date: Smart<Option<Datetime>>,
    /// The document's language, set from the first top-level set rule, e.g.
    ///
    /// ```typc
    /// set text(lang: "...", region: "...")
    /// ```
    pub locale: Smart<Locale>,
}

impl DocumentInfo {
    /// Populate this document info with details from the given styles.
    ///
    /// Document set rules are a bit special, so we need to do this manually.
    pub fn populate(&mut self, styles: &Styles) {
        let chain = StyleChain::new(styles);
        if styles.has(DocumentElem::title) {
            self.title = chain
                .get_ref(DocumentElem::title)
                .as_ref()
                .map(|content| content.plain_text());
        }
        if styles.has(DocumentElem::author) {
            self.author = chain.get_cloned(DocumentElem::author).0;
        }
        if styles.has(DocumentElem::description) {
            self.description = chain
                .get_ref(DocumentElem::description)
                .as_ref()
                .map(|content| content.plain_text());
        }
        if styles.has(DocumentElem::keywords) {
            self.keywords = chain.get_cloned(DocumentElem::keywords).0;
        }
        if styles.has(DocumentElem::date) {
            self.date = chain.get(DocumentElem::date);
        }
    }

    /// Populate this document info with locale details from the given styles.
    pub fn populate_locale(&mut self, styles: &Styles) {
        if self.locale.is_custom() {
            return;
        }

        let chain = StyleChain::new(styles);
        let mut locale: Option<Locale> = None;
        if styles.has(TextElem::lang) {
            locale.get_or_insert_default().lang = chain.get(TextElem::lang);
        }
        if styles.has(TextElem::region) {
            locale.get_or_insert_default().region = chain.get(TextElem::region);
        }
        self.locale = Smart::from(locale);
    }
}
