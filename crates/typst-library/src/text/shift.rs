use ecow::EcoString;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, Packed, SequenceElem, Show, StyleChain};
use crate::layout::{Em, Length};
use crate::text::{variant, SpaceElem, TextElem, TextSize};
use crate::World;

/// テキストを下付き文字でレンダリング。
///
/// テキストは小さくレンダリングされ、ベースラインは低くなります。
///
/// # 例
/// ```example
/// Revenue#sub[yearly]
/// ```
#[elem(title = "Subscript", Show)]
pub struct SubElem {
    /// フォントの下付き文字専用の字形を優先するかどうか。
    ///
    /// 有効化された場合、Typstは最初にテキストを下付き文字のコードポイントに変換できるか試します。
    /// 失敗した場合は、通常の文字を縮小し、位置を下げる挙動にフォールバックします。
    ///
    /// ```example
    /// N#sub(typographic: true)[1]
    /// N#sub(typographic: false)[1]
    /// ```
    #[default(true)]
    pub typographic: bool,

    /// 下付き文字の合成に用いるベースラインのシフト。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが下付き文字のコードポイントを持っている場合は適用されません。
    #[default(Em::new(0.2).into())]
    pub baseline: Length,

    /// 下付き文字の合成に用いるフォントの大きさ。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが下付き文字のコードポイントを持っている場合は適用されません。
    #[default(TextSize(Em::new(0.6).into()))]
    pub size: TextSize,

    /// 下付き文字で表示するテキスト。
    #[required]
    pub body: Content,
}

impl Show for Packed<SubElem> {
    #[typst_macros::time(name = "sub", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let body = self.body.clone();

        if self.typographic(styles) {
            if let Some(text) = convert_script(&body, true) {
                if is_shapable(engine, &text, styles) {
                    return Ok(TextElem::packed(text));
                }
            }
        };

        Ok(body
            .styled(TextElem::set_baseline(self.baseline(styles)))
            .styled(TextElem::set_size(self.size(styles))))
    }
}

/// テキストを上付き文字でレンダリング。
///
/// テキストは小さくレンダリングされ、ベースラインは高くなります。
///
/// # 例
/// ```example
/// 1#super[st] try!
/// ```
#[elem(title = "Superscript", Show)]
pub struct SuperElem {
    /// フォントの上付き文字専用の字形を優先するかどうか。
    ///
    /// 有効化された場合、Typstは最初にテキストを上付き文字のコードポイントに変換できるか試します。
    /// 失敗した場合は、通常の文字を縮小し、位置を上げる挙動にフォールバックします。
    ///
    /// ```example
    /// N#super(typographic: true)[1]
    /// N#super(typographic: false)[1]
    /// ```
    #[default(true)]
    pub typographic: bool,

    /// 上付き文字の合成に用いるベースラインのシフト。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが上付き文字のコードポイントを持っている場合は適用されません。
    #[default(Em::new(-0.5).into())]
    pub baseline: Length,

    /// 上付き文字の合成に用いるフォントの大きさ。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが上付き文字のコードポイントを持っている場合は適用されません。
    #[default(TextSize(Em::new(0.6).into()))]
    pub size: TextSize,

    /// 上付き文字で表示するテキスト。
    #[required]
    pub body: Content,
}

impl Show for Packed<SuperElem> {
    #[typst_macros::time(name = "super", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let body = self.body.clone();

        if self.typographic(styles) {
            if let Some(text) = convert_script(&body, false) {
                if is_shapable(engine, &text, styles) {
                    return Ok(TextElem::packed(text));
                }
            }
        };

        Ok(body
            .styled(TextElem::set_baseline(self.baseline(styles)))
            .styled(TextElem::set_size(self.size(styles))))
    }
}

/// Find and transform the text contained in `content` to the given script kind
/// if and only if it only consists of `Text`, `Space`, and `Empty` leaves.
fn convert_script(content: &Content, sub: bool) -> Option<EcoString> {
    if content.is::<SpaceElem>() {
        Some(' '.into())
    } else if let Some(elem) = content.to_packed::<TextElem>() {
        if sub {
            elem.text.chars().map(to_subscript_codepoint).collect()
        } else {
            elem.text.chars().map(to_superscript_codepoint).collect()
        }
    } else if let Some(sequence) = content.to_packed::<SequenceElem>() {
        sequence
            .children
            .iter()
            .map(|item| convert_script(item, sub))
            .collect()
    } else {
        None
    }
}

/// Checks whether the first retrievable family contains all code points of the
/// given string.
fn is_shapable(engine: &Engine, text: &str, styles: StyleChain) -> bool {
    let world = engine.world;
    for family in TextElem::font_in(styles) {
        if let Some(font) = world
            .book()
            .select(family.as_str(), variant(styles))
            .and_then(|id| world.font(id))
        {
            let covers = family.covers();
            return text.chars().all(|c| {
                covers.map_or(true, |cov| cov.is_match(c.encode_utf8(&mut [0; 4])))
                    && font.ttf().glyph_index(c).is_some()
            });
        }
    }

    false
}

/// Convert a character to its corresponding Unicode superscript.
fn to_superscript_codepoint(c: char) -> Option<char> {
    match c {
        '1' => Some('¹'),
        '2' => Some('²'),
        '3' => Some('³'),
        '0' | '4'..='9' => char::from_u32(c as u32 - '0' as u32 + '⁰' as u32),
        '+' => Some('⁺'),
        '−' => Some('⁻'),
        '=' => Some('⁼'),
        '(' => Some('⁽'),
        ')' => Some('⁾'),
        'n' => Some('ⁿ'),
        'i' => Some('ⁱ'),
        ' ' => Some(' '),
        _ => None,
    }
}

/// Convert a character to its corresponding Unicode subscript.
fn to_subscript_codepoint(c: char) -> Option<char> {
    match c {
        '0'..='9' => char::from_u32(c as u32 - '0' as u32 + '₀' as u32),
        '+' => Some('₊'),
        '−' => Some('₋'),
        '=' => Some('₌'),
        '(' => Some('₍'),
        ')' => Some('₎'),
        'a' => Some('ₐ'),
        'e' => Some('ₑ'),
        'o' => Some('ₒ'),
        'x' => Some('ₓ'),
        'h' => Some('ₕ'),
        'k' => Some('ₖ'),
        'l' => Some('ₗ'),
        'm' => Some('ₘ'),
        'n' => Some('ₙ'),
        'p' => Some('ₚ'),
        's' => Some('ₛ'),
        't' => Some('ₜ'),
        ' ' => Some(' '),
        _ => None,
    }
}
