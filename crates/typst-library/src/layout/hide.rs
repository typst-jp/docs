use crate::foundations::{Content, elem};
use crate::introspection::Tagged;

<<<<<<< HEAD
/// レイアウトに影響を与えないコンテンツの隠蔽。
///
/// `hide`関数を用いると、レイアウトにコンテンツを「認識」させながらコンテンツを隠せます。
/// これは何らかのコンテンツと全く同じ大きさを持つ空白を作る際に便利です。
/// 引数が出力に含まれないため、コンテンツを削除する際にも便利かもしれません。
///
/// # 例
=======
/// Hides content without affecting layout.
///
/// The `hide` function allows you to hide content while the layout still "sees"
/// it. This is useful for creating blank space that is exactly as large as some
/// content.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// Hello Jane \
/// #hide[Hello] Joe
/// ```
<<<<<<< HEAD
#[elem(Tagged)]
pub struct HideElem {
    /// 隠したいコンテンツ。
=======
///
/// # Redaction
/// This function may also be useful for redacting content as its arguments are
/// neither present visually nor accessible to Assistive Technology. That said,
/// there can be _some_ traces of the hidden content (such as a bookmarked
/// heading in the PDF's Document Outline).
///
/// Note that, depending on the circumstances, it may be possible for content to
/// be reverse engineered based on its size in the layout. We thus do not
/// recommend using this function to hide highly sensitive information.
#[elem(Tagged)]
pub struct HideElem {
    /// The content to hide.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,

    /// This style is set on the content contained in the `hide` element.
    #[internal]
    #[ghost]
    pub hidden: bool,
}
