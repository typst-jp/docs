use std::str::FromStr;

use chinese_number::{
    ChineseCase, ChineseVariant, from_u64_to_chinese_ten_thousand as u64_to_chinese,
};
use comemo::Tracked;
use ecow::{EcoString, EcoVec, eco_format};

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{Context, Func, Str, Value, cast, func};

/// 順序に応じた番号付け。
///
/// 番号付けは、一連の数値をコンテンツとしてどのように表示するかを定義します。
/// これはパターン文字列または任意の関数によって指定されます。
///
/// 番号付けパターンは、数値を置き換えるためのカウント記号、それらに付けるプレフィックス、
/// そして1つのサフィックスから構成されます。プレフィックスとサフィックスは、
/// そのままの形で繰り返し使用されます。
///
/// # 例
/// ```example
/// #numbering("1.1)", 1, 2, 3) \
/// #numbering("1.a.i", 1, 2) \
/// #numbering("I – 1", 12, 2) \
/// #numbering(
///   (..nums) => nums
///     .pos()
///     .map(str)
///     .join(".") + ")",
///   1, 2, 3,
/// )
/// ```
///
/// # 番号付けのパターン指定と関数指定
/// Typstではパターン指定または関数指定で番号付けを定義できる場面がいくつかあります。
/// 例えば、[見出し]($heading)や[図表]($figure)などに番号を付ける際に使用します。
/// いずれの場合も、指定の形式は後述する[`numbering`]($numbering.numbering)パラメーターと同じです。
///
/// 次の例は、番号付け用の関数が、単に数値を受け取って[`content`]を返す通常の[function]であることを示しています。
/// ```example
/// #let unary(.., last) = "|" * last
/// #set heading(numbering: unary)
/// = First heading
/// = Second heading
/// = Third heading
/// ```
#[func]
pub fn numbering(
    engine: &mut Engine,
    context: Tracked<Context>,
    /// 番号付けの表示形式を定義します。
    ///
    /// **カウント記号**として使用できるパターン文字は `1`, `a`, `A`, `i`, `I`, `α`, `Α`, `一`, `壹`, `あ`, `い`, `ア`, `イ`, `א`, `가`, `ㄱ`, `*`, `١`, `۱`, `१`, `১`, `ক`, `①`, `⓵`があります。
    /// これらの文字は、大文字・小文字を維持したまま、対応する順序の番号文字に置き換えられます。
    ///
    /// 記号`*`は `*`, `†`, `‡`, `§`, `¶`, `‖`の順序で番号付けすることを意味します。
    /// 項目が6つ以上ある場合は、記号を繰り返し使用して番号を表現します。
    ///
    /// **サフィックス**とは、最後のカウント記号の後ろに置く文字列です。
    /// これらは、生成された番号文字の末尾に、そのままの形で繰り返し表示されます。
    ///
    /// **プレフィックス**は、カウント記号でもサフィックスでもない文字列です。
    /// それぞれのカウント記号の前に、そのままの形で繰り返し表示されます。
    ///
    /// このパラメータには、数値を個別の引数として受け取る任意の関数も指定できます。
    /// 関数が与えられた場合、`numbering`関数はその引数をそのまま関数に渡します。
    /// これ自体は特に便利というわけではありませんが、番号付けがパターン指定であっても関数指定であっても、番号付けの定義を`numbering`関数に適用できるという意味を持ちます。
    numbering: Numbering,
    /// 番号付けを適用する対象の数値。負でない数で与えてください。
    ///
    /// 一般に番号は1から数えます。値が0の場合は、最初の要素がまだ出現していないことを示します。
    ///
    /// 番号付けがパターン指定であり、カウント記号よりも多くの数値が指定された場合、
    /// 最後のカウント記号とそのプレフィックスが繰り返されます。
    #[variadic]
    numbers: Vec<u64>,
) -> SourceResult<Value> {
    numbering.apply(engine, context, &numbers)
}

/// How to number a sequence of things.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Numbering {
    /// A pattern with prefix, numbering, lower / upper case and suffix.
    Pattern(NumberingPattern),
    /// A closure mapping from an item's number to content.
    Func(Func),
}

impl Numbering {
    /// Apply the pattern to the given numbers.
    pub fn apply(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        numbers: &[u64],
    ) -> SourceResult<Value> {
        Ok(match self {
            Self::Pattern(pattern) => Value::Str(pattern.apply(numbers).into()),
            Self::Func(func) => func.call(engine, context, numbers.iter().copied())?,
        })
    }

    /// Trim the prefix suffix if this is a pattern.
    pub fn trimmed(mut self) -> Self {
        if let Self::Pattern(pattern) = &mut self {
            pattern.trimmed = true;
        }
        self
    }
}

impl From<NumberingPattern> for Numbering {
    fn from(pattern: NumberingPattern) -> Self {
        Self::Pattern(pattern)
    }
}

cast! {
    Numbering,
    self => match self {
        Self::Pattern(pattern) => pattern.into_value(),
        Self::Func(func) => func.into_value(),
    },
    v: NumberingPattern => Self::Pattern(v),
    v: Func => Self::Func(v),
}

/// How to turn a number into text.
///
/// A pattern consists of a prefix, followed by one of the counter symbols (see
/// [`numbering()`] docs), and then a suffix.
///
/// Examples of valid patterns:
/// - `1)`
/// - `a.`
/// - `(I)`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberingPattern {
    pub pieces: EcoVec<(EcoString, NumberingKind)>,
    pub suffix: EcoString,
    trimmed: bool,
}

impl NumberingPattern {
    /// Apply the pattern to the given number.
    pub fn apply(&self, numbers: &[u64]) -> EcoString {
        let mut fmt = EcoString::new();
        let mut numbers = numbers.iter();

        for (i, ((prefix, kind), &n)) in self.pieces.iter().zip(&mut numbers).enumerate()
        {
            if i > 0 || !self.trimmed {
                fmt.push_str(prefix);
            }
            fmt.push_str(&kind.apply(n));
        }

        for ((prefix, kind), &n) in self.pieces.last().into_iter().cycle().zip(numbers) {
            if prefix.is_empty() {
                fmt.push_str(&self.suffix);
            } else {
                fmt.push_str(prefix);
            }
            fmt.push_str(&kind.apply(n));
        }

        if !self.trimmed {
            fmt.push_str(&self.suffix);
        }

        fmt
    }

    /// Apply only the k-th segment of the pattern to a number.
    pub fn apply_kth(&self, k: usize, number: u64) -> EcoString {
        let mut fmt = EcoString::new();
        if let Some((prefix, _)) = self.pieces.first() {
            fmt.push_str(prefix);
        }
        if let Some((_, kind)) = self
            .pieces
            .iter()
            .chain(self.pieces.last().into_iter().cycle())
            .nth(k)
        {
            fmt.push_str(&kind.apply(number));
        }
        fmt.push_str(&self.suffix);
        fmt
    }

    /// How many counting symbols this pattern has.
    pub fn pieces(&self) -> usize {
        self.pieces.len()
    }
}

impl FromStr for NumberingPattern {
    type Err = &'static str;

    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
        let mut pieces = EcoVec::new();
        let mut handled = 0;

        for (i, c) in pattern.char_indices() {
            let Some(kind) = NumberingKind::from_char(c) else {
                continue;
            };

            let prefix = pattern[handled..i].into();
            pieces.push((prefix, kind));
            handled = c.len_utf8() + i;
        }

        let suffix = pattern[handled..].into();
        if pieces.is_empty() {
            return Err("invalid numbering pattern");
        }

        Ok(Self { pieces, suffix, trimmed: false })
    }
}

cast! {
    NumberingPattern,
    self => {
        let mut pat = EcoString::new();
        for (prefix, kind) in &self.pieces {
            pat.push_str(prefix);
            pat.push(kind.to_char());
        }
        pat.push_str(&self.suffix);
        pat.into_value()
    },
    v: Str => v.parse()?,
}

/// Different kinds of numberings.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NumberingKind {
    /// Arabic numerals (1, 2, 3, etc.).
    Arabic,
    /// Lowercase Latin letters (a, b, c, etc.). Items beyond z use base-26.
    LowerLatin,
    /// Uppercase Latin letters (A, B, C, etc.). Items beyond Z use base-26.
    UpperLatin,
    /// Lowercase Roman numerals (i, ii, iii, etc.).
    LowerRoman,
    /// Uppercase Roman numerals (I, II, III, etc.).
    UpperRoman,
    /// Lowercase Greek letters (α, β, γ, etc.).
    LowerGreek,
    /// Uppercase Greek letters (Α, Β, Γ, etc.).
    UpperGreek,
    /// Paragraph/note-like symbols: *, †, ‡, §, ¶, and ‖. Further items use
    /// repeated symbols.
    Symbol,
    /// Hebrew numerals, including Geresh/Gershayim.
    Hebrew,
    /// Simplified Chinese standard numerals. This corresponds to the
    /// `ChineseCase::Lower` variant.
    LowerSimplifiedChinese,
    /// Simplified Chinese "banknote" numerals. This corresponds to the
    /// `ChineseCase::Upper` variant.
    UpperSimplifiedChinese,
    // TODO: Pick the numbering pattern based on languages choice.
    // As the first character of Simplified and Traditional Chinese numbering
    // are the same, we are unable to determine if the context requires
    // Simplified or Traditional by only looking at this character.
    #[allow(unused)]
    /// Traditional Chinese standard numerals. This corresponds to the
    /// `ChineseCase::Lower` variant.
    LowerTraditionalChinese,
    #[allow(unused)]
    /// Traditional Chinese "banknote" numerals. This corresponds to the
    /// `ChineseCase::Upper` variant.
    UpperTraditionalChinese,
    /// Hiragana in the gojūon order. Includes n but excludes wi and we.
    HiraganaAiueo,
    /// Hiragana in the iroha order. Includes wi and we but excludes n.
    HiraganaIroha,
    /// Katakana in the gojūon order. Includes n but excludes wi and we.
    KatakanaAiueo,
    /// Katakana in the iroha order. Includes wi and we but excludes n.
    KatakanaIroha,
    /// Korean jamo (ㄱ, ㄴ, ㄷ, etc.).
    KoreanJamo,
    /// Korean syllables (가, 나, 다, etc.).
    KoreanSyllable,
    /// Eastern Arabic numerals, used in some Arabic-speaking countries.
    EasternArabic,
    /// The variant of Eastern Arabic numerals used in Persian and Urdu.
    EasternArabicPersian,
    /// Devanagari numerals.
    DevanagariNumber,
    /// Bengali numerals.
    BengaliNumber,
    /// Bengali letters (ক, খ, গ, ...কক, কখ etc.).
    BengaliLetter,
    /// Circled numbers (①, ②, ③, etc.), up to 50.
    CircledNumber,
    /// Double-circled numbers (⓵, ⓶, ⓷, etc.), up to 10.
    DoubleCircledNumber,
}

impl NumberingKind {
    /// Create a numbering kind from a representative character.
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '1' => NumberingKind::Arabic,
            'a' => NumberingKind::LowerLatin,
            'A' => NumberingKind::UpperLatin,
            'i' => NumberingKind::LowerRoman,
            'I' => NumberingKind::UpperRoman,
            'α' => NumberingKind::LowerGreek,
            'Α' => NumberingKind::UpperGreek,
            '*' => NumberingKind::Symbol,
            'א' => NumberingKind::Hebrew,
            '一' => NumberingKind::LowerSimplifiedChinese,
            '壹' => NumberingKind::UpperSimplifiedChinese,
            'あ' => NumberingKind::HiraganaAiueo,
            'い' => NumberingKind::HiraganaIroha,
            'ア' => NumberingKind::KatakanaAiueo,
            'イ' => NumberingKind::KatakanaIroha,
            'ㄱ' => NumberingKind::KoreanJamo,
            '가' => NumberingKind::KoreanSyllable,
            '\u{0661}' => NumberingKind::EasternArabic,
            '\u{06F1}' => NumberingKind::EasternArabicPersian,
            '\u{0967}' => NumberingKind::DevanagariNumber,
            '\u{09E7}' => NumberingKind::BengaliNumber,
            '\u{0995}' => NumberingKind::BengaliLetter,
            '①' => NumberingKind::CircledNumber,
            '⓵' => NumberingKind::DoubleCircledNumber,
            _ => return None,
        })
    }

    /// The representative character for this numbering kind.
    pub fn to_char(self) -> char {
        match self {
            Self::Arabic => '1',
            Self::LowerLatin => 'a',
            Self::UpperLatin => 'A',
            Self::LowerRoman => 'i',
            Self::UpperRoman => 'I',
            Self::LowerGreek => 'α',
            Self::UpperGreek => 'Α',
            Self::Symbol => '*',
            Self::Hebrew => 'א',
            Self::LowerSimplifiedChinese | Self::LowerTraditionalChinese => '一',
            Self::UpperSimplifiedChinese | Self::UpperTraditionalChinese => '壹',
            Self::HiraganaAiueo => 'あ',
            Self::HiraganaIroha => 'い',
            Self::KatakanaAiueo => 'ア',
            Self::KatakanaIroha => 'イ',
            Self::KoreanJamo => 'ㄱ',
            Self::KoreanSyllable => '가',
            Self::EasternArabic => '\u{0661}',
            Self::EasternArabicPersian => '\u{06F1}',
            Self::DevanagariNumber => '\u{0967}',
            Self::BengaliNumber => '\u{09E7}',
            Self::BengaliLetter => '\u{0995}',
            Self::CircledNumber => '①',
            Self::DoubleCircledNumber => '⓵',
        }
    }

    /// Apply the numbering to the given number.
    pub fn apply(self, n: u64) -> EcoString {
        match self {
            Self::Arabic => {
                numeric(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], n)
            }
            Self::LowerRoman => additive(
                &[
                    ("m̅", 1000000),
                    ("d̅", 500000),
                    ("c̅", 100000),
                    ("l̅", 50000),
                    ("x̅", 10000),
                    ("v̅", 5000),
                    ("i̅v̅", 4000),
                    ("m", 1000),
                    ("cm", 900),
                    ("d", 500),
                    ("cd", 400),
                    ("c", 100),
                    ("xc", 90),
                    ("l", 50),
                    ("xl", 40),
                    ("x", 10),
                    ("ix", 9),
                    ("v", 5),
                    ("iv", 4),
                    ("i", 1),
                    ("n", 0),
                ],
                n,
            ),
            Self::UpperRoman => additive(
                &[
                    ("M̅", 1000000),
                    ("D̅", 500000),
                    ("C̅", 100000),
                    ("L̅", 50000),
                    ("X̅", 10000),
                    ("V̅", 5000),
                    ("I̅V̅", 4000),
                    ("M", 1000),
                    ("CM", 900),
                    ("D", 500),
                    ("CD", 400),
                    ("C", 100),
                    ("XC", 90),
                    ("L", 50),
                    ("XL", 40),
                    ("X", 10),
                    ("IX", 9),
                    ("V", 5),
                    ("IV", 4),
                    ("I", 1),
                    ("N", 0),
                ],
                n,
            ),
            Self::LowerGreek => additive(
                &[
                    ("͵θ", 9000),
                    ("͵η", 8000),
                    ("͵ζ", 7000),
                    ("͵ϛ", 6000),
                    ("͵ε", 5000),
                    ("͵δ", 4000),
                    ("͵γ", 3000),
                    ("͵β", 2000),
                    ("͵α", 1000),
                    ("ϡ", 900),
                    ("ω", 800),
                    ("ψ", 700),
                    ("χ", 600),
                    ("φ", 500),
                    ("υ", 400),
                    ("τ", 300),
                    ("σ", 200),
                    ("ρ", 100),
                    ("ϟ", 90),
                    ("π", 80),
                    ("ο", 70),
                    ("ξ", 60),
                    ("ν", 50),
                    ("μ", 40),
                    ("λ", 30),
                    ("κ", 20),
                    ("ι", 10),
                    ("θ", 9),
                    ("η", 8),
                    ("ζ", 7),
                    ("ϛ", 6),
                    ("ε", 5),
                    ("δ", 4),
                    ("γ", 3),
                    ("β", 2),
                    ("α", 1),
                    ("𐆊", 0),
                ],
                n,
            ),
            Self::UpperGreek => additive(
                &[
                    ("͵Θ", 9000),
                    ("͵Η", 8000),
                    ("͵Ζ", 7000),
                    ("͵Ϛ", 6000),
                    ("͵Ε", 5000),
                    ("͵Δ", 4000),
                    ("͵Γ", 3000),
                    ("͵Β", 2000),
                    ("͵Α", 1000),
                    ("Ϡ", 900),
                    ("Ω", 800),
                    ("Ψ", 700),
                    ("Χ", 600),
                    ("Φ", 500),
                    ("Υ", 400),
                    ("Τ", 300),
                    ("Σ", 200),
                    ("Ρ", 100),
                    ("Ϟ", 90),
                    ("Π", 80),
                    ("Ο", 70),
                    ("Ξ", 60),
                    ("Ν", 50),
                    ("Μ", 40),
                    ("Λ", 30),
                    ("Κ", 20),
                    ("Ι", 10),
                    ("Θ", 9),
                    ("Η", 8),
                    ("Ζ", 7),
                    ("Ϛ", 6),
                    ("Ε", 5),
                    ("Δ", 4),
                    ("Γ", 3),
                    ("Β", 2),
                    ("Α", 1),
                    ("𐆊", 0),
                ],
                n,
            ),
            Self::Hebrew => additive(
                &[
                    ("ת", 400),
                    ("ש", 300),
                    ("ר", 200),
                    ("ק", 100),
                    ("צ", 90),
                    ("פ", 80),
                    ("ע", 70),
                    ("ס", 60),
                    ("נ", 50),
                    ("מ", 40),
                    ("ל", 30),
                    ("כ", 20),
                    ("יט", 19),
                    ("יח", 18),
                    ("יז", 17),
                    ("טז", 16),
                    ("טו", 15),
                    ("י", 10),
                    ("ט", 9),
                    ("ח", 8),
                    ("ז", 7),
                    ("ו", 6),
                    ("ה", 5),
                    ("ד", 4),
                    ("ג", 3),
                    ("ב", 2),
                    ("א", 1),
                    ("-", 0),
                ],
                n,
            ),
            Self::LowerLatin => alphabetic(
                &[
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                ],
                n,
            ),
            Self::UpperLatin => alphabetic(
                &[
                    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                ],
                n,
            ),
            Self::HiraganaAiueo => alphabetic(
                &[
                    'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ',
                    'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
                    'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む',
                    'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ',
                    'を', 'ん',
                ],
                n,
            ),
            Self::HiraganaIroha => alphabetic(
                &[
                    'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と', 'ち', 'り', 'ぬ', 'る',
                    'を', 'わ', 'か', 'よ', 'た', 'れ', 'そ', 'つ', 'ね', 'な', 'ら',
                    'む', 'う', 'ゐ', 'の', 'お', 'く', 'や', 'ま', 'け', 'ふ', 'こ',
                    'え', 'て', 'あ', 'さ', 'き', 'ゆ', 'め', 'み', 'し', 'ゑ', 'ひ',
                    'も', 'せ', 'す',
                ],
                n,
            ),
            Self::KatakanaAiueo => alphabetic(
                &[
                    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ',
                    'シ', 'ス', 'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ',
                    'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ', 'ム',
                    'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ',
                    'ヲ', 'ン',
                ],
                n,
            ),
            Self::KatakanaIroha => alphabetic(
                &[
                    'イ', 'ロ', 'ハ', 'ニ', 'ホ', 'ヘ', 'ト', 'チ', 'リ', 'ヌ', 'ル',
                    'ヲ', 'ワ', 'カ', 'ヨ', 'タ', 'レ', 'ソ', 'ツ', 'ネ', 'ナ', 'ラ',
                    'ム', 'ウ', 'ヰ', 'ノ', 'オ', 'ク', 'ヤ', 'マ', 'ケ', 'フ', 'コ',
                    'エ', 'テ', 'ア', 'サ', 'キ', 'ユ', 'メ', 'ミ', 'シ', 'ヱ', 'ヒ',
                    'モ', 'セ', 'ス',
                ],
                n,
            ),
            Self::KoreanJamo => alphabetic(
                &[
                    'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ',
                    'ㅌ', 'ㅍ', 'ㅎ',
                ],
                n,
            ),
            Self::KoreanSyllable => alphabetic(
                &[
                    '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카',
                    '타', '파', '하',
                ],
                n,
            ),
            Self::BengaliLetter => alphabetic(
                &[
                    'ক', 'খ', 'গ', 'ঘ', 'ঙ', 'চ', 'ছ', 'জ', 'ঝ', 'ঞ', 'ট', 'ঠ', 'ড', 'ঢ',
                    'ণ', 'ত', 'থ', 'দ', 'ধ', 'ন', 'প', 'ফ', 'ব', 'ভ', 'ম', 'য', 'র', 'ল',
                    'শ', 'ষ', 'স', 'হ',
                ],
                n,
            ),
            Self::CircledNumber => fixed(
                &[
                    '⓪', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬',
                    '⑭', '⑮', '⑯', '⑰', '⑱', '⑲', '⑳', '㉑', '㉒', '㉓', '㉔', '㉕',
                    '㉖', '㉗', '㉘', '㉙', '㉚', '㉛', '㉜', '㉝', '㉞', '㉟', '㊱',
                    '㊲', '㊳', '㊴', '㊵', '㊶', '㊷', '㊸', '㊹', '㊺', '㊻', '㊼',
                    '㊽', '㊾', '㊿',
                ],
                n,
            ),
            Self::DoubleCircledNumber => {
                fixed(&['0', '⓵', '⓶', '⓷', '⓸', '⓹', '⓺', '⓻', '⓼', '⓽', '⓾'], n)
            }

            Self::LowerSimplifiedChinese => {
                u64_to_chinese(ChineseVariant::Simple, ChineseCase::Lower, n).into()
            }
            Self::UpperSimplifiedChinese => {
                u64_to_chinese(ChineseVariant::Simple, ChineseCase::Upper, n).into()
            }
            Self::LowerTraditionalChinese => {
                u64_to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, n).into()
            }
            Self::UpperTraditionalChinese => {
                u64_to_chinese(ChineseVariant::Traditional, ChineseCase::Upper, n).into()
            }

            Self::EasternArabic => {
                numeric(&['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'], n)
            }
            Self::EasternArabicPersian => {
                numeric(&['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'], n)
            }
            Self::DevanagariNumber => {
                numeric(&['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'], n)
            }
            Self::BengaliNumber => {
                numeric(&['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'], n)
            }
            Self::Symbol => symbolic(&['*', '†', '‡', '§', '¶', '‖'], n),
        }
    }
}

/// Stringify a number using symbols representing values. The decimal
/// representation of the number is recovered by summing over the values of the
/// symbols present.
///
/// Consider the situation where ['I': 1, 'IV': 4, 'V': 5],
///
/// ```text
/// 1 => 'I'
/// 2 => 'II'
/// 3 => 'III'
/// 4 => 'IV'
/// 5 => 'V'
/// 6 => 'VI'
/// 7 => 'VII'
/// 8 => 'VIII'
/// ```
///
/// where this is the start of the familiar Roman numeral system.
fn additive(symbols: &[(&str, u64)], mut n: u64) -> EcoString {
    if n == 0 {
        if let Some(&(symbol, 0)) = symbols.last() {
            return symbol.into();
        }
        return '0'.into();
    }

    let mut s = EcoString::new();
    for (symbol, weight) in symbols {
        if *weight == 0 || *weight > n {
            continue;
        }
        let reps = n / weight;
        for _ in 0..reps {
            s.push_str(symbol);
        }

        n -= weight * reps;
        if n == 0 {
            return s;
        }
    }
    s
}

/// Stringify a number using a base-n (where n is the number of provided
/// symbols) system without a zero symbol.
///
/// Consider the situation where ['A', 'B', 'C'] are the provided symbols,
///
/// ```text
/// 1 => 'A'
/// 2 => 'B'
/// 3 => 'C'
/// 4 => 'AA
/// 5 => 'AB'
/// 6 => 'AC'
/// 7 => 'BA'
/// ...
/// ```
///
/// This system is commonly used in spreadsheet software.
fn alphabetic(symbols: &[char], mut n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n == 0 {
        return '-'.into();
    }
    let mut s = EcoString::new();
    while n != 0 {
        n -= 1;
        s.push(symbols[(n % n_digits) as usize]);
        n /= n_digits;
    }
    s.chars().rev().collect()
}

/// Stringify a number using the symbols provided, defaulting to the arabic
/// representation when the number is greater than the number of symbols.
///
/// Consider the situation where ['0', 'A', 'B', 'C'] are the provided symbols,
///
/// ```text
/// 0 => '0'
/// 1 => 'A'
/// 2 => 'B'
/// 3 => 'C'
/// 4 => '4'
/// ...
/// n => 'n'
/// ```
fn fixed(symbols: &[char], n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n < n_digits {
        return symbols[(n) as usize].into();
    }
    eco_format!("{n}")
}

/// Stringify a number using a base-n (where n is the number of provided
/// symbols) system with a zero symbol.
///
/// Consider the situation where ['0', '1', '2'] are the provided symbols,
///
/// ```text
/// 0 => '0'
/// 1 => '1'
/// 2 => '2'
/// 3 => '10'
/// 4 => '11'
/// 5 => '12'
/// 6 => '20'
/// ...
/// ```
///
/// which is the familiar trinary counting system.
fn numeric(symbols: &[char], mut n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n == 0 {
        return symbols[0].into();
    }
    let mut s = EcoString::new();
    while n != 0 {
        s.push(symbols[(n % n_digits) as usize]);
        n /= n_digits;
    }
    s.chars().rev().collect()
}

/// Stringify a number using repeating symbols.
///
/// Consider the situation where ['A', 'B', 'C'] are the provided symbols,
///
/// ```text
/// 0 => '-'
/// 1 => 'A'
/// 2 => 'B'
/// 3 => 'C'
/// 4 => 'AA'
/// 5 => 'BB'
/// 6 => 'CC'
/// 7 => 'AAA'
/// ...
/// ```
fn symbolic(symbols: &[char], n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n == 0 {
        return '-'.into();
    }
    EcoString::from(symbols[((n - 1) % n_digits) as usize])
        .repeat((n.div_ceil(n_digits)) as usize)
}
