use ecow::EcoString;

use crate::diag::{bail, HintedStrResult, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Args, Array, Construct, Content, Datetime, Fields, OneOrMultiple, Smart,
    StyleChain, Styles, Value,
};

/// 文書とそのメタデータのルート要素。
///
/// すべての文書は、自動的に`document`（文書）要素でラップされます。
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
    /// これはPDFビューアーのウィンドウタイトルとして表示されることが多いです。
    ///
    /// これはコンテンツで指定可能ですが、PDFビューアーがプレーンテキストのタイトルしかサポートしないために、変換時に情報が失われる可能性があります。
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
}

impl DocumentInfo {
    /// Populate this document info with details from the given styles.
    ///
    /// Document set rules are a bit special, so we need to do this manually.
    pub fn populate(&mut self, styles: &Styles) {
        let chain = StyleChain::new(styles);
        let has = |field| styles.has::<DocumentElem>(field as _);
        if has(<DocumentElem as Fields>::Enum::Title) {
            self.title =
                DocumentElem::title_in(chain).map(|content| content.plain_text());
        }
        if has(<DocumentElem as Fields>::Enum::Author) {
            self.author = DocumentElem::author_in(chain).0;
        }
        if has(<DocumentElem as Fields>::Enum::Description) {
            self.description =
                DocumentElem::description_in(chain).map(|content| content.plain_text());
        }
        if has(<DocumentElem as Fields>::Enum::Keywords) {
            self.keywords = DocumentElem::keywords_in(chain).0;
        }
        if has(<DocumentElem as Fields>::Enum::Date) {
            self.date = DocumentElem::date_in(chain);
        }
    }
}
