use comemo::Track;
use ecow::eco_format;

use crate::diag::{At, Hint, SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Cast, Content, Context, Func, IntoValue, Label, NativeElement, Packed, Repr, Smart,
    StyleChain, Synthesize, cast, elem,
};
use crate::introspection::{Counter, CounterKey, Locatable, Tagged};
use crate::math::EquationElem;
use crate::model::{
    BibliographyElem, CiteElem, DirectLinkElem, Figurable, FootnoteElem, Numbering,
};
use crate::text::TextElem;

<<<<<<< HEAD
/// ラベルや参考文献への参照。
///
/// ラベルを指定して、その参照を生成します。参照の[`form`]($ref.form)には
/// `{"normal"}`と`{"page"}`の2種類があります。
///
/// デフォルトの`{"normal"}`参照では、ラベルに対するテキスト形式の参照が作られます。
/// 例えば見出しへの参照なら、最初の見出しに対しては「Section 1」のような文字列になります。
/// 「Section」という語は[`lang`]($text.lang)の設定により自動的にローカライズされます。
/// 参照は該当する要素へのリンクにもなります。
/// また、参照の構文は文献リストからの[cite]にも使用できます。
///
/// このデフォルト形式では補足語と番号が必要なため、ラベルは_参照可能な要素_に付けなくてはなりません。
/// 参照可能な要素としては、
/// [headings]($heading)、[figures]($figure)、[equations]($math.equation)、[footnotes]($footnote)
/// などがあります。
/// 定理（theorem）などのカスタム参照可能要素を作成したい場合は、カスタム[`kind`]($figure.kind)の図表として作成し、
/// それに対応するshowルールを書くことで作成できます。
/// 将来的には、カスタム参照可能要素をもっと直接的に定義する方法が導入されるかもしれません。
///
/// 自動的な文字列表現が不要で、単にラベル付き要素へリンクしたい場合は、
/// [`link`]関数の使用を検討してください。
///
/// `{"page"}`参照は、ラベルの位置に対応するページ番号への参照を生成します。
/// [pageのsupplement]($page.supplement)を使うと、ページ番号の前の文言を変更できます。
/// `{"normal"}`参照と異なり、ラベルは任意の要素に付けられます。
///
/// # 例
=======
/// A reference to a label or bibliography.
///
/// Takes a label and cross-references it. There are two kind of references,
/// determined by its [`form`]($ref.form): `{"normal"}` and `{"page"}`.
///
/// The default, a `{"normal"}` reference, produces a textual reference to a
/// label. For example, a reference to a heading will yield an appropriate
/// string such as "Section 1" for a reference to the first heading. The word
/// "Section" depends on the [`lang`]($text.lang) setting and is localized
/// accordingly. The references are also links to the respective element.
/// Reference syntax can also be used to [cite] from a bibliography.
///
/// As the default form requires a supplement and numbering, the label must be
/// attached to a _referenceable element_. Referenceable elements include
/// [headings]($heading), [figures]($figure), [equations]($math.equation), and
/// [footnotes]($footnote). To create a custom referenceable element like a
/// theorem, you can create a figure of a custom [`kind`]($figure.kind) and
/// write a show rule for it. In the future, there might be a more direct way
/// to define a custom referenceable element.
///
/// If you just want to link to a labelled element and not get an automatic
/// textual reference, consider using the [`link`] function instead.
///
/// A `{"page"}` reference produces a page reference to a label, displaying the
/// page number at its location. You can use the
/// [page's supplement]($page.supplement) to modify the text before the page
/// number. Unlike a `{"normal"}` reference, the label can be attached to any
/// element.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
<<<<<<< HEAD
/// この機能には専用の記法も用意されています。
/// `{"normal"}`の参照を作成するためには`@`に続けてラベル名を入力します（例えば`[= Introduction <intro>]`というラベルを参照するには`[@intro]`と入力します）。
///
/// 補足語をカスタマイズするには、
/// `[@intro[Chapter]]`のように、参照の後に角括弧でコンテンツを追加します。
///
/// # カスタマイズ
/// 図表や見出しなどのページ参照だけが必要な場合は、setルールで`form`の既定値を
/// `{"page"}`に変更できます。"page"より短い"p."のような補足語にしたい場合は、
/// [`page.supplement`]フィールドで変更できます。
=======
/// This function also has dedicated syntax: A `{"normal"}` reference to a
/// label can be created by typing an `@` followed by the name of the label
/// (e.g. `[= Introduction <intro>]` can be referenced by typing `[@intro]`).
///
/// To customize the supplement, add content in square brackets after the
/// reference: `[@intro[Chapter]]`.
///
/// # Customization
/// When you only ever need to reference pages of a figure/table/heading/etc. in
/// a document, the default `form` field value can be changed to `{"page"}` with
/// a set rule. If you prefer a short "p." supplement over "page", the
/// [`page.supplement`] field can be used for changing this:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// #set page(
///   numbering: "1",
///   supplement: "p.",
/// >>> margin: (bottom: 3em),
/// >>> footer-descent: 1.25em,
/// )
/// #set ref(form: "page")
///
/// #figure(
///   stack(
///     dir: ltr,
///     spacing: 1em,
///     circle(),
///     square(),
///   ),
///   caption: [Shapes],
/// ) <shapes>
///
/// #pagebreak()
///
/// See @shapes for examples
/// of different shapes.
/// ```
///
<<<<<<< HEAD
/// 参照のshowルールを書く場合、参照の`element`フィールドを通じて参照先の要素にアクセスできます。
/// ただし、Typstがまだそれを発見していない場合、`element`は存在していても`{none}`になる可能性があるため、
/// 常にコード内でそのケースを処理する必要があります。
=======
/// If you write a show rule for references, you can access the referenced
/// element through the `element` field of the reference. The `element` may
/// be `{none}` even if it exists if Typst hasn't discovered it yet, so you
/// always need to handle that case in your code.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
///
/// ```example
/// #set heading(numbering: "1.")
/// #set math.equation(numbering: "(1)")
///
/// #show ref: it => {
///   let eq = math.equation
///   let el = it.element
///   // Skip all other references.
///   if el == none or el.func() != eq { return it }
///   // Override equation references.
///   link(el.location(), numbering(
///     el.numbering,
///     ..counter(eq).at(el.location())
///   ))
/// }
///
/// = Beginnings <beginning>
/// In @beginning we prove @pythagoras.
/// $ a^2 + b^2 = c^2 $ <pythagoras>
/// ```
#[elem(title = "Reference", Locatable, Tagged, Synthesize)]
pub struct RefElem {
<<<<<<< HEAD
    /// 参照されるべき対象ラベル。
    ///
    /// ドキュメント内で定義されたラベルか、`{"normal"}`の[`form`]($ref.form)を使う場合は[`bibliography`]の項目でも構いません。
    #[required]
    pub target: Label,

    /// 参照の補足語。
    ///
    /// [`form`]($ref.form)が`{"normal"}`の場合は以下の通りです。
    ///
    /// - 見出しや図への参照では、この値が参照番号の前に追加されます。
    /// - 文献引用の場合は、ページ番号などを追記するのに使えます。
    ///
    /// [`form`]($ref.form)が`{"page"}`の場合は、参照先ラベルのページ番号の前にこの値が追加されます。
    ///
    /// また、関数が指定されている場合は、それに参照先の要素が渡され、戻り値のコンテンツが補足語となります。
=======
    /// The target label that should be referenced.
    ///
    /// Can be a label that is defined in the document or, if the
    /// [`form`]($ref.form) is set to `["normal"]`, an entry from the
    /// [`bibliography`].
    #[required]
    pub target: Label,

    /// A supplement for the reference.
    ///
    /// If the [`form`]($ref.form) is set to `{"normal"}`:
    /// - For references to headings or figures, this is added before the
    ///   referenced number.
    /// - For citations, this can be used to add a page number.
    ///
    /// If the [`form`]($ref.form) is set to `{"page"}`, then this is added
    /// before the page number of the label referenced.
    ///
    /// If a function is specified, it is passed the referenced element and
    /// should return content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
    pub supplement: Smart<Option<Supplement>>,

<<<<<<< HEAD
    /// 生成する参照の種類。
=======
    /// The kind of reference to produce.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    ///
    /// ```example
    /// #set page(numbering: "1")
    ///
    /// Here <here> we are on
    /// #ref(<here>, form: "page").
    /// ```
    #[default(RefForm::Normal)]
    pub form: RefForm,

<<<<<<< HEAD
    /// 合成された引用。
    #[synthesized]
    pub citation: Option<Packed<CiteElem>>,

    /// 参照先の要素。
=======
    /// A synthesized citation.
    #[synthesized]
    pub citation: Option<Packed<CiteElem>>,

    /// The referenced element.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
        elem.citation = Some(Some(citation));
        elem.element = Some(None);

        if !BibliographyElem::has(engine, elem.target)
            && let Ok(found) = engine.introspector.query_label(elem.target).cloned()
        {
            elem.element = Some(Some(found));
            return Ok(());
        }

        Ok(())
    }
}

impl Packed<RefElem> {
    /// Realize as a linked, textual reference.
    pub fn realize(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Content> {
        let elem = engine.introspector.query_label(self.target);
        let span = self.span();

        let form = self.form.get(styles);
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

            return realize_reference(
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
            if let Ok(elem) = elem {
                bail!(
                    span,
                    "label `{}` occurs both in the document and its bibliography",
                    self.target.repr();
                    hint: "change either the {}'s label or the \
                           bibliography key to resolve the ambiguity",
                    elem.func().name(),
                );
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
                if elem.func() == EquationElem::ELEM {
                    "math.equation"
                } else {
                    elem.func().name()
                }
            ))
            .at(span)?;

        realize_reference(
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
fn realize_reference(
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

    let supplement = match reference.supplement.get_ref(styles) {
        Smart::Auto => supplement,
        Smart::Custom(None) => Content::empty(),
        Smart::Custom(Some(supplement)) => supplement.resolve(engine, styles, [elem])?,
    };

    let alt = {
        let supplement = supplement.plain_text();
        let numbering = numbers.plain_text();
        eco_format!("{supplement} {numbering}",)
    };

    let mut content = numbers;
    if !supplement.is_empty() {
        content = supplement + TextElem::packed("\u{a0}") + content;
    }

    content = content.spanned(reference.span());

    Ok(DirectLinkElem::new(loc, content, Some(alt)).pack())
}

/// Turn a reference into a citation.
fn to_citation(
    reference: &Packed<RefElem>,
    engine: &mut Engine,
    styles: StyleChain,
) -> SourceResult<Packed<CiteElem>> {
    let mut elem = Packed::new(CiteElem::new(reference.target).with_supplement(
        match reference.supplement.get_cloned(styles) {
            Smart::Custom(Some(Supplement::Content(content))) => Some(content),
            _ => None,
        },
    ));

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

<<<<<<< HEAD
/// 参照の形式。
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum RefForm {
    /// ラベルに対して文字列での参照を生成します。
    #[default]
    Normal,
    /// ラベルに対してページ番号での参照を生成します。
=======
/// The form of the reference.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum RefForm {
    /// Produces a textual reference to a label.
    #[default]
    Normal,
    /// Produces a page reference to a label.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
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
