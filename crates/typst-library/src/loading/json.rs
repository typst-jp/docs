use ecow::eco_format;
use typst_syntax::Spanned;

<<<<<<< HEAD
use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{func, scope, Str, Value};
use crate::loading::{DataSource, Load, Readable};

/// JSONファイルから構造化データを読み込む。
///
/// 読み込むファイルにはオブジェクトや配列などの有効なJSON値が含まれていなければなりません。
/// JSONオブジェクトはTypstの辞書に変換され、
/// JSON配列はTypstの配列に変換されます。
/// 文字列やブール値はTypstの対応する型に変換され、`null`は`{none}`に、
/// 数値は整数値であれば整数型に、
/// そうでなければ浮動小数点数型に変換されます。
///
/// 2<sup>63</sup>-1より大きな整数は浮動小数点数に変換されるため、
/// 近似値になる可能性があることに留意してください。
///
/// この関数は、辞書、配列、
/// あるいはJSONファイルの内容に応じてその他のJSONデータ型を返します。
///
/// この例におけるJSONファイルは、
/// `temperature`、`unit`、および`weather`というキーを持つオブジェクトを含んでいます。
///
/// # 例
=======
use crate::diag::{At, LineCol, LoadError, LoadedWithin, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Str, Value, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// Reads structured data from a JSON file.
///
/// The file must contain a valid JSON value, such as object or array. The JSON
/// values will be converted into corresponding Typst values as listed in the
/// [table below](#conversion).
///
/// The function returns a dictionary, an array or, depending on the JSON file,
/// another JSON data type.
///
/// The JSON files in the example contain objects with the keys `temperature`,
/// `unit`, and `weather`.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #let forecast(day) = block[
///   #box(square(
///     width: 2cm,
///     inset: 8pt,
///     fill: if day.weather == "sunny" {
///       yellow
///     } else {
///       aqua
///     },
///     align(
///       bottom + right,
///       strong(day.weather),
///     ),
///   ))
///   #h(6pt)
///   #set text(22pt, baseline: -8pt)
///   #day.temperature °#day.unit
/// ]
///
/// #forecast(json("monday.json"))
/// #forecast(json("tuesday.json"))
/// ```
<<<<<<< HEAD
#[func(scope, title = "JSON")]
pub fn json(
    engine: &mut Engine,
    /// JSONファイルの[パス]($syntax/#paths)、または生のJSONバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let data = source.load(engine.world)?;
    serde_json::from_slice(data.as_slice())
        .map_err(|err| eco_format!("failed to parse JSON ({err})"))
        .at(source.span)
=======
///
/// # Conversion details { #conversion }
///
/// | JSON value | Converted into Typst |
/// | ---------- | -------------------- |
/// | `null`     | `{none}`             |
/// | bool       | [`bool`]             |
/// | number     | [`float`] or [`int`] |
/// | string     | [`str`]              |
/// | array      | [`array`]            |
/// | object     | [`dictionary`]       |
///
/// | Typst value                           | Converted into JSON              |
/// | ------------------------------------- | -------------------------------- |
/// | types that can be converted from JSON | corresponding JSON value         |
/// | [`bytes`]                             | string via [`repr`]              |
/// | [`symbol`]                            | string                           |
/// | [`content`]                           | an object describing the content |
/// | other types ([`length`], etc.)        | string via [`repr`]              |
///
/// ## Notes
/// - In most cases, JSON numbers will be converted to floats or integers
///   depending on whether they are whole numbers. However, be aware that
///   integers larger than 2<sup>63</sup>-1 or smaller than -2<sup>63</sup> will
///   be converted to floating-point numbers, which may result in an
///   approximative value.
///
/// - Bytes are not encoded as JSON arrays for performance and readability
///   reasons. Consider using [`cbor.encode`] for binary data.
///
/// - The `repr` function is [for debugging purposes only]($repr/#debugging-only),
///   and its output is not guaranteed to be stable across Typst versions.
#[func(scope, title = "JSON")]
pub fn json(
    engine: &mut Engine,
    /// A [path]($syntax/#paths) to a JSON file or raw JSON bytes.
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    serde_json::from_slice(loaded.data.as_slice())
        .map_err(|err| {
            let pos = LineCol::one_based(err.line(), err.column());
            LoadError::new(pos, "failed to parse JSON", err)
        })
        .within(&loaded)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

#[scope]
impl json {
<<<<<<< HEAD
    /// JSONの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode JSON")]
    #[deprecated = "`json.decode`は非推奨です。代わりにバイト列を直接`json`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// JSONデータ。
=======
    /// Reads structured data from a JSON string/bytes.
    #[func(title = "Decode JSON")]
    #[deprecated(
        message = "`json.decode` is deprecated, directly pass bytes to `json` instead",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// JSON data.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        json(engine, data.map(Readable::into_source))
    }

<<<<<<< HEAD
    /// 構造化データをJSON文字列にエンコードする。
    #[func(title = "Encode JSON")]
    pub fn encode(
        /// エンコード対象の値。
        value: Spanned<Value>,
        /// JSONを改行およびインデント付きで整形表示するかどうか。
=======
    /// Encodes structured data into a JSON string.
    #[func(title = "Encode JSON")]
    pub fn encode(
        /// Value to be encoded.
        value: Spanned<Value>,
        /// Whether to pretty print the JSON with newlines and indentation.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[named]
        #[default(true)]
        pretty: bool,
    ) -> SourceResult<Str> {
        let Spanned { v: value, span } = value;
        if pretty {
            serde_json::to_string_pretty(&value)
        } else {
            serde_json::to_string(&value)
        }
        .map(|v| v.into())
        .map_err(|err| eco_format!("failed to encode value as JSON ({err})"))
        .at(span)
    }
}
