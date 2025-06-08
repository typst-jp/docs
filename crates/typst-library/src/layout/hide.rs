use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, Packed, Show, StyleChain};

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
#[elem(Show)]
pub struct HideElem {
    /// 隠したいコンテンツ。
    #[required]
    pub body: Content,

    /// This style is set on the content contained in the `hide` element.
    #[internal]
    #[ghost]
    pub hidden: bool,
}

impl Show for Packed<HideElem> {
    #[typst_macros::time(name = "hide", span = self.span())]
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone().styled(HideElem::set_hidden(true)))
    }
}
