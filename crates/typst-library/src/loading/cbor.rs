use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Bytes, Value, func, scope};
use crate::loading::{DataSource, Load};

<<<<<<< HEAD
/// CBORファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なCBORによるシリアル化データが含まれていなければなりません。
/// CBORの値は、[下の表](#conversion)に示す対応するTypstの値に変換されます。
///
/// この関数は辞書、配列、あるいはCBORファイルの内容に応じた別のCBORデータ型を返します。
///
/// # 変換の詳細 { #conversion }
///
/// | CBORの値 | Typstへの変換先 |
/// | -------- | -------------- |
/// | integer  | [`int`] または [`float`] |
/// | bytes    | [`bytes`]      |
/// | float    | [`float`]      |
/// | text     | [`str`]        |
/// | bool     | [`bool`]       |
/// | null     | `{none}`       |
/// | array    | [`array`]      |
/// | map      | [`dictionary`] |
///
/// | Typstの値                            | CBORへの変換先                       |
/// | ------------------------------------- | ------------------------------------ |
/// | CBORから変換できる型                  | 対応するCBOR値                       |
/// | [`symbol`]                            | text                                 |
/// | [`content`]                           | contentを記述するマップ              |
/// | その他の型（[`length`]など）          | [`repr`]経由の文字列                 |
///
/// ## 注意事項
/// - 2<sup>63</sup>-1より大きい（または-2<sup>63</sup>より小さい）整数は
///   浮動小数点数に変換されるため、近似値になる可能性があります。
///
/// - CBORタグはサポートされず、エラーになります。
///
/// - `repr`関数は[デバッグ目的のみ]($repr/#debugging-only)で、
///   出力の安定性はTypstのバージョン間で保証されません。
#[func(scope, title = "CBOR")]
pub fn cbor(
    engine: &mut Engine,
    /// CBORファイルへの[パス]($syntax/#paths)、または生のCBORバイト列。
=======
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    ciborium::from_reader(loaded.data.as_slice())
        .map_err(|err| eco_format!("failed to parse CBOR ({err})"))
        .at(source.span)
}

#[scope]
impl cbor {
<<<<<<< HEAD
    /// CBORバイト列から構造化データを読み込む。
    #[func(title = "Decode CBOR")]
    #[deprecated(
        message = "`cbor.decode`は非推奨です。代わりにバイト列を直接`cbor`に渡してください。",
=======
    /// Reads structured data from CBOR bytes.
    #[func(title = "Decode CBOR")]
    #[deprecated(
        message = "`cbor.decode` is deprecated, directly pass bytes to `cbor` instead",
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
<<<<<<< HEAD
        /// CBORデータ。
=======
        /// CBOR data.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
