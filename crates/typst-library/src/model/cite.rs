use typst_syntax::Spanned;

use crate::diag::{error, At, HintedString, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Cast, Content, Derived, Label, Packed, Show, Smart, StyleChain,
    Synthesize,
};
use crate::introspection::Locatable;
use crate::model::bibliography::Works;
use crate::model::{CslSource, CslStyle};
use crate::text::{Lang, Region, TextElem};

/// 参考文献の引用。
///
/// 引用を始める前に、文書のどこかで[bibliography]を追加しておく必要があります。
///
/// # 例
/// ```example
/// This was already noted by
/// pirates long ago. @arrgh
///
/// Multiple sources say ...
/// @arrgh @netwok.
///
/// You can also call `cite`
/// explicitly. #cite(<arrgh>)
///
/// #bibliography("works.bib")
/// ```
///
/// ソース名にスラッシュなど`<>`構文では認識されない文字が含まれている場合は、代わりにlabelを明示的に呼び出すことで参照できます。
///
/// ```typ
/// Computer Modern is an example of a modernist serif typeface.
/// #cite(label("DBLP:books/lib/Knuth86a")).
/// >>> #bibliography("works.bib")
/// ```
///
/// # 構文
/// この関数は間接的に専用の構文を持っています。
/// [References]($ref)は参考文献を引用するために使用可能です。
/// ラベルは参照キーに対応します。
#[elem(Synthesize)]
pub struct CiteElem {
    /// 引用する文献を特定するラベルである参照キー。
    ///
    /// ```example
    /// // All the same
    /// @netwok \
    /// #cite(<netwok>) \
    /// #cite(label("netwok"))
    /// >>> #set text(0pt)
    /// >>> #bibliography("works.bib", style: "apa")
    /// ```
    #[required]
    pub key: Label,

    /// ページ番号や章番号などの引用の補足語。
    ///
    /// [References]($ref)の構文では、角括弧で囲むことで補足語を追加できます。
    ///
    /// ```example
    /// This has been proven. @distress[p.~7]
    ///
    /// #bibliography("works.bib")
    /// ```
    pub supplement: Option<Content>,

    /// 作成する引用の種類。異なる形式は異なるシナリオで有用です。
    /// 通常の引用は文末に置くソースとして有用ですが、"prose"引用は文章の途中に置くのに適しています。
    ///
    /// もし`{none}`と設定すると、引用文献は参考文献リストに含まれますが、文章内には表示されません。
    ///
    /// ```example
    /// #cite(<netwok>, form: "prose")
    /// show the outsized effects of
    /// pirate life on the human psyche.
    /// >>> #set text(0pt)
    /// >>> #bibliography("works.bib", style: "apa")
    /// ```
    #[default(Some(CitationForm::Normal))]
    pub form: Option<CitationForm>,
    /// 引用スタイル。
    ///
    /// This can be:
    /// - `{auto}` to automatically use the
    ///   [bibliography's style]($bibliography.style) for citations.
    /// - A string with the name of one of the built-in styles (see below). Some
    ///   of the styles listed below appear twice, once with their full name and
    ///   once with a short alias.
    /// - A path string to a [CSL file](https://citationstyles.org/). For more
    ///   details about paths, see the [Paths section]($syntax/#paths).
    /// - Raw bytes from which a CSL style should be decoded.
    #[parse(match args.named::<Spanned<Smart<CslSource>>>("style")? {
        Some(Spanned { v: Smart::Custom(source), span }) => Some(Smart::Custom(
            CslStyle::load(engine.world, Spanned::new(source, span))?
        )),
        Some(Spanned { v: Smart::Auto, .. }) => Some(Smart::Auto),
        None => None,
    })]
    #[borrowed]
    pub style: Smart<Derived<CslSource, CslStyle>>,

    /// The text language setting where the citation is.
    #[internal]
    #[synthesized]
    pub lang: Lang,

    /// The text region setting where the citation is.
    #[internal]
    #[synthesized]
    pub region: Option<Region>,
}

impl Synthesize for Packed<CiteElem> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        let elem = self.as_mut();
        elem.push_lang(TextElem::lang_in(styles));
        elem.push_region(TextElem::region_in(styles));
        Ok(())
    }
}

cast! {
    CiteElem,
    v: Content => v.unpack::<Self>().map_err(|_| "expected citation")?,
}

/// The form of the citation.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum CitationForm {
    /// 現在設定しているスタイルの標準的な方法で表示する。
    #[default]
    Normal,
    /// 文章に含めるのに適した引用を作成する。
    Prose,
    /// 参考文献リストと同じく、引用された文献の完全な情報を表示する。
    Full,
    /// 引用文献の著者らのみを表示する。
    Author,
    /// 引用文献の発行年のみを表示する。
    Year,
}

/// A group of citations.
///
/// This is automatically created from adjacent citations during show rule
/// application.
#[elem(Locatable, Show)]
pub struct CiteGroup {
    /// The citations.
    #[required]
    pub children: Vec<Packed<CiteElem>>,
}

impl Show for Packed<CiteGroup> {
    #[typst_macros::time(name = "cite", span = self.span())]
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        let location = self.location().unwrap();
        let span = self.span();
        Works::generate(engine)
            .at(span)?
            .citations
            .get(&location)
            .cloned()
            .ok_or_else(failed_to_format_citation)
            .at(span)?
    }
}

/// The error message when a citation wasn't found in the pre-formatted list.
#[cold]
fn failed_to_format_citation() -> HintedString {
    error!(
        "cannot format citation in isolation";
        hint: "check whether this citation is measured \
               without being inserted into the document"
    )
}
