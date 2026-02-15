use crate::foundations::{Content, elem};
use crate::introspection::Tagged;

/// レイアウトに影響を与えないコンテンツの隠蔽。
///
/// `hide`関数を用いると、レイアウトにコンテンツを「認識」させながらコンテンツを隠すことができます。
/// これは何らかのコンテンツと全く同じ大きさを持つ空白を作る際に便利です。
/// 引数が出力に含まれないため、コンテンツを削除する際にも便利かもしれません。
///
/// # 例
/// ```example
/// Hello Jane \
/// #hide[Hello] Joe
/// ```
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
