use crate::foundations::{Content, elem};

<<<<<<< HEAD
/// スモールキャピタルでテキストを表示。
///
/// # 例
=======
/// Displays text in small capitals.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// Hello \
/// #smallcaps[Hello]
/// ```
///
<<<<<<< HEAD
/// # スモールキャピタルのフォント
/// デフォルトでは、この関数はフォントのOpenTypeフィーチャーの`smcp`および`c2sc`を使用します。
/// 全てのフォントがこれらのフィーチャーをサポートしているわけではありません。
/// スモールキャピタルは専用のフォントとして提供されることがあります。
/// この例として_Latin Modern_フォントファミリーが該当します。
/// この場合、show-setルールを用いてスモールキャピタルでのテキストの見た目がカスタマイズできます。
=======
/// # Smallcaps fonts
/// By default, this uses the `smcp` and `c2sc` OpenType features on the font.
/// Not all fonts support these features. Sometimes, smallcaps are part of a
/// dedicated font. This is, for example, the case for the _Latin Modern_ family
/// of fonts. In those cases, you can use a show-set rule to customize the
/// appearance of the text in smallcaps:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```typ
/// #show smallcaps: set text(font: "Latin Modern Roman Caps")
/// ```
///
<<<<<<< HEAD
/// 将来的に、この関数は標準サイズの文字からスモールキャピタルの文字を合成することをサポートする予定ですが、まだ実装されていません。
///
/// # スモールキャピタルの見出し
/// [showルール]($styling/#show-rules)を用いて見出し全てにスモールキャピタルを適用できます。
/// 以下の例では、見出しを中央揃えにし、通常の太字フォントの無効化も行っています。
=======
/// In the future, this function will support synthesizing smallcaps from normal
/// letters, but this is not yet implemented.
///
/// # Smallcaps headings
/// You can use a [show rule]($styling/#show-rules) to apply smallcaps
/// formatting to all your headings. In the example below, we also center-align
/// our headings and disable the standard bold font.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
#[elem(title = "Small Capitals")]
pub struct SmallcapsElem {
<<<<<<< HEAD
    /// 大文字も同様にスモールキャピタルに変更するかどうか。
    ///
    /// showルールで上書きされない限り、これはOpenTypeフィーチャーの`c2sc`を有効化します。
=======
    /// Whether to turn uppercase letters into small capitals as well.
    ///
    /// Unless overridden by a show rule, this enables the `c2sc` OpenType
    /// feature.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #smallcaps(all: true)[UNICEF] is an
    /// agency of #smallcaps(all: true)[UN].
    /// ```
    #[default(false)]
    pub all: bool,
<<<<<<< HEAD
    /// スモールキャピタルで表示するコンテンツ。
=======
    /// The content to display in small capitals.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

/// What becomes small capitals.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Smallcaps {
    /// Minuscules become small capitals.
    Minuscules,
    /// All letters become small capitals.
    All,
}
