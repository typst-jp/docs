use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, Packed, Show, StyleChain, Value};
use crate::introspection::Locatable;

/// 可視コンテンツの生成を伴わないクエリシステムへの値の公開。
///
/// この要素は[`query`]関数や[`typst query`]($reference/introspection/query/#command-line-queries)を用いてコマンドラインから取得できます。
/// その目的は任意の値を内省システムに公開することです。
/// メタデータの値を他と識別するために、[`label`]を付けて、それを検索することができます。
///
/// `metadata`要素は、外部に任意の値を公開できるため、特にコマンドラインクエリで便利です。
///
/// ```example
/// // Put metadata somewhere.
/// #metadata("This is a note") <note>
///
/// // And find it from anywhere else.
/// #context {
///   query(<note>).first().value
/// }
/// ```
#[elem(Show, Locatable)]
pub struct MetadataElem {
    /// 文書に埋め込む値。
    #[required]
    pub value: Value,
}

impl Show for Packed<MetadataElem> {
    fn show(&self, _: &mut Engine, _styles: StyleChain) -> SourceResult<Content> {
        Ok(Content::empty())
    }
}
