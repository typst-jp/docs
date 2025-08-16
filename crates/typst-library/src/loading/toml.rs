use ecow::{eco_format, EcoString};
use typst_syntax::{is_newline, Spanned};

use crate::diag::{At, FileError, SourceResult};
use crate::engine::Engine;
use crate::foundations::{func, scope, Str, Value};
use crate::loading::{DataSource, Load, Readable};

/// TOMLファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なTOMLテーブルが含まれていなければなりません。
/// TOMLテーブルはTypstの辞書に変換され、
/// TOML配列はTypstの配列に変換されます。
/// 文字列、ブール値、日時はTypstの対応する型に変換され、
/// 数値は整数値であれば整数型に、そうでなければ浮動小数点数型に変換されます。
///
/// この例におけるTOMLファイルは、
/// `title`、`version`、および`authors`のキーを持つテーブルで構成されています。
///
/// # 例
/// ```example
/// #let details = toml("details.toml")
///
/// Title: #details.title \
/// Version: #details.version \
/// Authors: #(details.authors
///   .join(", ", last: " and "))
/// ```
#[func(scope, title = "TOML")]
pub fn toml(
    engine: &mut Engine,
    /// TOMLファイルの[パス]($syntax/#paths)、または生のTOMLバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let data = source.load(engine.world)?;
    let raw = data.as_str().map_err(FileError::from).at(source.span)?;
    ::toml::from_str(raw)
        .map_err(|err| format_toml_error(err, raw))
        .at(source.span)
}

#[scope]
impl toml {
    /// TOMLの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode TOML")]
    #[deprecated = "`toml.decode`は非推奨です。代わりにバイト列を直接`toml`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// TOMLデータ。
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        toml(engine, data.map(Readable::into_source))
    }

    /// 構造化データをTOML文字列にエンコードする。
    #[func(title = "Encode TOML")]
    pub fn encode(
        /// エンコード対象の値。
        value: Spanned<Value>,
        /// TOMLを整形表示するかどうか。
        #[named]
        #[default(true)]
        pretty: bool,
    ) -> SourceResult<Str> {
        let Spanned { v: value, span } = value;
        if pretty { ::toml::to_string_pretty(&value) } else { ::toml::to_string(&value) }
            .map(|v| v.into())
            .map_err(|err| eco_format!("failed to encode value as TOML ({err})"))
            .at(span)
    }
}

/// Format the user-facing TOML error message.
fn format_toml_error(error: ::toml::de::Error, raw: &str) -> EcoString {
    if let Some(head) = error.span().and_then(|range| raw.get(..range.start)) {
        let line = head.lines().count();
        let column = 1 + head.chars().rev().take_while(|&c| !is_newline(c)).count();
        eco_format!(
            "failed to parse TOML ({} at line {line} column {column})",
            error.message(),
        )
    } else {
        eco_format!("failed to parse TOML ({})", error.message())
    }
}
