use ecow::eco_format;
use typst_syntax::Spanned;

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
}

#[scope]
impl json {
    /// JSONの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode JSON")]
    #[deprecated = "`json.decode`は非推奨です。代わりにバイト列を直接`json`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// JSONデータ。
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        json(engine, data.map(Readable::into_source))
    }

    /// 構造化データをJSON文字列にエンコードする。
    #[func(title = "Encode JSON")]
    pub fn encode(
        /// エンコード対象の値。
        value: Spanned<Value>,
        /// JSONを改行およびインデント付きで整形表示するかどうか。
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
