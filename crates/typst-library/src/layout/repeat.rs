use crate::foundations::{Content, elem};
use crate::introspection::Tagged;
use crate::layout::Length;

<<<<<<< HEAD
/// 利用可能なスペースでのコンテンツの繰り返し。
///
/// これは独自の索引、参考文献、目次を作成する際に便利です。
///
/// bodyパラメーターの実体の間に空白が挿入され得るため、[`justify`]($repeat.justify)パラメーターを正しく調整しているか確かめてください。
///
/// 利用可能なスペースに上限がない場合は、コンテンツを無限に生成してしまうためエラーになります。
///
/// # 例
=======
/// Repeats content to the available space.
///
/// This can be useful when implementing a custom index, reference, or outline.
///
/// Space may be inserted between the instances of the body parameter, so be
/// sure to adjust the [`justify`]($repeat.justify) parameter accordingly.
///
/// Errors if there are no bounds on the available space, as it would create
/// infinite content.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
<<<<<<< HEAD
/// # アクセシビリティ
/// 繰り返されたコンテンツは自動的に[アーティファクト]($pdf.artifact)としてマークされ、支援技術（AT）からは認識されません。
/// そのため、文書の意味にかかわるコンテンツの作成には、この関数を使用しないでください。
#[elem(Tagged)]
pub struct RepeatElem {
    /// 繰り返すコンテンツ。
    #[required]
    pub body: Content,

    /// 本文の実体間の間隔。
    #[default]
    pub gap: Length,

    /// 利用可能なスペースを完全に埋めるために、実体間の間隔を大きくするかどうか。
=======
/// # Accessibility
/// Repeated content is automatically marked as an [artifact]($pdf.artifact) and
/// hidden from Assistive Technology (AT). Do not use this function to create
/// content that contributes to the meaning of your document.
#[elem(Tagged)]
pub struct RepeatElem {
    /// The content to repeat.
    #[required]
    pub body: Content,

    /// The gap between each instance of the body.
    #[default]
    pub gap: Length,

    /// Whether to increase the gap between instances to completely fill the
    /// available space.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[default(true)]
    pub justify: bool,
}
