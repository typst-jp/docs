use codex::styling::MathVariant;

use crate::foundations::{Cast, Content, func};
use crate::math::EquationElem;

<<<<<<< HEAD
/// 数式中の太字フォントスタイル。
=======
/// Bold font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ bold(A) := B^+ $
/// ```
#[func(keywords = ["mathbf"])]
pub fn bold(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::bold, true)
}

<<<<<<< HEAD
/// 数式中の立体（非斜体）フォントスタイル。
=======
/// Upright (non-italic) font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ upright(A) != A $
/// ```
#[func(keywords = ["mathup"])]
pub fn upright(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::italic, Some(false))
}

<<<<<<< HEAD
/// 数式中の斜体フォントスタイル。
///
/// これがローマ字とギリシャ文字の小文字のデフォルトです。
#[func(keywords = ["mathit"])]
pub fn italic(
    /// スタイルを適用するコンテンツ。
=======
/// Italic font style in math.
///
/// For roman letters and greek lowercase letters, this is already the default.
#[func(keywords = ["mathit"])]
pub fn italic(
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::italic, Some(true))
}

<<<<<<< HEAD
/// 数式中のセリフ（ローマン）フォントスタイル。
///
/// これがデフォルトです。
#[func(keywords = ["mathrm"])]
pub fn serif(
    /// スタイルを適用するコンテンツ。
=======
/// Serif (roman) font style in math.
///
/// This is already the default.
#[func(keywords = ["mathrm"])]
pub fn serif(
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Plain))
}

<<<<<<< HEAD
/// 数式中のサンセリフフォントスタイル。
=======
/// Sans-serif font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ sans(A B C) $
/// ```
#[func(title = "Sans Serif", keywords = ["mathsf"])]
pub fn sans(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::SansSerif))
}

<<<<<<< HEAD
/// 数式中のカリグラフィーフォントスタイル。
=======
/// Calligraphic (chancery) font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// Let $cal(P)$ be the set of ...
/// ```
///
<<<<<<< HEAD
/// これは大半の数式フォントにおけるデフォルトのカリグラフィー／スクリプトスタイルです。
/// もう一方のスタイル（roundhand）の指定方法については[`scr`]($math.scr)を参照してください。
#[func(title = "Calligraphic", keywords = ["mathcal", "chancery"])]
pub fn cal(
    /// スタイルを適用するコンテンツ。
=======
/// This is the default calligraphic/script style for most math fonts. See
/// [`scr`]($math.scr) for more on how to get the other style (roundhand).
#[func(title = "Calligraphic", keywords = ["mathcal", "chancery"])]
pub fn cal(
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Chancery))
}

<<<<<<< HEAD
/// 数式中のスクリプト（roundhand）フォントスタイル。
=======
/// Script (roundhand) font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $scr(L)$ is not the set of linear
/// maps $cal(L)$.
/// ```
///
<<<<<<< HEAD
/// フォントが`cal`と`scr`を区別できるようにする方法は2つあります。
/// 1つはUnicodeの字形指示列を用いる方法です。
/// これはTypstでそのまま動作しますが、現時点でこの方式をサポートする数式フォントはわずかです。
///
/// もう1つは[フォントフィーチャー]($text.features)を用いる方法です。
/// 例えば、roundhandスタイルがフォントの_[スタイリスティックセット]($text.stylistic-set)1_（`ss01`）フィーチャーを通じて利用できる場合があります。
/// 以下の例のように独自の`scr`関数を定義するとTypstで使用できます。
///
/// ```example:"スタイリスティックセット1による再現"
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// #let scr(it) = text(
///   stylistic-set: 1,
///   $cal(it)$,
/// )
///
/// We establish $cal(P) != scr(P)$.
/// ```
#[func(title = "Script Style", keywords = ["mathscr", "roundhand"])]
pub fn scr(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Roundhand))
}

<<<<<<< HEAD
/// 数式中のフラクトゥールフォントスタイル。
=======
/// Fraktur font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ frak(P) $
/// ```
#[func(title = "Fraktur", keywords = ["mathfrak"])]
pub fn frak(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Fraktur))
}

<<<<<<< HEAD
/// 数式中の等幅フォントスタイル。
=======
/// Monospace font style in math.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// $ mono(x + y = z) $
/// ```
#[func(title = "Monospace", keywords = ["mathtt"])]
pub fn mono(
<<<<<<< HEAD
    /// スタイルを適用するコンテンツ。
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::Monospace))
}

<<<<<<< HEAD
/// 数式中の黒板太字（double-struck）フォントスタイル。
///
/// 大文字のラテン文字では、黒板太字は、[symbols]($category/symbols/sym)にあるように、`NN`や`RR`のような形式でも使用できます。
=======
/// Blackboard bold (double-struck) font style in math.
///
/// For uppercase latin letters, blackboard bold is additionally available
/// through [symbols]($category/symbols/sym) of the form `NN` and `RR`.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
=======
    /// The content to style.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    body: Content,
) -> Content {
    body.set(EquationElem::variant, Some(MathVariant::DoubleStruck))
}

<<<<<<< HEAD
/// 数式中でディスプレイスタイルを強制します。
///
/// これはブロック数式における標準サイズです。
=======
/// Forced display style in math.
///
/// This is the normal size for block equations.
///
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[named]
    #[default(false)]
    cramped: bool,
) -> Content {
    body.set(EquationElem::size, MathSize::Display)
        .set(EquationElem::cramped, cramped)
}

<<<<<<< HEAD
/// 数式中でインライン（テキスト）スタイルを強制します。
///
/// これはインライン数式における標準サイズです。
=======
/// Forced inline (text) style in math.
///
/// This is the normal size for inline equations.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[named]
    #[default(false)]
    cramped: bool,
) -> Content {
    body.set(EquationElem::size, MathSize::Text)
        .set(EquationElem::cramped, cramped)
}

<<<<<<< HEAD
/// 数式中でスクリプトスタイルを強制します。
///
/// これは、冪乗、下付き文字、上付き文字で使用される小さいサイズです。
=======
/// Forced script style in math.
///
/// This is the smaller size used in powers or sub- or superscripts.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[named]
    #[default(true)]
    cramped: bool,
) -> Content {
    body.set(EquationElem::size, MathSize::Script)
        .set(EquationElem::cramped, cramped)
}

<<<<<<< HEAD
/// 数式中で第2スクリプトスタイルを強制します。
///
/// これは、第2レベルの下付き文字や上付き文字（添え字の添え字）で使用される最も小さいサイズです。
=======
/// Forced second script style in math.
///
/// This is the smallest size, used in second-level sub- and superscripts
/// (script of the script).
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
