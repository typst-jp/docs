use typst_utils::singleton;

use crate::foundations::{elem, Content, NativeElement};

/// 改行の挿入。
///
/// 段落を次の行へ進めます。
/// 段落の終わりにある単一の改行は無視されますが、それより多くある場合は空行が作成されます。
///
/// # 例
/// ```example
/// *Date:* 26.12.2022 \
/// *Topic:* Infrastructure Test \
/// *Severity:* High \
/// ```
///
/// # 構文
/// この関数は専用の構文も持っています。
/// 改行を挿入するには、単にバックスラッシュと空白を書いてください。
/// これは常に両端揃えではない改行を作成します。
#[elem(title = "Line Break")]
pub struct LinebreakElem {
    /// 改行の前の行を両端揃えするかどうか。
    ///
    /// これは、Typstが両端揃えを行ったテキストよりも良い改行位置が見つかった場合に便利です。
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
