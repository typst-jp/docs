use crate::foundations::{Content, elem};
use crate::introspection::Tagged;
use crate::layout::Length;

/// 利用可能なスペースでのコンテンツの繰り返し。
///
/// これは独自の索引、参考文献、目次を作成する際に便利です。
///
/// bodyパラメーターの実体の間に空白が挿入される可能性があるため、[`justify`]($repeat.justify)パラメーターを正しく調整しているか確かめてください。
///
/// 利用可能なスペースに上限がない場合は、コンテンツを無限に生成してしまうためエラーになります。
///
/// # 例
/// ```example
/// Sign on the dotted line:
/// #box(width: 1fr, repeat[.])
///
/// #set text(10pt)
/// #v(8pt, weak: true)
/// #align(right)[
///   Berlin, the 22nd of December, 2022
/// ]
/// ```
///
/// # Accessibility
/// Repeated content is automatically marked as an [artifact]($pdf.artifact) and
/// hidden from Assistive Technology (AT). Do not use this function to create
/// content that contributes to the meaning of your document.
#[elem(Tagged)]
pub struct RepeatElem {
    /// 繰り返すコンテンツ。
    #[required]
    pub body: Content,

    /// 本文の実体間の間隔。
    #[default]
    pub gap: Length,

    /// 利用可能なスペースを完全に埋めるために、実体間の間隔を大きくするかどうか。
    #[default(true)]
    pub justify: bool,
}
