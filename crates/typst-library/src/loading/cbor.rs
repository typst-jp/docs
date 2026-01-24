use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Bytes, Value, func, scope};
use crate::loading::{DataSource, Load};

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
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    ciborium::from_reader(loaded.data.as_slice())
        .map_err(|err| eco_format!("failed to parse CBOR ({err})"))
        .at(source.span)
}

#[scope]
impl cbor {
    /// CBORバイト列から構造化データを読み込む。
    #[func(title = "Decode CBOR")]
    #[deprecated(
        message = "`cbor.decode`は非推奨です。代わりにバイト列を直接`cbor`に渡してください。",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// CBORデータ。
        data: Spanned<Bytes>,
    ) -> SourceResult<Value> {
        cbor(engine, data.map(DataSource::Bytes))
    }

    /// 構造化データをCBORバイト列にエンコードする。
    #[func(title = "Encode CBOR")]
    pub fn encode(
        /// エンコード対象の値。
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
