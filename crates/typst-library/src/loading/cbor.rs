use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, SourceResult};
use crate::engine::Engine;
<<<<<<< HEAD
use crate::foundations::{func, scope, Bytes, Value};
use crate::loading::{DataSource, Load};

/// CBORファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なCBORによるシリアル化データが含まれていなければなりません。
/// マッピングはTypstの辞書に変換され、シーケンスはTypstの配列に変換されます。
/// 文字列やブール値はTypstの対応する型に変換され、
/// ヌル値（`null`、`~`、または空の``）は`{none}`に、
/// 数値は整数値であれば整数型に、
/// そうでなければ浮動小数点数型に変換されます。
///
/// 2<sup>63</sup>-1より大きな整数は浮動小数点数に変換されるため、
/// 近似値になる可能性があることに留意してください。
#[func(scope, title = "CBOR")]
pub fn cbor(
    engine: &mut Engine,
    /// CBORファイルの[パス]($syntax/#paths)、または生のCBORバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let data = source.load(engine.world)?;
    ciborium::from_reader(data.as_slice())
=======
use crate::foundations::{Bytes, Value, func, scope};
use crate::loading::{DataSource, Load};

/// Reads structured data from a CBOR file.
///
/// The file must contain a valid CBOR serialization. The CBOR values will be
/// converted into corresponding Typst values as listed in the
/// [table below](#conversion).
///
/// The function returns a dictionary, an array or, depending on the CBOR file,
/// another CBOR data type.
///
/// # Conversion details { #conversion }
///
/// | CBOR value | Converted into Typst   |
/// | ---------- | ---------------------- |
/// | integer    | [`int`] (or [`float`]) |
/// | bytes      | [`bytes`]              |
/// | float      | [`float`]              |
/// | text       | [`str`]                |
/// | bool       | [`bool`]               |
/// | null       | `{none}`               |
/// | array      | [`array`]              |
/// | map        | [`dictionary`]         |
///
/// | Typst value                           | Converted into CBOR          |
/// | ------------------------------------- | ---------------------------- |
/// | types that can be converted from CBOR | corresponding CBOR value     |
/// | [`symbol`]                            | text                         |
/// | [`content`]                           | a map describing the content |
/// | other types ([`length`], etc.)        | text via [`repr`]            |
///
/// ## Notes
///
/// - Be aware that CBOR integers larger than 2<sup>63</sup>-1 or smaller than
///   -2<sup>63</sup> will be converted to floating point numbers, which may
///   result in an approximative value.
///
/// - CBOR tags are not supported, and an error will be thrown.
///
/// - The `repr` function is [for debugging purposes only]($repr/#debugging-only),
///   and its output is not guaranteed to be stable across Typst versions.
#[func(scope, title = "CBOR")]
pub fn cbor(
    engine: &mut Engine,
    /// A [path]($syntax/#paths) to a CBOR file or raw CBOR bytes.
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    ciborium::from_reader(loaded.data.as_slice())
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        .map_err(|err| eco_format!("failed to parse CBOR ({err})"))
        .at(source.span)
}

#[scope]
impl cbor {
<<<<<<< HEAD
    /// CBORバイト列から構造化データを読み込む。
    #[func(title = "Decode CBOR")]
    #[deprecated = "`cbor.decode`は非推奨です。代わりにバイト列を直接`cbor`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// CBORデータ。
=======
    /// Reads structured data from CBOR bytes.
    #[func(title = "Decode CBOR")]
    #[deprecated(
        message = "`cbor.decode` is deprecated, directly pass bytes to `cbor` instead",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// CBOR data.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        data: Spanned<Bytes>,
    ) -> SourceResult<Value> {
        cbor(engine, data.map(DataSource::Bytes))
    }

<<<<<<< HEAD
    /// 構造化データをCBORバイト列にエンコードする。
    #[func(title = "Encode CBOR")]
    pub fn encode(
        /// エンコード対象の値。
=======
    /// Encode structured data into CBOR bytes.
    #[func(title = "Encode CBOR")]
    pub fn encode(
        /// Value to be encoded.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        value: Spanned<Value>,
    ) -> SourceResult<Bytes> {
        let Spanned { v: value, span } = value;
        let mut res = Vec::new();
        ciborium::into_writer(&value, &mut res)
            .map(|_| Bytes::new(res))
            .map_err(|err| eco_format!("failed to encode value as CBOR ({err})"))
            .at(span)
    }
}
