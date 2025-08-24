use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{func, scope, Str, Value};
use crate::loading::{DataSource, Load, Readable};

/// YAMLファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なYAMLオブジェクトまたは配列が含まれていなければなりません。
/// YAMLのマッピングはTypstの辞書に、
/// YAMLのシーケンスはTypstの配列に変換されます。
/// 文字列やブール値はTypstの対応する型に変換され、
/// ヌル値（`null`、`~`、または空の``）は`{none}`に、
/// 数値は整数値であれば整数型に、そうでなければ浮動小数点数型に変換されます。
/// カスタムYAMLタグは無視されますが、読み込まれた値はそのまま保持されます。
///
/// 2<sup>63</sup>-1より大きな整数は浮動小数点数に変換されるため、
/// 近似値になる可能性があることに留意してください。
///
/// この例におけるYAMLファイルには著者名をキーとするオブジェクトが含まれており、
/// それぞれの著者には`title`と`published`というキーを持つ
/// サブマッピングのシーケンスが含まれています。
///
/// # 例
/// ```example
/// #let bookshelf(contents) = {
///   for (author, works) in contents {
///     author
///     for work in works [
///       - #work.title (#work.published)
///     ]
///   }
/// }
///
/// #bookshelf(
///   yaml("scifi-authors.yaml")
/// )
/// ```
#[func(scope, title = "YAML")]
pub fn yaml(
    engine: &mut Engine,
    /// YAMLファイルの[パス]($syntax/#paths)、または生のYAMLバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let data = source.load(engine.world)?;
    serde_yaml::from_slice(data.as_slice())
        .map_err(|err| eco_format!("failed to parse YAML ({err})"))
        .at(source.span)
}

#[scope]
impl yaml {
    /// YAMLの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode YAML")]
    #[deprecated = "`yaml.decode`は非推奨です。代わりにバイト列を直接`yaml`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// YAMLデータ。
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        yaml(engine, data.map(Readable::into_source))
    }

    /// 構造化データをYAML文字列にエンコードする。
    #[func(title = "Encode YAML")]
    pub fn encode(
        /// エンコード対象の値。
        value: Spanned<Value>,
    ) -> SourceResult<Str> {
        let Spanned { v: value, span } = value;
        serde_yaml::to_string(&value)
            .map(|v| v.into())
            .map_err(|err| eco_format!("failed to encode value as YAML ({err})"))
            .at(span)
    }
}
