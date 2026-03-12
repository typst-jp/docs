use ecow::eco_format;
use typst_syntax::Spanned;

use crate::diag::{At, LoadError, LoadedWithin, ReportPos, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Dict, Str, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// TOMLファイルから構造化データを読み込む。
///
/// 読み込むファイルには有効なTOMLテーブルが含まれていなければなりません。
/// TOMLの値は、[下の表](#conversion)に示す対応するTypstの値に変換されます。
///
/// この関数はTOMLテーブルに対応する辞書を返します。
///
/// この例におけるTOMLファイルは、
/// `title`、`version`、および`authors`のキーを持つテーブルで構成されています。
///
/// # 例
/// ```example
/// #let details = toml("details.toml")
///
/// Title: #details.title \
/// Version: #details.version \
/// Authors: #(details.authors
///   .join(", ", last: " and "))
/// ```
///
/// # 変換の詳細 { #conversion }
///
/// まず、TOML文書はテーブルです。その他の値はテーブルに入れて
/// エンコード/デコードする必要があります。
///
/// | TOMLの値 | Typstへの変換先 |
/// | -------- | -------------- |
/// | string   | [`str`]        |
/// | integer  | [`int`]        |
/// | float    | [`float`]      |
/// | boolean  | [`bool`]       |
/// | datetime | [`datetime`]   |
/// | array    | [`array`]      |
/// | table    | [`dictionary`] |
///
/// | Typstの値                            | TOMLへの変換先                      |
/// | ------------------------------------- | ----------------------------------- |
/// | TOMLから変換できる型                  | 対応するTOML値                      |
/// | `{none}`                              | 無視                                |
/// | [`bytes`]                             | [`repr`]経由の文字列                |
/// | [`symbol`]                            | 文字列                              |
/// | [`content`]                           | コンテンツを記述するテーブル         |
/// | その他の型（[`length`]など）          | [`repr`]経由の文字列                |
///
/// ## 注意事項
/// - 2<sup>63</sup>-1より大きい（または-2<sup>63</sup>より小さい）TOML整数は
///   Typstで損失なく表現できず、
///   [仕様](https://toml.io/en/v1.0.0#integer)に従ってエラーになります。
///
/// - `bytes`は性能と可読性のためTOML配列としてはエンコードされません。
///   バイナリデータには[`cbor.encode`]を検討してください。
///
/// - `repr`関数は[デバッグ目的のみ]($repr/#debugging-only)で、
///   出力の安定性はTypstのバージョン間で保証されません。
#[func(scope, title = "TOML")]
pub fn toml(
    engine: &mut Engine,
    /// TOMLファイルの[パス]($syntax/#paths)、または生のTOMLバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Dict> {
    let loaded = source.load(engine.world)?;
    let raw = loaded.data.as_str().within(&loaded)?;
    ::toml::from_str(raw).map_err(format_toml_error).within(&loaded)
}

#[scope]
impl toml {
    /// TOMLの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode TOML")]
    #[deprecated(
        message = "`toml.decode`は非推奨です。代わりにバイト列を直接`toml`に渡してください。",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// TOMLデータ。
        data: Spanned<Readable>,
    ) -> SourceResult<Dict> {
        toml(engine, data.map(Readable::into_source))
    }

    /// 構造化データをTOML文字列にエンコードする。
    #[func(title = "Encode TOML")]
    pub fn encode(
        /// エンコード対象の値。
        ///
        /// TOML文書はテーブルなので、辞書のみが適しています。
        value: Spanned<Dict>,
        /// TOMLを整形表示するかどうか。
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
fn format_toml_error(error: ::toml::de::Error) -> LoadError {
    let pos = error.span().map(ReportPos::from).unwrap_or_default();
    LoadError::new(pos, "failed to parse TOML", error.message())
}
