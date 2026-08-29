use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, LineCol, LoadError, LoadedWithin, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Str, Value, func, scope};
use crate::loading::{DataSource, Load, Readable};

<<<<<<< HEAD
/// JSONファイルから構造化データを読み込む。
///
/// 読み込むファイルにはオブジェクトや配列などの有効なJSON値が含まれていなければなりません。
/// JSONの値は、[下の表](#conversion)に示す対応するTypstの値に変換されます。
///
/// この関数は辞書、配列、あるいはJSONファイルの内容に応じた別のJSONデータ型を返します。
///
/// この例におけるJSONファイルは、
/// `temperature`、`unit`、および`weather`というキーを持つオブジェクトを含んでいます。
///
/// # 例
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
///
<<<<<<< HEAD
/// # 変換の詳細 { #conversion }
///
/// | JSONの値 | Typstへの変換先 |
/// | -------- | -------------- |
/// | `null`   | `{none}`       |
/// | bool     | [`bool`]       |
/// | number   | [`float`] または [`int`] |
/// | string   | [`str`]        |
/// | array    | [`array`]      |
/// | object   | [`dictionary`] |
///
/// | Typstの値                            | JSONへの変換先                       |
/// | ------------------------------------- | ------------------------------------ |
/// | JSONから変換できる型                  | 対応するJSON値                       |
/// | [`bytes`]                             | [`repr`]経由の文字列                 |
/// | [`symbol`]                            | 文字列                               |
/// | [`content`]                           | コンテンツを記述するオブジェクト      |
/// | その他の型（[`length`]など）          | [`repr`]経由の文字列                 |
///
/// ## 注意事項
/// - 多くの場合、JSONの数値は整数か小数かに応じて`float`または`int`に変換されます。
///   ただし、2<sup>63</sup>-1より大きい（または-2<sup>63</sup>より小さい）整数は
///   浮動小数点数に変換されるため、近似値になる可能性があります。
///
/// - `bytes`は性能と可読性のためJSON配列としてはエンコードされません。
///   バイナリデータには[`cbor.encode`]を検討してください。
///
/// - `repr`関数は[デバッグ目的のみ]($repr/#debugging-only)で、
///   出力の安定性はTypstのバージョン間で保証されません。
#[func(scope, title = "JSON")]
pub fn json(
    engine: &mut Engine,
    /// JSONファイルの[パス]($syntax/#paths)、または生のJSONバイト列。
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    serde_json::from_slice(loaded.data.as_slice())
        .map_err(|err| {
            let pos = LineCol::one_based(err.line(), err.column());
            LoadError::new(pos, "failed to parse JSON", err)
        })
        .within(&loaded)
}

#[scope]
impl json {
<<<<<<< HEAD
    /// JSONの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode JSON")]
    #[deprecated(
        message = "`json.decode`は非推奨です。代わりにバイト列を直接`json`に渡してください。",
=======
    /// Reads structured data from a JSON string/bytes.
    #[func(title = "Decode JSON")]
    #[deprecated(
        message = "`json.decode` is deprecated, directly pass bytes to `json` instead",
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
<<<<<<< HEAD
        /// JSONデータ。
=======
        /// JSON data.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
