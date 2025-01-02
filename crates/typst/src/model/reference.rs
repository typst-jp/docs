use comemo::Track;
use ecow::eco_format;

use crate::diag::{bail, At, Hint, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Content, Context, Func, IntoValue, Label, NativeElement, Packed, Show,
    Smart, StyleChain, Synthesize,
};
use crate::introspection::{Counter, Locatable};
use crate::math::EquationElem;
use crate::model::{
    BibliographyElem, CiteElem, Destination, Figurable, FootnoteElem, Numbering,
};
use crate::text::TextElem;

/// ラベルや参考文献への参照。
///
/// ラベルに対するテキストの参照を生成します。
/// 例えば、見出しに対する参照は、最初の見出しに対して"Section 1"のような適切な文字列を生成します。
/// 参照は、それぞれの要素へのリンクでもあります。
/// 参照の構文は参考文献からの[引用]($cite)にも使用できます。
///
/// 参照可能な要素には[見出し]($heading)、[図表]($figure)、[数式]($math.equation)、[脚注]($footnote)が含まれます。
/// 定理のようなカスタムの参照可能な要素を作成するには、
/// カスタムの[`種類`]($figure.kind)の図表を作成し、
/// それに対するshowルールを書きます。
/// 将来的には、カスタムの参照可能な要素を定義するためのより直接的な方法が提供されるかもしれません。
///
/// ラベルのついた要素にリンクを行いたいだけで、
/// 自動的なテキストの参照が必要ない場合は、代わりに[`link`]関数の使用を検討してください。
///
/// # 例
/// ```example
/// #set heading(numbering: "1.")
/// #set math.equation(numbering: "(1)")
///
/// = Introduction <intro>
/// Recent developments in
/// typesetting software have
/// rekindled hope in previously
/// frustrated researchers. @distress
/// As shown in @results, we ...
///
/// = Results <results>
/// We discuss our approach in
/// comparison with others.
///
/// == Performance <perf>
/// @slow demonstrates what slow
/// software looks like.
/// $ T(n) = O(2^n) $ <slow>
///
/// #bibliography("works.bib")
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
/// ラベルへの参照は、@の後にラベル名を入力することで作成できます
/// （例：`[= Introduction <intro>]`は`[@intro]`と入力することで参照できます）。
///
/// 補足をカスタマイズするには、
/// `[@intro[Chapter]]`のように、参照の後に角括弧でコンテンツを追加します。
///
/// # カスタム
/// 参照のshowルールを書く場合、
/// 参照の`element`フィールドを通じて参照先の要素にアクセスできます。
/// ただし、Typstがまだそれを発見していない場合、`element`は存在していても`{none}`になる可能性があるため、
/// 常にコード内でそのケースを処理する必要があります。   
///
/// ```example
/// #set heading(numbering: "1.")
/// #set math.equation(numbering: "(1)")
///
/// #show ref: it => {
///   let eq = math.equation
///   let el = it.element
///   if el != none and el.func() == eq {
///     // Override equation references.
///     link(el.location(),numbering(
///       el.numbering,
///       ..counter(eq).at(el.location())
///     ))
///   } else {
///     // Other references as usual.
///     it
///   }
/// }
///
/// = Beginnings <beginning>
/// In @beginning we prove @pythagoras.
/// $ a^2 + b^2 = c^2 $ <pythagoras>
/// ```
#[elem(title = "Reference", Synthesize, Locatable, Show)]
pub struct RefElem {
    /// 参照されるべき対象ラベル。
    ///
    /// これは、ドキュメント内で定義されたラベルや、
    /// [`参考文献リスト`]($bibliography)の参照キーである場合があります。
    #[required]
    pub target: Label,

    /// 参照の補足。
    ///
    /// 見出しや図表への参照の場合、参照される番号の前に追加されます。
    /// 引用の場合、ページ番号を追加するために使用できます。
    ///
    /// 関数が指定されている場合、それに参照先の要素が渡され、
    /// 関数はコンテンツを返す必要があります。
    ///
    /// ```example
    /// #set heading(numbering: "1.")
    /// #set ref(supplement: it => {
    ///   if it.func() == heading {
    ///     "Chapter"
    ///   } else {
    ///     "Thing"
    ///   }
    /// })
    ///
    /// = Introduction <intro>
    /// In @intro, we see how to turn
    /// Sections into Chapters. And
    /// in @intro[Part], it is done
    /// manually.
    /// ```
    #[borrowed]
    pub supplement: Smart<Option<Supplement>>,

    /// A synthesized citation.
    #[synthesized]
    pub citation: Option<Packed<CiteElem>>,

    /// The referenced element.
    #[synthesized]
    pub element: Option<Content>,
}

impl Synthesize for Packed<RefElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let citation = to_citation(self, engine, styles)?;

        let elem = self.as_mut();
        elem.push_citation(Some(citation));
        elem.push_element(None);

        let target = *elem.target();
        if !BibliographyElem::has(engine, target) {
            if let Ok(found) = engine.introspector.query_label(target).cloned() {
                elem.push_element(Some(found));
                return Ok(());
            }
        }

        Ok(())
    }
}

impl Show for Packed<RefElem> {
    #[typst_macros::time(name = "ref", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let target = *self.target();
        let elem = engine.introspector.query_label(target);
        let span = self.span();

        if BibliographyElem::has(engine, target) {
            if elem.is_ok() {
                bail!(span, "label occurs in the document and its bibliography");
            }

            return Ok(to_citation(self, engine, styles)?.pack().spanned(span));
        }

        let elem = elem.at(span)?;

        if let Some(footnote) = elem.to_packed::<FootnoteElem>() {
            return Ok(footnote.into_ref(target).pack().spanned(span));
        }

        let elem = elem.clone();
        let refable = elem
            .with::<dyn Refable>()
            .ok_or_else(|| {
                if elem.can::<dyn Figurable>() {
                    eco_format!(
                        "cannot reference {} directly, try putting it into a figure",
                        elem.func().name()
                    )
                } else {
                    eco_format!("cannot reference {}", elem.func().name())
                }
            })
            .at(span)?;

        let numbering = refable
            .numbering()
            .ok_or_else(|| {
                eco_format!("cannot reference {} without numbering", elem.func().name())
            })
            .hint(eco_format!(
                "you can enable {} numbering with `#set {}(numbering: \"1.\")`",
                elem.func().name(),
                if elem.func() == EquationElem::elem() {
                    "math.equation"
                } else {
                    elem.func().name()
                }
            ))
            .at(span)?;

        let loc = elem.location().unwrap();
        let numbers = refable.counter().display_at_loc(
            engine,
            loc,
            styles,
            &numbering.clone().trimmed(),
        )?;

        let supplement = match self.supplement(styles).as_ref() {
            Smart::Auto => refable.supplement(),
            Smart::Custom(None) => Content::empty(),
            Smart::Custom(Some(supplement)) => {
                supplement.resolve(engine, styles, [elem])?
            }
        };

        let mut content = numbers;
        if !supplement.is_empty() {
            content = supplement + TextElem::packed("\u{a0}") + content;
        }

        Ok(content.linked(Destination::Location(loc)))
    }
}

/// Turn a reference into a citation.
fn to_citation(
    reference: &Packed<RefElem>,
    engine: &mut Engine,
    styles: StyleChain,
) -> SourceResult<Packed<CiteElem>> {
    let mut elem = Packed::new(CiteElem::new(*reference.target()).with_supplement(
        match reference.supplement(styles).clone() {
            Smart::Custom(Some(Supplement::Content(content))) => Some(content),
            _ => None,
        },
    ));

    if let Some(loc) = reference.location() {
        elem.set_location(loc);
    }

    elem.synthesize(engine, styles)?;

    Ok(elem)
}

/// Additional content for a reference.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Supplement {
    Content(Content),
    Func(Func),
}

impl Supplement {
    /// Tries to resolve the supplement into its content.
    pub fn resolve<T: IntoValue>(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        args: impl IntoIterator<Item = T>,
    ) -> SourceResult<Content> {
        Ok(match self {
            Supplement::Content(content) => content.clone(),
            Supplement::Func(func) => func
                .call(engine, Context::new(None, Some(styles)).track(), args)?
                .display(),
        })
    }
}

cast! {
    Supplement,
    self => match self {
        Self::Content(v) => v.into_value(),
        Self::Func(v) => v.into_value(),
    },
    v: Content => Self::Content(v),
    v: Func => Self::Func(v),
}

/// Marks an element as being able to be referenced. This is used to implement
/// the `@ref` element.
pub trait Refable {
    /// The supplement, if not overridden by the reference.
    fn supplement(&self) -> Content;

    /// Returns the counter of this element.
    fn counter(&self) -> Counter;

    /// Returns the numbering of this element.
    fn numbering(&self) -> Option<&Numbering>;
}
