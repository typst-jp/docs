use ecow::eco_format;
use typst_syntax::Spanned;

<<<<<<< HEAD
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
=======
use crate::diag::{At, LineCol, LoadError, LoadedWithin, ReportPos, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Str, Value, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// Reads structured data from a YAML file.
///
/// The file must contain a valid YAML object or array. The YAML values will be
/// converted into corresponding Typst values as listed in the
/// [table below](#conversion).
///
/// The function returns a dictionary, an array or, depending on the YAML file,
/// another YAML data type.
///
/// The YAML files in the example contain objects with authors as keys,
/// each with a sequence of their own submapping with the keys
/// "title" and "published".
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
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
=======
///
/// # Conversion details { #conversion }
///
/// | YAML value                             | Converted into Typst |
/// | -------------------------------------- | -------------------- |
/// | null-values (`null`, `~` or empty ` `) | `{none}`             |
/// | boolean                                | [`bool`]             |
/// | number                                 | [`float`] or [`int`] |
/// | string                                 | [`str`]              |
/// | sequence                               | [`array`]            |
/// | mapping                                | [`dictionary`]       |
///
/// | Typst value                           | Converted into YAML              |
/// | ------------------------------------- | -------------------------------- |
/// | types that can be converted from YAML | corresponding YAML value         |
/// | [`bytes`]                             | string via [`repr`]              |
/// | [`symbol`]                            | string                           |
/// | [`content`]                           | a mapping describing the content |
/// | other types ([`length`], etc.)        | string via [`repr`]              |
///
/// ## Notes
/// - In most cases, YAML numbers will be converted to floats or integers
///   depending on whether they are whole numbers. However, be aware that
///   integers larger than 2<sup>63</sup>-1 or smaller than -2<sup>63</sup> will
///   be converted to floating-point numbers, which may result in an
///   approximative value.
///
/// - Custom YAML tags are ignored, though the loaded value will still be present.
///
/// - Bytes are not encoded as YAML sequences for performance and readability
///   reasons. Consider using [`cbor.encode`] for binary data.
///
/// - The `repr` function is [for debugging purposes only]($repr/#debugging-only),
///   and its output is not guaranteed to be stable across Typst versions.
#[func(scope, title = "YAML")]
pub fn yaml(
    engine: &mut Engine,
    /// A [path]($syntax/#paths) to a YAML file or raw YAML bytes.
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    serde_yaml::from_slice(loaded.data.as_slice())
        .map_err(format_yaml_error)
        .within(&loaded)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

#[scope]
impl yaml {
<<<<<<< HEAD
    /// YAMLの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode YAML")]
    #[deprecated = "`yaml.decode`は非推奨です。代わりにバイト列を直接`yaml`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// YAMLデータ。
=======
    /// Reads structured data from a YAML string/bytes.
    #[func(title = "Decode YAML")]
    #[deprecated(
        message = "`yaml.decode` is deprecated, directly pass bytes to `yaml` instead",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// YAML data.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        yaml(engine, data.map(Readable::into_source))
    }

<<<<<<< HEAD
    /// 構造化データをYAML文字列にエンコードする。
    #[func(title = "Encode YAML")]
    pub fn encode(
        /// エンコード対象の値。
=======
    /// Encode structured data into a YAML string.
    #[func(title = "Encode YAML")]
    pub fn encode(
        /// Value to be encoded.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        value: Spanned<Value>,
    ) -> SourceResult<Str> {
        let Spanned { v: value, span } = value;
        serde_yaml::to_string(&value)
            .map(|v| v.into())
            .map_err(|err| eco_format!("failed to encode value as YAML ({err})"))
            .at(span)
    }
}
<<<<<<< HEAD
=======

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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
