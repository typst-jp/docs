<<<<<<< HEAD
use ecow::EcoString;
use roxmltree::ParsingOptions;
use typst_syntax::Spanned;

use crate::diag::{format_xml_like_error, At, FileError, SourceResult};
use crate::engine::Engine;
use crate::foundations::{dict, func, scope, Array, Dict, IntoValue, Str, Value};
use crate::loading::{DataSource, Load, Readable};

/// XMLファイルから構造化データを読み込む。
///
/// XMLファイルは辞書と文字列からなる配列にパースされます。
/// XMLノードは要素または文字列になり得ます。
/// 要素は以下のキーを持つ辞書として表現されます。
///
/// - `tag`: 要素の名称を表す文字列。
/// - `attrs`: 要素の属性を表す文字列からなる辞書。
/// - `children`: 要素の子ノードからなる配列。
///
/// この例におけるXMLファイルは、ルート要素である`news`タグと複数の`article`タグを含んでいます。
/// それぞれのarticleは`title`、`author`、および`content`タグを持っています。
/// `content`タグは1つ以上の段落を含んでおり、
/// これらは`p`タグとして表現されています。
///
/// # 例
=======
use roxmltree::ParsingOptions;
use typst_syntax::Spanned;

use crate::diag::{LoadError, LoadedWithin, SourceResult, format_xml_like_error};
use crate::engine::Engine;
use crate::foundations::{Array, Dict, IntoValue, Str, Value, dict, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// Reads structured data from an XML file.
///
/// The XML file is parsed into an array of dictionaries and strings. XML nodes
/// can be elements or strings. Elements are represented as dictionaries with
/// the following keys:
///
/// - `tag`: The name of the element as a string.
/// - `attrs`: A dictionary of the element's attributes as strings.
/// - `children`: An array of the element's child nodes.
///
/// The XML file in the example contains a root `news` tag with multiple
/// `article` tags. Each article has a `title`, `author`, and `content` tag. The
/// `content` tag contains one or more paragraphs, which are represented as `p`
/// tags.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #let find-child(elem, tag) = {
///   elem.children
///     .find(e => "tag" in e and e.tag == tag)
/// }
///
/// #let article(elem) = {
///   let title = find-child(elem, "title")
///   let author = find-child(elem, "author")
///   let pars = find-child(elem, "content")
///
///   [= #title.children.first()]
///   text(10pt, weight: "medium")[
///     Published by
///     #author.children.first()
///   ]
///
///   for p in pars.children {
///     if type(p) == dictionary {
///       parbreak()
///       p.children.first()
///     }
///   }
/// }
///
/// #let data = xml("example.xml")
/// #for elem in data.first().children {
///   if type(elem) == dictionary {
///     article(elem)
///   }
/// }
/// ```
#[func(scope, title = "XML")]
pub fn xml(
    engine: &mut Engine,
<<<<<<< HEAD
    /// XMLファイルの[パス]($syntax/#paths)または生のXMLバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let data = source.load(engine.world)?;
    let text = data.as_str().map_err(FileError::from).at(source.span)?;
=======
    /// A [path]($syntax/#paths) to an XML file or raw XML bytes.
    source: Spanned<DataSource>,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    let text = loaded.data.as_str().within(&loaded)?;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let document = roxmltree::Document::parse_with_options(
        text,
        ParsingOptions { allow_dtd: true, ..Default::default() },
    )
    .map_err(format_xml_error)
<<<<<<< HEAD
    .at(source.span)?;
=======
    .within(&loaded)?;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Ok(convert_xml(document.root()))
}

#[scope]
impl xml {
<<<<<<< HEAD
    /// XMLの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode XML")]
    #[deprecated = "`xml.decode`は非推奨です。代わりにバイト列を直接`xml`に渡してください。"]
    pub fn decode(
        engine: &mut Engine,
        /// XMLデータ。
=======
    /// Reads structured data from an XML string/bytes.
    #[func(title = "Decode XML")]
    #[deprecated(
        message = "`xml.decode` is deprecated, directly pass bytes to `xml` instead",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// XML data.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        xml(engine, data.map(Readable::into_source))
    }
}

/// Convert an XML node to a Typst value.
fn convert_xml(node: roxmltree::Node) -> Value {
    if node.is_text() {
        return node.text().unwrap_or_default().into_value();
    }

    let children: Array = node.children().map(convert_xml).collect();
    if node.is_root() {
        return Value::Array(children);
    }

    let tag: Str = node.tag_name().name().into();
    let attrs: Dict = node
        .attributes()
        .map(|attr| (attr.name().into(), attr.value().into_value()))
        .collect();

    Value::Dict(dict! {
        "tag" => tag,
        "attrs" => attrs,
        "children" => children,
    })
}

/// Format the user-facing XML error message.
<<<<<<< HEAD
fn format_xml_error(error: roxmltree::Error) -> EcoString {
=======
fn format_xml_error(error: roxmltree::Error) -> LoadError {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    format_xml_like_error("XML", error)
}
