use ecow::EcoString;
use typst_syntax::is_newline;
use unicode_segmentation::UnicodeSegmentation;

use crate::diag::{bail, HintedStrResult, StrResult};
use crate::foundations::{
    array, cast, dict, elem, Array, Dict, FromValue, Packed, PlainText, Smart, Str,
};
use crate::layout::Dir;
use crate::text::{Lang, Region};

/// 言語を認識し、コンテキストに反応する引用符。
///
/// アクティブな[テキストの言語設定]($text.lang)に基づいて適切な開き引用符か閉じ引用符に自動的に変更します。
///
/// # 例
/// ```example
/// "This is in quotes."
///
/// #set text(lang: "de")
/// "Das ist in Anführungszeichen."
///
/// #set text(lang: "fr")
/// "C'est entre guillemets."
/// ```
///
/// # 構文
/// この関数は専用の構文もあります。
/// 通常の引用記号（`'`と`"`）です。
/// Typstはそれらを自動的にスマートクォートとして扱います。
#[elem(name = "smartquote", PlainText)]
pub struct SmartQuoteElem {
    /// 二重引用符にすべきかどうか。
    #[default(true)]
    pub double: bool,

    /// スマートクォートを有効化するかどうか。
    ///
    /// 単一引用符の場合は、バックスラッシュでエスケープしても無効化できます。
    ///
    /// ```example
    /// #set smartquote(enabled: false)
    ///
    /// These are "dumb" quotes.
    /// ```
    #[default(true)]
    pub enabled: bool,

    /// 代替引用符を使用するかどうか。
    ///
    /// 代替引用符を持たない言語に対してや、明示的に引用符が設定されている場合には、何もしません。
    ///
    /// ```example
    /// #set text(lang: "de")
    /// #set smartquote(alternative: true)
    ///
    /// "Das ist in anderen Anführungszeichen."
    /// ```
    #[default(false)]
    pub alternative: bool,

    /// 使用する引用符。
    ///
    /// - `{auto}`に設定された場合、[テキストの言語]($text.lang)に対して適切な単一引用符が使用されます。これがデフォルトです。
    /// - カスタム引用符として文字列、配列、辞書のいずれかを渡せます。
    ///   - [文字列]($str): 開き二重引用符と閉じ二重引用符の2文字からなる文字列（ここの文字はUnicodeグラフェムクラスターを指します）
    ///   - [配列]($array): 開き二重引用符と閉じ二重引用符を持つ配列
    ///   - [辞書]($dictionary): doubleやsingleをキーとして引用符を指定する辞書。その値は`{auto}`、文字列、配列のいずれかで指定します。
    ///
    /// ```example
    /// #set text(lang: "de")
    /// 'Das sind normale Anführungszeichen.'
    ///
    /// #set smartquote(quotes: "()")
    /// "Das sind eigene Anführungszeichen."
    ///
    /// #set smartquote(quotes: (single: ("[[", "]]"),  double: auto))
    /// 'Das sind eigene Anführungszeichen.'
    /// ```
    #[borrowed]
    pub quotes: Smart<SmartQuoteDict>,
}

impl PlainText for Packed<SmartQuoteElem> {
    fn plain_text(&self, text: &mut EcoString) {
        if self.double.unwrap_or(true) {
            text.push_str("\"");
        } else {
            text.push_str("'");
        }
    }
}

/// A smart quote substitutor with zero lookahead.
#[derive(Debug, Clone)]
pub struct SmartQuoter {
    /// The amount of quotes that have been opened.
    depth: u8,
    /// Each bit indicates whether the quote at this nesting depth is a double.
    /// Maximum supported depth is thus 32.
    kinds: u32,
}

impl SmartQuoter {
    /// Start quoting.
    pub fn new() -> Self {
        Self { depth: 0, kinds: 0 }
    }

    /// Determine which smart quote to substitute given this quoter's nesting
    /// state and the character immediately preceding the quote.
    pub fn quote<'a>(
        &mut self,
        before: Option<char>,
        quotes: &SmartQuotes<'a>,
        double: bool,
    ) -> &'a str {
        let opened = self.top();
        let before = before.unwrap_or(' ');

        // If we are after a number and haven't most recently opened a quote of
        // this kind, produce a prime. Otherwise, we prefer a closing quote.
        if before.is_numeric() && opened != Some(double) {
            return if double { "″" } else { "′" };
        }

        // If we have a single smart quote, didn't recently open a single
        // quotation, and are after an alphabetic char or an object (e.g. a
        // math equation), interpret this as an apostrophe.
        if !double
            && opened != Some(false)
            && (before.is_alphabetic() || before == '\u{FFFC}')
        {
            return "’";
        }

        // If the most recently opened quotation is of this kind and the
        // previous char does not indicate a nested quotation, close it.
        if opened == Some(double)
            && !before.is_whitespace()
            && !is_newline(before)
            && !is_opening_bracket(before)
        {
            self.pop();
            return quotes.close(double);
        }

        // Otherwise, open a new the quotation.
        self.push(double);
        quotes.open(double)
    }

    /// The top of our quotation stack. Returns `Some(double)` for the most
    /// recently opened quote or `None` if we didn't open one.
    fn top(&self) -> Option<bool> {
        self.depth.checked_sub(1).map(|i| (self.kinds >> i) & 1 == 1)
    }

    /// Push onto the quotation stack.
    fn push(&mut self, double: bool) {
        if self.depth < 32 {
            self.kinds |= (double as u32) << self.depth;
            self.depth += 1;
        }
    }

    /// Pop from the quotation stack.
    fn pop(&mut self) {
        self.depth -= 1;
        self.kinds &= (1 << self.depth) - 1;
    }
}

impl Default for SmartQuoter {
    fn default() -> Self {
        Self::new()
    }
}

/// Whether the character is an opening bracket, parenthesis, or brace.
fn is_opening_bracket(c: char) -> bool {
    matches!(c, '(' | '{' | '[')
}

/// Decides which quotes to substitute smart quotes with.
pub struct SmartQuotes<'s> {
    /// The opening single quote.
    pub single_open: &'s str,
    /// The closing single quote.
    pub single_close: &'s str,
    /// The opening double quote.
    pub double_open: &'s str,
    /// The closing double quote.
    pub double_close: &'s str,
}

impl<'s> SmartQuotes<'s> {
    /// Create a new `Quotes` struct with the given quotes, optionally falling
    /// back to the defaults for a language and region.
    ///
    /// The language should be specified as an all-lowercase ISO 639-1 code, the
    /// region as an all-uppercase ISO 3166-alpha2 code.
    ///
    /// Currently, the supported languages are: English, Czech, Danish, German,
    /// Swiss / Liechtensteinian German, Estonian, Icelandic, Italian, Latin,
    /// Lithuanian, Latvian, Slovak, Slovenian, Spanish, Bosnian, Finnish,
    /// Swedish, French, Swiss French, Hungarian, Polish, Romanian, Japanese,
    /// Traditional Chinese, Russian, Norwegian, Hebrew and Croatian.
    ///
    /// For unknown languages, the English quotes are used as fallback.
    pub fn get(
        quotes: &'s Smart<SmartQuoteDict>,
        lang: Lang,
        region: Option<Region>,
        alternative: bool,
    ) -> Self {
        let region = region.as_ref().map(Region::as_str);

        let default = ("‘", "’", "“", "”");
        let low_high = ("‚", "‘", "„", "“");

        let (single_open, single_close, double_open, double_close) = match lang.as_str() {
            "de" if matches!(region, Some("CH" | "LI")) => match alternative {
                false => ("‹", "›", "«", "»"),
                true => low_high,
            },
            "fr" if matches!(region, Some("CH")) => match alternative {
                false => ("‹\u{202F}", "\u{202F}›", "«\u{202F}", "\u{202F}»"),
                true => default,
            },
            "cs" | "da" | "de" | "sk" | "sl" if alternative => ("›", "‹", "»", "«"),
            "cs" | "de" | "et" | "is" | "lt" | "lv" | "sk" | "sl" => low_high,
            "da" => ("‘", "’", "“", "”"),
            "fr" | "ru" if alternative => default,
            "fr" => ("‹\u{00A0}", "\u{00A0}›", "«\u{00A0}", "\u{00A0}»"),
            "fi" | "sv" if alternative => ("’", "’", "»", "»"),
            "bs" | "fi" | "sv" => ("’", "’", "”", "”"),
            "it" if alternative => default,
            "la" if alternative => ("“", "”", "«\u{202F}", "\u{202F}»"),
            "it" | "la" => ("“", "”", "«", "»"),
            "es" if matches!(region, Some("ES") | None) => ("“", "”", "«", "»"),
            "hu" | "pl" | "ro" => ("’", "’", "„", "”"),
            "no" | "nb" | "nn" if alternative => low_high,
            "ru" | "no" | "nb" | "nn" | "uk" => ("’", "’", "«", "»"),
            "el" => ("‘", "’", "«", "»"),
            "he" => ("’", "’", "”", "”"),
            "hr" => ("‘", "’", "„", "”"),
            "bg" => ("’", "’", "„", "“"),
            _ if lang.dir() == Dir::RTL => ("’", "‘", "”", "“"),
            _ => default,
        };

        fn inner_or_default<'s>(
            quotes: Smart<&'s SmartQuoteDict>,
            f: impl FnOnce(&'s SmartQuoteDict) -> Smart<&'s SmartQuoteSet>,
            default: [&'s str; 2],
        ) -> [&'s str; 2] {
            match quotes.and_then(f) {
                Smart::Auto => default,
                Smart::Custom(SmartQuoteSet { open, close }) => {
                    [open, close].map(|s| s.as_str())
                }
            }
        }

        let quotes = quotes.as_ref();
        let [single_open, single_close] =
            inner_or_default(quotes, |q| q.single.as_ref(), [single_open, single_close]);
        let [double_open, double_close] =
            inner_or_default(quotes, |q| q.double.as_ref(), [double_open, double_close]);

        Self {
            single_open,
            single_close,
            double_open,
            double_close,
        }
    }

    /// The opening quote.
    pub fn open(&self, double: bool) -> &'s str {
        if double {
            self.double_open
        } else {
            self.single_open
        }
    }

    /// The closing quote.
    pub fn close(&self, double: bool) -> &'s str {
        if double {
            self.double_close
        } else {
            self.single_close
        }
    }
}

/// An opening and closing quote.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SmartQuoteSet {
    open: EcoString,
    close: EcoString,
}

cast! {
    SmartQuoteSet,
    self => array![self.open, self.close].into_value(),
    value: Array => {
        let [open, close] = array_to_set(value)?;
        Self { open, close }
    },
    value: Str => {
        let [open, close] = str_to_set(value.as_str())?;
        Self { open, close }
    },
}

fn str_to_set(value: &str) -> StrResult<[EcoString; 2]> {
    let mut iter = value.graphemes(true);
    match (iter.next(), iter.next(), iter.next()) {
        (Some(open), Some(close), None) => Ok([open.into(), close.into()]),
        _ => {
            let count = value.graphemes(true).count();
            bail!(
                "expected 2 characters, found {count} character{}",
                if count > 1 { "s" } else { "" }
            );
        }
    }
}

fn array_to_set(value: Array) -> HintedStrResult<[EcoString; 2]> {
    let value = value.as_slice();
    if value.len() != 2 {
        bail!(
            "expected 2 quotes, found {} quote{}",
            value.len(),
            if value.len() > 1 { "s" } else { "" }
        );
    }

    let open: EcoString = value[0].clone().cast()?;
    let close: EcoString = value[1].clone().cast()?;

    Ok([open, close])
}

/// A dict of single and double quotes.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SmartQuoteDict {
    double: Smart<SmartQuoteSet>,
    single: Smart<SmartQuoteSet>,
}

cast! {
    SmartQuoteDict,
    self => dict! { "double" => self.double, "single" => self.single }.into_value(),
    mut value: Dict => {
        let keys = ["double", "single"];

        let double = value
            .take("double")
            .ok()
            .map(FromValue::from_value)
            .transpose()?
            .unwrap_or(Smart::Auto);
        let single = value
            .take("single")
            .ok()
            .map(FromValue::from_value)
            .transpose()?
            .unwrap_or(Smart::Auto);

        value.finish(&keys)?;

        Self { single, double }
    },
    value: SmartQuoteSet => Self {
        double: Smart::Custom(value),
        single: Smart::Auto,
    },
}
