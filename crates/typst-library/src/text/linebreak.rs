use typst_utils::singleton;

<<<<<<< HEAD
use crate::foundations::{elem, Content, NativeElement};

/// 改行の挿入。
///
/// 段落を次の行へ進めます。
/// 段落の終わりにある単一の改行は無視されますが、それより多くある場合は空行が作成されます。
///
/// # 例
=======
use crate::foundations::{Content, NativeElement, elem};

/// Inserts a line break.
///
/// Advances the paragraph to the next line. A single trailing line break at the
/// end of a paragraph is ignored, but more than one creates additional empty
/// lines.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// *Date:* 26.12.2022 \
/// *Topic:* Infrastructure Test \
/// *Severity:* High \
/// ```
///
<<<<<<< HEAD
/// # 構文
/// この関数は専用の構文も持っています。
/// 改行を挿入するには、単にバックスラッシュと空白を書いてください。
/// これは常に両端揃えではない改行を作成します。
#[elem(title = "Line Break")]
pub struct LinebreakElem {
    /// 改行の前の行を両端揃えするかどうか。
    ///
    /// これは、Typstが両端揃えを行ったテキストよりも良い改行位置が見つかった場合に便利です。
=======
/// # Syntax
/// This function also has dedicated syntax: To insert a line break, simply write
/// a backslash followed by whitespace. This always creates an unjustified
/// break.
#[elem(title = "Line Break")]
pub struct LinebreakElem {
    /// Whether to justify the line before the break.
    ///
    /// This is useful if you found a better line break opportunity in your
    /// justified text than Typst did.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set par(justify: true)
    /// #let jb = linebreak(justify: true)
    ///
    /// I have manually tuned the #jb
    /// line breaks in this paragraph #jb
    /// for an _interesting_ result. #jb
    /// ```
    #[default(false)]
    pub justify: bool,
}

impl LinebreakElem {
    /// Get the globally shared linebreak element.
    pub fn shared() -> &'static Content {
        singleton!(Content, LinebreakElem::new().pack())
    }
}
