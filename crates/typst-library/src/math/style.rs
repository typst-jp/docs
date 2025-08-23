use crate::foundations::{func, Cast, Content, Smart};
use crate::math::EquationElem;

/// 数式中の太字フォントスタイル。
///
/// ```example
/// $ bold(A) := B^+ $
/// ```
#[func(keywords = ["mathbf"])]
pub fn bold(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_bold(true))
}

/// 数式中の立体（非斜体）フォントスタイル。
///
/// ```example
/// $ upright(A) != A $
/// ```
#[func(keywords = ["mathup"])]
pub fn upright(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_italic(Smart::Custom(false)))
}

/// 数式中の斜体フォントスタイル。
///
/// これがローマ字とギリシャ文字の小文字のデフォルトです。
#[func(keywords = ["mathit"])]
pub fn italic(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_italic(Smart::Custom(true)))
}

/// 数式中のセリフ（ローマン）フォントスタイル。
///
/// これがデフォルトです。
#[func(keywords = ["mathrm"])]
pub fn serif(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Serif))
}

/// 数式中のサンセリフフォントスタイル。
///
/// ```example
/// $ sans(A B C) $
/// ```
#[func(title = "Sans Serif", keywords = ["mathsf"])]
pub fn sans(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Sans))
}

/// 数式中のカリグラフィーフォントスタイル。
///
/// ```example
/// Let $cal(P)$ be the set of ...
/// ```
///
/// このスタイルはLaTeXの`\mathcal`と`\mathscr`の両方に対応します。
/// これは両スタイルが同じUnicodeのコードポイントを共有しているためです。
/// このため、スタイル間の切り替えは[フォントフィーチャー]($text.features)を用いてサポートされているフォントでのみ可能です。
///
/// デフォルトの数式フォントでは、ラウンドハンドスタイル（丸みを帯びた筆記体）が`ss01`フィーチャーとして利用可能です。
/// したがって、以下のように独自の`\mathscr`が定義できます。
///
/// ```example
/// #let scr(it) = text(
///   features: ("ss01",),
///   box($cal(it)$),
/// )
///
/// We establish $cal(P) != scr(P)$.
/// ```
///
/// （ボックスは概念的には不要ですが、現在のTypstの数式テキストスタイル処理の制約により必要です）
#[func(title = "Calligraphic", keywords = ["mathcal", "mathscr"])]
pub fn cal(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Cal))
}

/// 数式中のフラクトゥールフォントスタイル。
///
/// ```example
/// $ frak(P) $
/// ```
#[func(title = "Fraktur", keywords = ["mathfrak"])]
pub fn frak(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Frak))
}

/// 数式中の等幅フォントスタイル。
///
/// ```example
/// $ mono(x + y = z) $
/// ```
#[func(title = "Monospace", keywords = ["mathtt"])]
pub fn mono(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Mono))
}

/// 数式中の黒板太字（double-struck）フォントスタイル。
///
/// 大文字のラテン文字では、黒板太字は、[symbols]($category/symbols/sym)にあるように、`NN`や`RR`のような形式でも使用できます。
///
/// ```example
/// $ bb(b) $
/// $ bb(N) = NN $
/// $ f: NN -> RR $
/// ```
#[func(title = "Blackboard Bold", keywords = ["mathbb"])]
pub fn bb(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Bb))
}

/// 数式中でディスプレイスタイルを強制します。
///
/// これはブロック数式における標準サイズです。

/// ```example
/// $sum_i x_i/2 = display(sum_i x_i/2)$
/// ```
#[func(title = "Display Size", keywords = ["displaystyle"])]
pub fn display(
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
    #[named]
    #[default(false)]
    cramped: bool,
) -> Content {
    body.styled(EquationElem::set_size(MathSize::Display))
        .styled(EquationElem::set_cramped(cramped))
}

/// 数式中でインライン（テキスト）スタイルを強制します。
///
/// これはインライン数式における標準サイズです。
///
/// ```example
/// $ sum_i x_i/2
///     = inline(sum_i x_i/2) $
/// ```
#[func(title = "Inline Size", keywords = ["textstyle"])]
pub fn inline(
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
    #[named]
    #[default(false)]
    cramped: bool,
) -> Content {
    body.styled(EquationElem::set_size(MathSize::Text))
        .styled(EquationElem::set_cramped(cramped))
}

/// 数式中でスクリプトスタイルを強制します。
///
/// これは、冪乗、下付き文字、上付き文字で使用される小さいサイズです。
///
/// ```example
/// $sum_i x_i/2 = script(sum_i x_i/2)$
/// ```
#[func(title = "Script Size", keywords = ["scriptstyle"])]
pub fn script(
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
    #[named]
    #[default(true)]
    cramped: bool,
) -> Content {
    body.styled(EquationElem::set_size(MathSize::Script))
        .styled(EquationElem::set_cramped(cramped))
}

/// 数式中で第2スクリプトスタイルを強制します。
///
/// これは、第2レベルの下付き文字や上付き文字（添え字の添え字）で使用される最も小さいサイズです。
///
/// ```example
/// $sum_i x_i/2 = sscript(sum_i x_i/2)$
/// ```
#[func(title = "Script-Script Size", keywords = ["scriptscriptstyle"])]
pub fn sscript(
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
    #[named]
    #[default(true)]
    cramped: bool,
) -> Content {
    body.styled(EquationElem::set_size(MathSize::ScriptScript))
        .styled(EquationElem::set_cramped(cramped))
}

/// The size of elements in an equation.
///
/// See the TeXbook p. 141.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Cast, Hash)]
pub enum MathSize {
    /// Second-level sub- and superscripts.
    ScriptScript,
    /// Sub- and superscripts.
    Script,
    /// Math in text.
    Text,
    /// Math on its own line.
    Display,
}

/// A mathematical style variant, as defined by Unicode.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Cast, Hash)]
pub enum MathVariant {
    #[default]
    Serif,
    Sans,
    Cal,
    Frak,
    Mono,
    Bb,
}
