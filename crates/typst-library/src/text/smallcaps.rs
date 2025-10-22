use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, Packed, Show, StyleChain};
use crate::text::TextElem;

/// スモールキャピタルでテキストを表示。
///
/// # 例
/// ```example
/// Hello \
/// #smallcaps[Hello]
/// ```
///
/// # スモールキャピタルのフォント
/// デフォルトでは、この関数はフォントのOpenTypeフィーチャーの`smcp`および`c2sc`を使用します。
/// 全てのフォントがこれらのフィーチャーをサポートしているわけではありません。
/// スモールキャピタルは専用のフォントとして提供されることがあります。
/// この例として _Latin Modern_ フォントファミリーが該当します。
/// この場合、show-setルールを用いてスモールキャピタルでのテキストの見た目がカスタマイズできます。
///
/// ```typ
/// #show smallcaps: set text(font: "Latin Modern Roman Caps")
/// ```
///
/// 将来的に、この関数は標準サイズの文字からスモールキャピタルの文字を合成することをサポートする予定ですが、まだ実装されていません。
///
/// # スモールキャピタルの見出し
/// [showルール]($styling/#show-rules)を用いて見出し全てにスモールキャピタルを適用できます。
/// 以下の例では、見出しを中央揃えにし、通常の太字フォントの無効化も行っています。
///
/// ```example
/// #set par(justify: true)
/// #set heading(numbering: "I.")
///
/// #show heading: smallcaps
/// #show heading: set align(center)
/// #show heading: set text(
///   weight: "regular"
/// )
///
/// = Introduction
/// #lorem(40)
/// ```
#[elem(title = "Small Capitals", Show)]
pub struct SmallcapsElem {
    /// 大文字も同様にスモールキャピタルに変更するかどうか。
    ///
    /// showルールで上書きされない限り、これはOpenTypeフィーチャーの`c2sc`を有効化します。
    ///
    /// ```example
    /// #smallcaps(all: true)[UNICEF] is an
    /// agency of #smallcaps(all: true)[UN].
    /// ```
    #[default(false)]
    pub all: bool,
    /// スモールキャピタルで表示するコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<SmallcapsElem> {
    #[typst_macros::time(name = "smallcaps", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let sc = if self.all(styles) { Smallcaps::All } else { Smallcaps::Minuscules };
        Ok(self.body.clone().styled(TextElem::set_smallcaps(Some(sc))))
    }
}

/// What becomes small capitals.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Smallcaps {
    /// Minuscules become small capitals.
    Minuscules,
    /// All letters become small capitals.
    All,
}
