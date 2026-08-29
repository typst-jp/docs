use crate::foundations::{Value, elem};
use crate::introspection::Locatable;

<<<<<<< HEAD
/// 可視コンテンツの生成を伴わないクエリシステムへの値の公開。
///
/// この要素は[`query`]関数や[`typst query`]($reference/introspection/query/#command-line-queries)を用いてコマンドラインから取得できます。
/// その目的は任意の値を内省システムに公開することです。
/// メタデータの値を他と識別するために、[`label`]を付けて、それを検索できます。
///
/// `metadata`要素は、外部に任意の値を公開できるため、特にコマンドラインクエリで便利です。
=======
/// Exposes a value to the query system without producing visible content.
///
/// This element can be retrieved with the [`query`] function and from the
/// command line with
/// [`typst query`]($reference/introspection/query/#command-line-queries). Its
/// purpose is to expose an arbitrary value to the introspection system. To
/// identify a metadata value among others, you can attach a [`label`] to it and
/// query for that label.
///
/// The `metadata` element is especially useful for command line queries because
/// it allows you to expose arbitrary values to the outside world.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
#[elem(Locatable)]
pub struct MetadataElem {
<<<<<<< HEAD
    /// 文書に埋め込む値。
=======
    /// The value to embed into the document.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub value: Value,
}
