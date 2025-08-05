use crate::foundations::{func, Cast, Content, Smart};
use crate::math::EquationElem;

/// Bold font style in math.
///
/// ```example
/// $ bold(A) := B^+ $
/// ```
#[func(keywords = ["mathbf"])]
pub fn bold(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_bold(true))
}

/// Upright (non-italic) font style in math.
///
/// ```example
/// $ upright(A) != A $
/// ```
#[func(keywords = ["mathup"])]
pub fn upright(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_italic(Smart::Custom(false)))
}

/// Italic font style in math.
///
/// For roman letters and greek lowercase letters, this is already the default.
#[func(keywords = ["mathit"])]
pub fn italic(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_italic(Smart::Custom(true)))
}

/// Serif (roman) font style in math.
///
/// This is already the default.
#[func(keywords = ["mathrm"])]
pub fn serif(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Serif))
}

/// Sans-serif font style in math.
///
/// ```example
/// $ sans(A B C) $
/// ```
#[func(title = "Sans Serif", keywords = ["mathsf"])]
pub fn sans(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Sans))
}

/// Calligraphic font style in math.
///
/// ```example
/// Let $cal(P)$ be the set of ...
/// ```
///
/// This corresponds both to LaTeX's `\mathcal` and `\mathscr` as both of these
/// styles share the same Unicode codepoints. Switching between the styles is
/// thus only possible if supported by the font via
/// [font features]($text.features).
///
/// For the default math font, the roundhand style is available through the
/// `ss01` feature. Therefore, you could define your own version of `\mathscr`
/// like this:
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
/// (The box is not conceptually necessary, but unfortunately currently needed
/// due to limitations in Typst's text style handling in math.)
#[func(title = "Calligraphic", keywords = ["mathcal", "mathscr"])]
pub fn cal(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Cal))
}

/// Fraktur font style in math.
///
/// ```example
/// $ frak(P) $
/// ```
#[func(title = "Fraktur", keywords = ["mathfrak"])]
pub fn frak(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Frak))
}

/// Monospace font style in math.
///
/// ```example
/// $ mono(x + y = z) $
/// ```
#[func(title = "Monospace", keywords = ["mathtt"])]
pub fn mono(
    /// The content to style.
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Mono))
}

/// Blackboard bold (double-struck) font style in math.
///
/// For uppercase latin letters, blackboard bold is additionally available
/// through [symbols]($category/symbols/sym) of the form `NN` and `RR`.
///
/// ```example
/// $ bb(b) $
/// $ bb(N) = NN $
/// $ f: NN -> RR $
/// ```
#[func(title = "Blackboard Bold", keywords = ["mathbb"])]
pub fn bb(
    /// The content to style.
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
