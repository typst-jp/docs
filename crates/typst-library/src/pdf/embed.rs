use ecow::EcoString;
use typst_library::foundations::Target;
use typst_syntax::Spanned;

use crate::diag::{warning, At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    elem, Bytes, Cast, Content, Derived, Packed, Show, StyleChain, TargetElem,
};
use crate::introspection::Locatable;
use crate::World;

/// 出力されるPDFに埋め込まれるファイル。
///
/// この関数は、PDFに関連する追加のファイルをPDF内に埋め込んで配布するために使用できます。
/// PDFリーダーはファイルリストにファイルを表示します。
///
/// 一部の国際規格では、
/// この仕組みを使用して、
/// PDFの視覚的な内容を反映した機械可読データ（例：請求書のZUGFeRD/Factur-X）を埋め込んでいます。
///
/// # 例
/// ```typ
/// #pdf.embed(
///   "experiment.csv",
///   relationship: "supplement",
///   mime-type: "text/csv",
///   description: "Raw Oxygen readings from the Arctic experiment",
/// )
/// ```
///
/// # 注意
/// - この要素はPDF以外の形式にエクスポートする場合は無視されます。
/// - PDF/A-2へのエクスポートでは、埋め込みファイルは現在サポートされていません。
///   たとえ埋め込まれるファイルがPDF/A-1やPDF/A-2に準拠していたとしてもです。
#[elem(Show, Locatable)]
pub struct EmbedElem {
    /// 埋め込まれるファイルの[パス]($syntax/#paths)。
    ///
    /// 常に指定する必要がありますが、
    /// 次の引数でデータが提供されていない場合にのみ読み取られます。
    #[required]
    #[parse(
        let Spanned { v: path, span } =
            args.expect::<Spanned<EcoString>>("path")?;
        let id = span.resolve_path(&path).at(span)?;
        // The derived part is the project-relative resolved path.
        let resolved = id.vpath().as_rootless_path().to_string_lossy().replace("\\", "/").into();
        Derived::new(path.clone(), resolved)
    )]
    #[borrowed]
    pub path: Derived<EcoString, EcoString>,

    /// 任意で指定する生のファイルデータ。
    ///
    /// この引数が指定されない場合、データは指定されたパスから読み取られます。
    #[positional]
    // Not actually required as an argument, but always present as a field.
    // We can't distinguish between the two at the moment.
    #[required]
    #[parse(
        match args.find::<Bytes>()? {
            Some(data) => data,
            None => engine.world.file(id).at(span)?,
        }
    )]
    pub data: Bytes,

    /// 埋め込まれるファイルと文書との関係。
    ///
    /// エクスポート対象がPDF/A-3でない場合は無視されます。
    pub relationship: Option<EmbeddedFileRelationship>,

    /// 埋め込まれるファイルのMIMEタイプ。
    #[borrowed]
    pub mime_type: Option<EcoString>,

    /// 埋め込まれるファイルの説明。
    #[borrowed]
    pub description: Option<EcoString>,
}

impl Show for Packed<EmbedElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        if TargetElem::target_in(styles) == Target::Html {
            engine
                .sink
                .warn(warning!(self.span(), "embed was ignored during HTML export"));
        }
        Ok(Content::empty())
    }
}

/// The relationship of an embedded file with the document.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum EmbeddedFileRelationship {
    /// PDF文書がそのソースファイルから作成されたことを示す。
    Source,
    /// ファイルがPDFにおける視覚的表現を得るために使われたことを示す。
    Data,
    /// 文書の代替的な表現であることを示す。
    Alternative,
    /// 文書への追加のリソースであることを示す。
    Supplement,
}
