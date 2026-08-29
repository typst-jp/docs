use crate::introspection::Tagged;
use ttf_parser::Tag;

use crate::foundations::{Content, Smart, elem};
use crate::layout::{Em, Length};
use crate::text::{FontMetrics, ScriptMetrics, TextSize};

<<<<<<< HEAD
/// テキストを下付き文字でレンダリング。
///
/// テキストは小さくレンダリングされ、ベースラインは低くなります。
///
/// # 例
=======
/// Renders text in subscript.
///
/// The text is rendered smaller and its baseline is lowered.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// Revenue#sub[yearly]
/// ```
#[elem(title = "Subscript", Tagged)]
pub struct SubElem {
<<<<<<< HEAD
    /// フォントの下付き文字専用のグリフを優先するかどうか。
    ///
    /// 理想的にはフォントがOpenTypeフィーチャーの`subs`で下付きグリフを提供します。
    /// そうでない場合は、通常のグリフを縮小して下げて合成します。
    ///
    /// `{false}`にすると、フォントが下付きグリフを持っていても合成を使用します。
    /// `{true}`でも、フォントに必要なグリフがない場合は合成にフォールバックします。
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// N#sub(typographic: true)[1]
    /// N#sub(typographic: false)[1]
    /// ```
    #[default(true)]
    pub typographic: bool,

<<<<<<< HEAD
    /// 合成した下付き文字のベースラインの下方向シフト。
    ///
    /// これは合成時のみ有効です。つまり、`typographic`が`true`かつ、
    /// フォントが必要な下付きグリフを持つ場合は効果がありません。
    ///
    /// `{auto}`の場合、フォントメトリクスに従ってシフトし、
    /// メトリクスがない場合は`{0.2em}`にフォールバックします。
    pub baseline: Smart<Length>,

    /// 合成した下付き文字のフォントサイズ。
    ///
    /// これは合成時のみ有効です。つまり、`typographic`が`true`かつ、
    /// フォントが必要な下付きグリフを持つ場合は効果がありません。
    ///
    /// `{auto}`の場合、フォントメトリクスに従って拡縮し、
    /// メトリクスがない場合は`{0.6em}`にフォールバックします。
    pub size: Smart<TextSize>,

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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// テキストを上付き文字でレンダリング。
///
/// テキストは小さくレンダリングされ、ベースラインは高くなります。
///
/// # 例
=======
/// Renders text in superscript.
///
/// The text is rendered smaller and its baseline is raised.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// 1#super[st] try!
/// ```
#[elem(title = "Superscript", Tagged)]
pub struct SuperElem {
<<<<<<< HEAD
    /// フォントの上付き文字専用のグリフを優先するかどうか。
    ///
    /// 理想的にはフォントがOpenTypeフィーチャーの`sups`で上付きグリフを提供します。
    /// そうでない場合は、通常のグリフを縮小して持ち上げて合成します。
    ///
    /// `{false}`にすると、フォントが上付きグリフを持っていても合成を使用します。
    /// `{true}`でも、フォントに必要なグリフがない場合は合成にフォールバックします。
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// N#super(typographic: true)[1]
    /// N#super(typographic: false)[1]
    /// ```
    #[default(true)]
    pub typographic: bool,

<<<<<<< HEAD
    /// 合成した上付き文字のベースラインの下方向シフト。
    ///
    /// これは合成時のみ有効です。つまり、`typographic`が`true`かつ、
    /// フォントが必要な上付きグリフを持つ場合は効果がありません。
    ///
    /// `{auto}`の場合、フォントメトリクスに従ってシフトし、
    /// メトリクスがない場合は`{-0.5em}`にフォールバックします。
    ///
    /// ベースラインは下方向に適用されるため、上に上げたい場合は
    /// 負の値を指定してください。
    pub baseline: Smart<Length>,

    /// 合成した上付き文字のフォントサイズ。
    ///
    /// これは合成時のみ有効です。つまり、`typographic`が`true`かつ、
    /// フォントが必要な上付きグリフを持つ場合は効果がありません。
    ///
    /// `{auto}`の場合、フォントメトリクスに従って拡縮し、
    /// メトリクスがない場合は`{0.6em}`にフォールバックします。
    pub size: Smart<TextSize>,

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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    pub body: Content,
}

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
