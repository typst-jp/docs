<<<<<<< HEAD
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
=======
use crate::introspection::Tagged;
use ttf_parser::Tag;

use crate::foundations::{Content, Smart, elem};
use crate::layout::{Em, Length};
use crate::text::{FontMetrics, ScriptMetrics, TextSize};

/// Renders text in subscript.
///
/// The text is rendered smaller and its baseline is lowered.
///
/// # Example
/// ```example
/// Revenue#sub[yearly]
/// ```
#[elem(title = "Subscript", Tagged)]
pub struct SubElem {
    /// Whether to use subscript glyphs from the font if available.
    ///
    /// Ideally, subscripts glyphs are provided by the font (using the `subs`
    /// OpenType feature). Otherwise, Typst is able to synthesize subscripts by
    /// lowering and scaling down regular glyphs.
    ///
    /// When this is set to `{false}`, synthesized glyphs will be used
    /// regardless of whether the font provides dedicated subscript glyphs. When
    /// `{true}`, synthesized glyphs may still be used in case the font does not
    /// provide the necessary subscript glyphs.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// N#sub(typographic: true)[1]
    /// N#sub(typographic: false)[1]
    /// ```
    #[default(true)]
    pub typographic: bool,

<<<<<<< HEAD
    /// 下付き文字の合成に用いるベースラインのシフト。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが下付き文字のコードポイントを持っている場合は適用されません。
    #[default(Em::new(0.2).into())]
    pub baseline: Length,

    /// 下付き文字の合成に用いるフォントの大きさ。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが下付き文字のコードポイントを持っている場合は適用されません。
    #[default(TextSize(Em::new(0.6).into()))]
    pub size: TextSize,

    /// 下付き文字で表示するテキスト。
=======
    /// The downward baseline shift for synthesized subscripts.
    ///
    /// This only applies to synthesized subscripts. In other words, this has no
    /// effect if `typographic` is `{true}` and the font provides the necessary
    /// subscript glyphs.
    ///
    /// If set to `{auto}`, the baseline is shifted according to the metrics
    /// provided by the font, with a fallback to `{0.2em}` in case the font does
    /// not define the necessary metrics.
    pub baseline: Smart<Length>,

    /// The font size for synthesized subscripts.
    ///
    /// This only applies to synthesized subscripts. In other words, this has no
    /// effect if `typographic` is `{true}` and the font provides the necessary
    /// subscript glyphs.
    ///
    /// If set to `{auto}`, the size is scaled according to the metrics provided
    /// by the font, with a fallback to `{0.6em}` in case the font does not
    /// define the necessary metrics.
    pub size: Smart<TextSize>,

    /// The text to display in subscript.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
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
=======
/// Renders text in superscript.
///
/// The text is rendered smaller and its baseline is raised.
///
/// # Example
/// ```example
/// 1#super[st] try!
/// ```
#[elem(title = "Superscript", Tagged)]
pub struct SuperElem {
    /// Whether to use superscript glyphs from the font if available.
    ///
    /// Ideally, superscripts glyphs are provided by the font (using the `sups`
    /// OpenType feature). Otherwise, Typst is able to synthesize superscripts
    /// by raising and scaling down regular glyphs.
    ///
    /// When this is set to `{false}`, synthesized glyphs will be used
    /// regardless of whether the font provides dedicated superscript glyphs.
    /// When `{true}`, synthesized glyphs may still be used in case the font
    /// does not provide the necessary superscript glyphs.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// N#super(typographic: true)[1]
    /// N#super(typographic: false)[1]
    /// ```
    #[default(true)]
    pub typographic: bool,

<<<<<<< HEAD
    /// 上付き文字の合成に用いるベースラインのシフト。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが上付き文字のコードポイントを持っている場合は適用されません。
    #[default(Em::new(-0.5).into())]
    pub baseline: Length,

    /// 上付き文字の合成に用いるフォントの大きさ。
    /// `typographic`がtrueかつ与えられた`body`に対してフォントが上付き文字のコードポイントを持っている場合は適用されません。
    #[default(TextSize(Em::new(0.6).into()))]
    pub size: TextSize,

    /// 上付き文字で表示するテキスト。
=======
    /// The downward baseline shift for synthesized superscripts.
    ///
    /// This only applies to synthesized superscripts. In other words, this has
    /// no effect if `typographic` is `{true}` and the font provides the
    /// necessary superscript glyphs.
    ///
    /// If set to `{auto}`, the baseline is shifted according to the metrics
    /// provided by the font, with a fallback to `{-0.5em}` in case the font
    /// does not define the necessary metrics.
    ///
    /// Note that, since the baseline shift is applied downward, you will need
    /// to provide a negative value for the content to appear as raised above
    /// the normal baseline.
    pub baseline: Smart<Length>,

    /// The font size for synthesized superscripts.
    ///
    /// This only applies to synthesized superscripts. In other words, this has
    /// no effect if `typographic` is `{true}` and the font provides the
    /// necessary superscript glyphs.
    ///
    /// If set to `{auto}`, the size is scaled according to the metrics provided
    /// by the font, with a fallback to `{0.6em}` in case the font does not
    /// define the necessary metrics.
    pub size: Smart<TextSize>,

    /// The text to display in superscript.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
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
=======
/// Configuration values for sub- or superscript text.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShiftSettings {
    /// Whether the OpenType feature should be used if possible.
    pub typographic: bool,
    /// The baseline shift of the script, relative to the outer text size.
    ///
    /// For superscripts, this is positive. For subscripts, this is negative. A
    /// value of [`Smart::Auto`] indicates that the value should be obtained
    /// from font metrics.
    pub shift: Smart<Em>,
    /// The size of the script, relative to the outer text size.
    ///
    /// A value of [`Smart::Auto`] indicates that the value should be obtained
    /// from font metrics.
    pub size: Smart<Em>,
    /// The kind of script (either a subscript, or a superscript).
    ///
    /// This is used to know which OpenType table to use to resolve
    /// [`Smart::Auto`] values.
    pub kind: ScriptKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScriptKind {
    Sub,
    Super,
}

impl ScriptKind {
    /// Returns the default metrics for this script kind.
    ///
    /// This can be used as a last resort if neither the user nor the font
    /// provided those metrics.
    pub fn default_metrics(self) -> &'static ScriptMetrics {
        match self {
            Self::Sub => &DEFAULT_SUBSCRIPT_METRICS,
            Self::Super => &DEFAULT_SUPERSCRIPT_METRICS,
        }
    }

    /// Reads the script metrics from the font table for to this script kind.
    pub fn read_metrics(self, font_metrics: &FontMetrics) -> &ScriptMetrics {
        match self {
            Self::Sub => font_metrics.subscript.as_ref(),
            Self::Super => font_metrics.superscript.as_ref(),
        }
        .unwrap_or(self.default_metrics())
    }

    /// The corresponding OpenType feature.
    pub const fn feature(self) -> Tag {
        match self {
            Self::Sub => Tag::from_bytes(b"subs"),
            Self::Super => Tag::from_bytes(b"sups"),
        }
    }
}
pub static DEFAULT_SUBSCRIPT_METRICS: ScriptMetrics = ScriptMetrics {
    width: Em::new(0.6),
    height: Em::new(0.6),
    horizontal_offset: Em::zero(),
    vertical_offset: Em::new(-0.2),
};

pub static DEFAULT_SUPERSCRIPT_METRICS: ScriptMetrics = ScriptMetrics {
    width: Em::new(0.6),
    height: Em::new(0.6),
    horizontal_offset: Em::zero(),
    vertical_offset: Em::new(0.5),
};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
