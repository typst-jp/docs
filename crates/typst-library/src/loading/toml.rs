<<<<<<< HEAD
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
=======
use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, LoadError, LoadedWithin, ReportPos, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Dict, Str, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// Reads structured data from a TOML file.
///
/// The file must contain a valid TOML table. The TOML values will be converted
/// into corresponding Typst values as listed in the [table below](#conversion).
///
/// The function returns a dictionary representing the TOML table.
///
/// The TOML file in the example consists of a table with the keys `title`,
/// `version`, and `authors`.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #let details = toml("details.toml")
///
/// Title: #details.title \
/// Version: #details.version \
/// Authors: #(details.authors
///   .join(", ", last: " and "))
/// ```
<<<<<<< HEAD
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
=======
///
/// # Conversion details { #conversion }
///
/// First of all, TOML documents are tables. Other values must be put in a table
/// to be encoded or decoded.
///
/// | TOML value | Converted into Typst |
/// | ---------- | -------------------- |
/// | string     | [`str`]              |
/// | integer    | [`int`]              |
/// | float      | [`float`]            |
/// | boolean    | [`bool`]             |
/// | datetime   | [`datetime`]         |
/// | array      | [`array`]            |
/// | table      | [`dictionary`]       |
///
/// | Typst value                           | Converted into TOML            |
/// | ------------------------------------- | ------------------------------ |
/// | types that can be converted from TOML | corresponding TOML value       |
/// | `{none}`                              | ignored                        |
/// | [`bytes`]                             | string via [`repr`]            |
/// | [`symbol`]                            | string                         |
/// | [`content`]                           | a table describing the content |
/// | other types ([`length`], etc.)        | string via [`repr`]            |
///
/// ## Notes
/// - Be aware that TOML integers larger than 2<sup>63</sup>-1 or smaller
///   than -2<sup>63</sup> cannot be represented losslessly in Typst, and an
///   error will be thrown according to the
///   [specification](https://toml.io/en/v1.0.0#integer).
///
/// - Bytes are not encoded as TOML arrays for performance and readability
///   reasons. Consider using [`cbor.encode`] for binary data.
///
/// - The `repr` function is [for debugging purposes only]($repr/#debugging-only),
///   and its output is not guaranteed to be stable across Typst versions.
#[func(scope, title = "TOML")]
pub fn toml(
    engine: &mut Engine,
    /// A [path]($syntax/#paths) to a TOML file or raw TOML bytes.
    source: Spanned<DataSource>,
) -> SourceResult<Dict> {
    let loaded = source.load(engine.world)?;
    let raw = loaded.data.as_str().within(&loaded)?;
    ::toml::from_str(raw).map_err(format_toml_error).within(&loaded)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

#[scope]
impl toml {
<<<<<<< HEAD
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
=======
    /// Reads structured data from a TOML string/bytes.
    #[func(title = "Decode TOML")]
    #[deprecated(
        message = "`toml.decode` is deprecated, directly pass bytes to `toml` instead",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// TOML data.
        data: Spanned<Readable>,
    ) -> SourceResult<Dict> {
        toml(engine, data.map(Readable::into_source))
    }

    /// Encodes structured data into a TOML string.
    #[func(title = "Encode TOML")]
    pub fn encode(
        /// Value to be encoded.
        ///
        /// TOML documents are tables. Therefore, only dictionaries are suitable.
        value: Spanned<Dict>,
        /// Whether to pretty-print the resulting TOML.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
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
=======
fn format_toml_error(error: ::toml::de::Error) -> LoadError {
    let pos = error.span().map(ReportPos::from).unwrap_or_default();
    LoadError::new(pos, "failed to parse TOML", error.message())
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}
