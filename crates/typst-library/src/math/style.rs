<<<<<<< HEAD
use crate::foundations::{func, Cast, Content, Smart};
use crate::math::EquationElem;

/// 数式中の太字フォントスタイル。
=======
use codex::styling::MathVariant;

use crate::foundations::{Cast, Content, func};
use crate::math::EquationElem;

/// Bold font style in math.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ bold(A) := B^+ $
/// ```
#[func(keywords = ["mathbf"])]
pub fn bold(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_bold(true))
}

/// 数式中の立体（非斜体）フォントスタイル。
=======
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::bold, true)
}

/// Upright (non-italic) font style in math.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ upright(A) != A $
/// ```
#[func(keywords = ["mathup"])]
pub fn upright(
<<<<<<< HEAD
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
=======
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::italic, Some(false))
}

/// Italic font style in math.
///
/// For roman letters and greek lowercase letters, this is already the default.
#[func(keywords = ["mathit"])]
pub fn italic(
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::italic, Some(true))
}

/// Serif (roman) font style in math.
///
/// This is already the default.
#[func(keywords = ["mathrm"])]
pub fn serif(
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Plain))
}

/// Sans-serif font style in math.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ sans(A B C) $
/// ```
#[func(title = "Sans Serif", keywords = ["mathsf"])]
pub fn sans(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Sans))
}

/// 数式中のカリグラフィーフォントスタイル。
=======
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::SansSerif))
}

/// Calligraphic (chancery) font style in math.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// Let $cal(P)$ be the set of ...
/// ```
///
<<<<<<< HEAD
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
=======
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// )
///
/// We establish $cal(P) != scr(P)$.
/// ```
<<<<<<< HEAD
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
=======
#[func(title = "Script Style", keywords = ["mathscr", "roundhand"])]
pub fn scr(
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Roundhand))
}

/// Fraktur font style in math.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ frak(P) $
/// ```
#[func(title = "Fraktur", keywords = ["mathfrak"])]
pub fn frak(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Frak))
}

/// 数式中の等幅フォントスタイル。
=======
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Fraktur))
}

/// Monospace font style in math.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ mono(x + y = z) $
/// ```
#[func(title = "Monospace", keywords = ["mathtt"])]
pub fn mono(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Mono))
}

/// 数式中の黒板太字（double-struck）フォントスタイル。
///
/// 大文字のラテン文字では、黒板太字は、[symbols]($category/symbols/sym)にあるように、`NN`や`RR`のような形式でも使用できます。
=======
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Monospace))
}

/// Blackboard bold (double-struck) font style in math.
///
/// For uppercase latin letters, blackboard bold is additionally available
/// through [symbols]($category/symbols/sym) of the form `NN` and `RR`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ bb(b) $
/// $ bb(N) = NN $
/// $ f: NN -> RR $
/// ```
#[func(title = "Blackboard Bold", keywords = ["mathbb"])]
pub fn bb(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
    body: Content,
) -> Content {
    body.styled(EquationElem::set_variant(MathVariant::Bb))
}

/// 数式中でディスプレイスタイルを強制します。
///
/// これはブロック数式における標準サイズです。

=======
    /// The content to style.
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::DoubleStruck))
}

/// Forced display style in math.
///
/// This is the normal size for block equations.
///
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// $sum_i x_i/2 = display(sum_i x_i/2)$
/// ```
#[func(title = "Display Size", keywords = ["displaystyle"])]
pub fn display(
<<<<<<< HEAD
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
=======
    /// The content to size.
    body: Content,
    /// Whether to impose a height restriction for exponents, like regular sub-
    /// and superscripts do.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[named]
    #[default(false)]
    cramped: bool,
) -> Content {
<<<<<<< HEAD
    body.styled(EquationElem::set_size(MathSize::Display))
        .styled(EquationElem::set_cramped(cramped))
}

/// 数式中でインライン（テキスト）スタイルを強制します。
///
/// これはインライン数式における標準サイズです。
=======
    body.set(EquationElem::size, MathSize::Display)
        .set(EquationElem::cramped, cramped)
}

/// Forced inline (text) style in math.
///
/// This is the normal size for inline equations.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ sum_i x_i/2
///     = inline(sum_i x_i/2) $
/// ```
#[func(title = "Inline Size", keywords = ["textstyle"])]
pub fn inline(
<<<<<<< HEAD
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
=======
    /// The content to size.
    body: Content,
    /// Whether to impose a height restriction for exponents, like regular sub-
    /// and superscripts do.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[named]
    #[default(false)]
    cramped: bool,
) -> Content {
<<<<<<< HEAD
    body.styled(EquationElem::set_size(MathSize::Text))
        .styled(EquationElem::set_cramped(cramped))
}

/// 数式中でスクリプトスタイルを強制します。
///
/// これは、冪乗、下付き文字、上付き文字で使用される小さいサイズです。
=======
    body.set(EquationElem::size, MathSize::Text)
        .set(EquationElem::cramped, cramped)
}

/// Forced script style in math.
///
/// This is the smaller size used in powers or sub- or superscripts.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $sum_i x_i/2 = script(sum_i x_i/2)$
/// ```
#[func(title = "Script Size", keywords = ["scriptstyle"])]
pub fn script(
<<<<<<< HEAD
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
=======
    /// The content to size.
    body: Content,
    /// Whether to impose a height restriction for exponents, like regular sub-
    /// and superscripts do.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[named]
    #[default(true)]
    cramped: bool,
) -> Content {
<<<<<<< HEAD
    body.styled(EquationElem::set_size(MathSize::Script))
        .styled(EquationElem::set_cramped(cramped))
}

/// 数式中で第2スクリプトスタイルを強制します。
///
/// これは、第2レベルの下付き文字や上付き文字（添え字の添え字）で使用される最も小さいサイズです。
=======
    body.set(EquationElem::size, MathSize::Script)
        .set(EquationElem::cramped, cramped)
}

/// Forced second script style in math.
///
/// This is the smallest size, used in second-level sub- and superscripts
/// (script of the script).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $sum_i x_i/2 = sscript(sum_i x_i/2)$
/// ```
#[func(title = "Script-Script Size", keywords = ["scriptscriptstyle"])]
pub fn sscript(
<<<<<<< HEAD
    /// 大きさを指定したいコンテンツ。
    body: Content,
    /// 通常の下付き文字や上付き文字のように、指数に高さ制限を課すかどうか。
=======
    /// The content to size.
    body: Content,
    /// Whether to impose a height restriction for exponents, like regular sub-
    /// and superscripts do.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[named]
    #[default(true)]
    cramped: bool,
) -> Content {
<<<<<<< HEAD
    body.styled(EquationElem::set_size(MathSize::ScriptScript))
        .styled(EquationElem::set_cramped(cramped))
=======
    body.set(EquationElem::size, MathSize::ScriptScript)
        .set(EquationElem::cramped, cramped)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// The size of elements in an equation.
///
/// See the TeXbook p. 141.
<<<<<<< HEAD
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Cast, Hash)]
=======
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Cast)]
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD

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
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
