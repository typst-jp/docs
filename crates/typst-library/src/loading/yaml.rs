use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, LineCol, LoadError, LoadedWithin, ReportPos, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Str, Value, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// YAMLファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なYAMLオブジェクトまたは配列が含まれていなければなりません。
/// YAMLの値は、[下の表](#conversion)に示す対応するTypstの値に変換されます。
///
/// この関数は辞書、配列、またはYAMLファイルの内容に応じた別のYAMLデータ型を返します。
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
///
/// # 変換の詳細 { #conversion }
///
/// | YAMLの値                              | Typstへの変換先       |
/// | ------------------------------------- | -------------------- |
/// | null値 (`null`、`~`、空の` `)          | `{none}`             |
/// | boolean                               | [`bool`]             |
/// | number                                | [`float`] または [`int`] |
/// | string                                | [`str`]              |
/// | sequence                              | [`array`]            |
/// | mapping                               | [`dictionary`]       |
///
/// | Typstの値                            | YAMLへの変換先                      |
/// | ------------------------------------- | ----------------------------------- |
/// | YAMLから変換できる型                  | 対応するYAML値                      |
/// | [`bytes`]                             | [`repr`]経由の文字列                |
/// | [`symbol`]                            | 文字列                              |
/// | [`content`]                           | コンテンツを記述するマッピング       |
/// | その他の型（[`length`]など）          | [`repr`]経由の文字列                |
///
/// ## 注意事項
/// - 多くの場合、YAMLの数値は整数か小数かに応じて`float`または`int`に変換されます。
///   ただし、2<sup>63</sup>-1より大きい（または-2<sup>63</sup>より小さい）整数は
///   浮動小数点数に変換されるため、近似値になる可能性があります。
///
/// - カスタムYAMLタグは無視されますが、読み込まれた値はそのまま保持されます。
///
/// - `bytes`は性能と可読性のためYAMLシーケンスとしてはエンコードされません。
///   バイナリデータには[`cbor.encode`]を検討してください。
///
/// - `repr`関数は[デバッグ目的のみ]($repr/#debugging-only)で、
///   出力の安定性はTypstのバージョン間で保証されません。
#[func(scope, title = "YAML")]
pub fn yaml(
    engine: &mut Engine,
    /// YAMLファイルの[パス]($syntax/#paths)、または生のYAMLバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    serde_yaml::from_slice(loaded.data.as_slice())
        .map_err(format_yaml_error)
        .within(&loaded)
}

#[scope]
impl yaml {
    /// YAMLの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode YAML")]
    #[deprecated(
        message = "`yaml.decode`は非推奨です。代わりにバイト列を直接`yaml`に渡してください。",
        until = "0.15.0"
    )]
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

/// Format the user-facing YAML error message.
pub fn format_yaml_error(error: serde_yaml::Error) -> LoadError {
    let pos = error
        .location()
        .map(|loc| {
            let line_col = LineCol::one_based(loc.line(), loc.column());
            let range = loc.index()..loc.index();
            ReportPos::full(range, line_col)
        })
        .unwrap_or_default();
    LoadError::new(pos, "failed to parse YAML", error)
}
