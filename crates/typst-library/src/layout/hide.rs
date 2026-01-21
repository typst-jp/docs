<<<<<<< HEAD
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
=======
use crate::foundations::{Content, elem};
use crate::introspection::Tagged;

/// Hides content without affecting layout.
///
/// The `hide` function allows you to hide content while the layout still 'sees'
/// it. This is useful to create whitespace that is exactly as large as some
/// content. It may also be useful to redact content because its arguments are
/// not included in the output.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// Hello Jane \
/// #hide[Hello] Joe
/// ```
<<<<<<< HEAD
#[elem(Show)]
pub struct HideElem {
    /// 隠したいコンテンツ。
=======
#[elem(Tagged)]
pub struct HideElem {
    /// The content to hide.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,

    /// This style is set on the content contained in the `hide` element.
    #[internal]
    #[ghost]
    pub hidden: bool,
}
<<<<<<< HEAD

impl Show for Packed<HideElem> {
    #[typst_macros::time(name = "hide", span = self.span())]
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone().styled(HideElem::set_hidden(true)))
    }
}
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
