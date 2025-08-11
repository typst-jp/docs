use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{func, scope, Bytes, Value};
use crate::loading::{DataSource, Load};

/// CBORファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なCBORによるシリアル化データが含まれていなければなりません。
/// マッピングはTypstの辞書に変換され、シーケンスはTypstの配列に変換されます。
/// 文字列やブール値はTypstの対応する値に変換され、
/// ヌル値（`null`、`~`、または空の``）は`{none}`に、
/// 数値は整数値であれば整数型に、
/// そうでなければ浮動小数点数型に変換されます。
///
/// 2<sup>63</sup>-1より大きな整数は浮動小数点数に変換されるため、
/// 近似値になる可能性があることに留意してください。
#[func(scope, title = "CBOR")]
pub fn cbor(
    engine: &mut Engine,
    /// CBORファイルへの[パス]($syntax/#paths)、または生のCBORバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let data = source.load(engine.world)?;
    ciborium::from_reader(data.as_slice())
        .map_err(|err| eco_format!("failed to parse CBOR ({err})"))
        .at(source.span)
}

#[scope]
impl cbor {
    /// CBORバイト列から構造化データを読み込む。
    #[func(title = "Decode CBOR")]
    #[deprecated = "`cbor.decode`は非推奨です。代わりにバイト列を直接`cbor`に渡してください。"]
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
