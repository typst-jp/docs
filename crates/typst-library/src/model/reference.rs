use comemo::Track;
use ecow::eco_format;

use crate::diag::{bail, At, Hint, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Cast, Content, Context, Func, IntoValue, Label, NativeElement, Packed,
    Show, Smart, StyleChain, Synthesize,
};
use crate::introspection::{Counter, CounterKey, Locatable};
use crate::math::EquationElem;
use crate::model::{
    BibliographyElem, CiteElem, Destination, Figurable, FootnoteElem, Numbering,
};
use crate::text::TextElem;

/// ラベルや参考文献への参照。
///
/// ラベルを指定して、その参照を生成します。
/// 参照の[`form`]($ref.form)には`{"normal"}`と`{"page"}`の2種類があります。
///
/// デフォルトの`{"normal"}`参照では、ラベルに対するテキスト形式の参照が作られます。
/// たとえば見出しへの参照なら、"Section 1"などのような適切な文字列が表示されます。
/// この参照は、該当する要素へのリンクとしても機能します。
/// また、参照の構文は文献リストからの引用を行う[cite]にも使用できます。
///
/// このデフォルト形式では補足語と番号が必要なため、ラベルは _参照可能な要素_ に付けなくてはなりません。
/// 参照可能な要素としては、
/// [headings]($heading)、[figures]($figure)、[equations]($math.equation)、[footnotes]($footnote)
/// などがあります。
/// 定理（theorem）などのカスタム参照可能要素を作成したい場合は、カスタム[`kind`]($figure.kind)の図表として作成し、それに対応するshowルールを書くことで作成可能です。
/// 将来的には、カスタム参照可能要素をもっと直接的に定義する方法が導入されるかもしれません。
///
/// # 例
/// ```example
/// #set page(numbering: "1")
/// #set heading(numbering: "1.")
/// #set math.equation(numbering: "(1)")
///
/// = Introduction <intro>
/// Recent developments in
/// typesetting software have
/// rekindled hope in previously
/// frustrated researchers. @distress
/// As shown in @results (see
/// #ref(<results>, form: "page")),
/// we ...
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
/// # Syntax
/// この機能には専用の記法も用意されています。
/// `{"normal"}` の参照を作成するためには`@`に続けてラベル名を入力します。
/// （たとえば`[= Introduction <intro>]`というラベルを参照するには`[@intro]`と入力します。）
///
/// 補足語をカスタマイズするには、
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

    /// 参照の補足語。
    ///
    /// [`form`]($ref.form)が`{"normal"}`で設定されている場合は以下のとおりです。
    /// - 見出しや図への参照では、この値が参照番号の前に追加されます。
    /// - 文献引用の場合は、ページ番号などを追記するのに使えます。
    ///
    /// もし[`form`]($ref.form)が`{"page"}`に設定されている場合には、
    /// 参照先ラベルのページ番号の前にこの値が追加されます。
    ///
    /// また、関数が指定されている場合は、それに参照先の要素が渡され、戻り値のコンテンツが補足語となります。
    ///
    /// ```example
    /// #set heading(numbering: "1.")
    /// #show ref.where(
    ///   form: "normal"
    /// ): set ref(supplement: it => {
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

    /// 生成する参照の種類
    ///
    /// ```example
    /// #set page(numbering: "1")
    ///
    /// Here <here> we are on
    /// #ref(<here>, form: "page").
    /// ```
    #[default(RefForm::Normal)]
    pub form: RefForm,

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

        if !BibliographyElem::has(engine, elem.target) {
            if let Ok(found) = engine.introspector.query_label(elem.target).cloned() {
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
        let elem = engine.introspector.query_label(self.target);
        let span = self.span();

        let form = self.form(styles);
        if form == RefForm::Page {
            let elem = elem.at(span)?;
            let elem = elem.clone();

            let loc = elem.location().unwrap();
            let numbering = engine
                .introspector
                .page_numbering(loc)
                .ok_or_else(|| eco_format!("cannot reference without page numbering"))
                .hint(eco_format!(
                    "you can enable page numbering with `#set page(numbering: \"1\")`"
                ))
                .at(span)?;
            let supplement = engine.introspector.page_supplement(loc);

            return show_reference(
                self,
                engine,
                styles,
                Counter::new(CounterKey::Page),
                numbering.clone(),
                supplement,
                elem,
            );
        }
        // RefForm::Normal

        if BibliographyElem::has(engine, self.target) {
            if elem.is_ok() {
                bail!(span, "label occurs in the document and its bibliography");
            }

            return Ok(to_citation(self, engine, styles)?.pack().spanned(span));
        }

        let elem = elem.at(span)?;

        if let Some(footnote) = elem.to_packed::<FootnoteElem>() {
            return Ok(footnote.into_ref(self.target).pack().spanned(span));
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

        show_reference(
            self,
            engine,
            styles,
            refable.counter(),
            numbering.clone(),
            refable.supplement(),
            elem,
        )
    }
}

/// Show a reference.
fn show_reference(
    reference: &Packed<RefElem>,
    engine: &mut Engine,
    styles: StyleChain,
    counter: Counter,
    numbering: Numbering,
    supplement: Content,
    elem: Content,
) -> SourceResult<Content> {
    let loc = elem.location().unwrap();
    let numbers = counter.display_at_loc(engine, loc, styles, &numbering.trimmed())?;

    let supplement = match reference.supplement(styles).as_ref() {
        Smart::Auto => supplement,
        Smart::Custom(None) => Content::empty(),
        Smart::Custom(Some(supplement)) => supplement.resolve(engine, styles, [elem])?,
    };

    let mut content = numbers;
    if !supplement.is_empty() {
        content = supplement + TextElem::packed("\u{a0}") + content;
    }

    Ok(content.linked(Destination::Location(loc)))
}

/// Turn a reference into a citation.
fn to_citation(
    reference: &Packed<RefElem>,
    engine: &mut Engine,
    styles: StyleChain,
) -> SourceResult<Packed<CiteElem>> {
    let mut elem = Packed::new(CiteElem::new(reference.target).with_supplement(
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

/// The form of the reference.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum RefForm {
    /// ラベルに対して文字列での参照を生成します。
    #[default]
    Normal,
    /// ラベルに対してページ番号での参照を生成します。
    Page,
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
