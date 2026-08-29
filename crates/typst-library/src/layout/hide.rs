use crate::foundations::{Content, elem};
use crate::introspection::Tagged;

/// レイアウトに影響を与えないコンテンツの隠蔽。
///
/// `hide`関数を用いると、レイアウトにコンテンツを「認識」させながらコンテンツを隠せます。
/// これは何らかのコンテンツと全く同じ大きさを持つ空白を作る際に便利です。
///
/// # 例
/// ```example
/// Hello Jane \
/// #hide[Hello] Joe
/// ```
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
    /// 隠したいコンテンツ。
    #[required]
    pub body: Content,

    /// This style is set on the content contained in the `hide` element.
    #[internal]
    #[ghost]
    pub hidden: bool,
}
