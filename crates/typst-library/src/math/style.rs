use codex::styling::MathVariant;

use crate::foundations::{Cast, Content, func};
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
    body.set(EquationElem::bold, true)
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
    body.set(EquationElem::italic, Some(false))
}

/// 数式中の斜体フォントスタイル。
///
/// これがローマ字とギリシャ文字の小文字のデフォルトです。
#[func(keywords = ["mathit"])]
pub fn italic(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.set(EquationElem::italic, Some(true))
}

/// 数式中のセリフ（ローマン）フォントスタイル。
///
/// これがデフォルトです。
#[func(keywords = ["mathrm"])]
pub fn serif(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Plain))
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
    body.set(EquationElem::variant, Some(MathVariant::SansSerif))
}

/// 数式中のカリグラフィーフォントスタイル。
///
/// ```example
/// Let $cal(P)$ be the set of ...
/// ```
///
/// This is the default calligraphic/script style for most math fonts. See
/// [`scr`]($math.scr) for more on how to get the other style (roundhand).
#[func(title = "Calligraphic", keywords = ["mathcal", "chancery"])]
pub fn cal(
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Chancery))
}

/// Script (roundhand) font style in math.
///
/// ```example
/// $scr(L)$ is not the set of linear
/// maps $cal(L)$.
/// ```
///
/// There are two ways that fonts can support differentiating `cal` and `scr`.
/// The first is using Unicode variation sequences. This works out of the box
/// in Typst, however only a few math fonts currently support this.
///
/// The other way is using [font features]($text.features). For example, the
/// roundhand style might be available in a font through the
/// _[stylistic set]($text.stylistic-set) 1_ (`ss01`) feature. To use it in
/// Typst, you could then define your own version of `scr` like in the example
/// below.
///
/// ```example:"Recreation using stylistic set 1"
/// #let scr(it) = text(
///   stylistic-set: 1,
///   $cal(it)$,
/// )
///
/// We establish $cal(P) != scr(P)$.
/// ```
#[func(title = "Script Style", keywords = ["mathscr", "roundhand"])]
pub fn scr(
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Roundhand))
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
    body.set(EquationElem::variant, Some(MathVariant::Fraktur))
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
    body.set(EquationElem::variant, Some(MathVariant::Monospace))
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
    body.set(EquationElem::variant, Some(MathVariant::DoubleStruck))
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
    body.set(EquationElem::size, MathSize::Display)
        .set(EquationElem::cramped, cramped)
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
    body.set(EquationElem::size, MathSize::Text)
        .set(EquationElem::cramped, cramped)
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
    body.set(EquationElem::size, MathSize::Script)
        .set(EquationElem::cramped, cramped)
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
    body.set(EquationElem::size, MathSize::ScriptScript)
        .set(EquationElem::cramped, cramped)
}

/// The size of elements in an equation.
///
/// See the TeXbook p. 141.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Cast)]
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
