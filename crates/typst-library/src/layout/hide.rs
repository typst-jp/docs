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
/// # 墨消し
/// この関数は、引数が視覚的に表示されず、支援技術からもアクセスできないため、コンテンツを墨消しする際にも便利かもしれません。
/// ただし、隠したコンテンツの痕跡は*多少*残ることがあります（PDFの目次にブックマークとして表示される見出しなど）。
///
/// 状況によっては、レイアウト上のサイズからコンテンツの内容を推測できる可能性がある点に注意してください。
/// そのため、機密性の高い情報を隠す目的でこの関数を使用することはおすすめしません。
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
