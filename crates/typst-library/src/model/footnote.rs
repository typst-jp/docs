use std::num::NonZeroUsize;
use std::str::FromStr;

use typst_utils::NonZeroExt;

use crate::diag::{bail, At, SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Content, Label, NativeElement, Packed, Show, ShowSet, Smart,
    StyleChain, Styles,
};
use crate::introspection::{Count, Counter, CounterUpdate, Locatable, Location};
use crate::layout::{Abs, Em, HElem, Length, Ratio};
use crate::model::{Destination, Numbering, NumberingPattern, ParElem};
use crate::text::{SuperElem, TextElem, TextSize};
use crate::visualize::{LineElem, Stroke};

/// 脚注。
///
/// 脚注を用いて追加のコメントや参考文献を同じページに記述します。
/// 脚注は、ページ下部の注釈にリンクする上付きの番号を挿入します。
/// 注釈は文書全体で連続して番号付けされ、
/// 複数のページにまたがることができます。
///
/// 脚注リストの項目の外観をカスタマイズするには、
/// [`footnote.entry`]($footnote.entry)を参照してください。
/// 脚注自体は通常の上付き文字として実現されているため、
/// [`super`]関数に対してsetルールを適用してカスタマイズできます。
/// また、showルールを適用して、本文中の脚注マーカー（上付き番号）のみをカスタマイズすることもできます。
///
/// # 例
/// ```example
/// Check the docs for more details.
/// #footnote[https://typst.app/docs]
/// ```
///
/// 脚注は、マークアップにおいて前の単語との間にスペースがあったとしても、
/// 自動的にその単語に付加されます。
/// スペースを強制するには、文字列の`[#" "]`や明示的な[horizontal spacing]($h)を使用できます。
///
/// 脚注にラベルをつけることにより、脚注に対して複数の参照を持つことができます。
///
/// ```example
/// You can edit Typst documents online.
/// #footnote[https://typst.app/app] <fn>
/// Checkout Typst's website. @fn
/// And the online app. #footnote(<fn>)
/// ```
///
/// _注意:_ `footnote`が呼び出されるスコープ内でのsetルールやshowルールは、脚注のコンテンツに適用されない場合があります。
/// 詳細については[こちら][issue]を参照してください。
///
/// [issue]: https://github.com/typst/typst/issues/1467#issuecomment-1588799440
#[elem(scope, Locatable, Show, Count)]
pub struct FootnoteElem {
    /// 脚注の番号付け方法。
    ///
    /// デフォルトでは、脚注の番号付けは文書全体で連続します。
    /// ページごとに脚注の番号付けを行いたい場合は、
    /// ページの[header]($page.header)で脚注の[counter]をリセットできます。
    /// 将来的には、これを簡単に実現する方法が提供されるかもしれません。
    ///
    /// ```example
    /// #set footnote(numbering: "*")
    ///
    /// Footnotes:
    /// #footnote[Star],
    /// #footnote[Dagger]
    /// ```
    #[borrowed]
    #[default(Numbering::Pattern(NumberingPattern::from_str("1").unwrap()))]
    pub numbering: Numbering,

    /// 脚注に挿入するコンテンツ。
    /// この脚注が参照すべき他の脚注のラベルを指定することもできます。
    #[required]
    pub body: FootnoteBody,
}

#[scope]
impl FootnoteElem {
    #[elem]
    type FootnoteEntry;
}

impl FootnoteElem {
    /// Creates a new footnote that the passed content as its body.
    pub fn with_content(content: Content) -> Self {
        Self::new(FootnoteBody::Content(content))
    }

    /// Creates a new footnote referencing the footnote with the specified label.
    pub fn with_label(label: Label) -> Self {
        Self::new(FootnoteBody::Reference(label))
    }

    /// Creates a new footnote referencing the footnote with the specified label,
    /// with the other fields from the current footnote cloned.
    pub fn into_ref(&self, label: Label) -> Self {
        Self {
            body: FootnoteBody::Reference(label),
            ..self.clone()
        }
    }

    /// Tests if this footnote is a reference to another footnote.
    pub fn is_ref(&self) -> bool {
        matches!(self.body, FootnoteBody::Reference(_))
    }

    /// Returns the content of the body of this footnote if it is not a ref.
    pub fn body_content(&self) -> Option<&Content> {
        match &self.body {
            FootnoteBody::Content(content) => Some(content),
            _ => None,
        }
    }
}

impl Packed<FootnoteElem> {
    /// Returns the location of the definition of this footnote.
    pub fn declaration_location(&self, engine: &Engine) -> StrResult<Location> {
        match self.body {
            FootnoteBody::Reference(label) => {
                let element = engine.introspector.query_label(label)?;
                let footnote = element
                    .to_packed::<FootnoteElem>()
                    .ok_or("referenced element should be a footnote")?;
                if self.location() == footnote.location() {
                    bail!("footnote cannot reference itself");
                }
                footnote.declaration_location(engine)
            }
            _ => Ok(self.location().unwrap()),
        }
    }
}

impl Show for Packed<FootnoteElem> {
    #[typst_macros::time(name = "footnote", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let span = self.span();
        let loc = self.declaration_location(engine).at(span)?;
        let numbering = self.numbering(styles);
        let counter = Counter::of(FootnoteElem::elem());
        let num = counter.display_at_loc(engine, loc, styles, numbering)?;
        let sup = SuperElem::new(num).pack().spanned(span);
        let loc = loc.variant(1);
        // Add zero-width weak spacing to make the footnote "sticky".
        Ok(HElem::hole().pack() + sup.linked(Destination::Location(loc)))
    }
}

impl Count for Packed<FootnoteElem> {
    fn update(&self) -> Option<CounterUpdate> {
        (!self.is_ref()).then(|| CounterUpdate::Step(NonZeroUsize::ONE))
    }
}

/// The body of a footnote can be either some content or a label referencing
/// another footnote.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FootnoteBody {
    Content(Content),
    Reference(Label),
}

cast! {
    FootnoteBody,
    self => match self {
        Self::Content(v) => v.into_value(),
        Self::Reference(v) => v.into_value(),
    },
    v: Content => Self::Content(v),
    v: Label => Self::Reference(v),
}

/// 脚注リストの項目。
///
/// この関数は直接呼び出されることを意図されていません。
/// 代わりに、setルールやshowルールで脚注リストをカスタマイズするために使用されます。
///
/// ```example
/// #show footnote.entry: set text(red)
///
/// My footnote listing
/// #footnote[It's down here]
/// has red text!
/// ```
///
/// _注意:_ 脚注項目のプロパティは、
/// 各ページラン（ページ間に明示的な改ページがないページ群）全体で一貫している必要があります。
/// このため、脚注項目に対するsetルールやshowルールは通常はドキュメントの最初の部分など、
/// ページコンテンツの前に定義される必要があります。
#[elem(name = "entry", title = "Footnote Entry", Show, ShowSet)]
pub struct FootnoteEntry {
    /// この項目の脚注。
    /// その位置を指定して、脚注カウンターの状態を決定する事ができます。
    ///
    /// ```example
    /// #show footnote.entry: it => {
    ///   let loc = it.note.location()
    ///   numbering(
    ///     "1: ",
    ///     ..counter(footnote).at(loc),
    ///   )
    ///   it.note.body
    /// }
    ///
    /// Customized #footnote[Hello]
    /// listing #footnote[World! 🌏]
    /// ```
    #[required]
    pub note: Packed<FootnoteElem>,

    /// 文書の本文と脚注リストの間の区切り記号。
    ///
    /// ```example
    /// #set footnote.entry(
    ///   separator: repeat[.]
    /// )
    ///
    /// Testing a different separator.
    /// #footnote[
    ///   Unconventional, but maybe
    ///   not that bad?
    /// ]
    /// ```
    #[default(
        LineElem::new()
            .with_length(Ratio::new(0.3).into())
            .with_stroke(Stroke {
                thickness: Smart::Custom(Abs::pt(0.5).into()),
                ..Default::default()
            })
            .pack()
    )]
    pub separator: Content,

    /// 文書の本文と区切り記号の間の余白の量。
    ///
    /// ```example
    /// #set footnote.entry(clearance: 3em)
    ///
    /// Footnotes also need ...
    /// #footnote[
    ///   ... some space to breathe.
    /// ]
    /// ```
    #[default(Em::new(1.0).into())]
    #[resolve]
    pub clearance: Length,

    /// 脚注項目同士の間隔。
    ///
    /// ```example
    /// #set footnote.entry(gap: 0.8em)
    ///
    /// Footnotes:
    /// #footnote[Spaced],
    /// #footnote[Apart]
    /// ```
    #[default(Em::new(0.5).into())]
    #[resolve]
    pub gap: Length,

    /// 各脚注項目の字下げ。
    ///
    /// ```example
    /// #set footnote.entry(indent: 0em)
    ///
    /// Footnotes:
    /// #footnote[No],
    /// #footnote[Indent]
    /// ```
    #[default(Em::new(1.0).into())]
    pub indent: Length,
}

impl Show for Packed<FootnoteEntry> {
    #[typst_macros::time(name = "footnote.entry", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let span = self.span();
        let number_gap = Em::new(0.05);
        let default = StyleChain::default();
        let numbering = self.note.numbering(default);
        let counter = Counter::of(FootnoteElem::elem());
        let Some(loc) = self.note.location() else {
            bail!(
                span, "footnote entry must have a location";
                hint: "try using a query or a show rule to customize the footnote instead"
            );
        };

        let num = counter.display_at_loc(engine, loc, styles, numbering)?;
        let sup = SuperElem::new(num)
            .pack()
            .spanned(span)
            .linked(Destination::Location(loc))
            .located(loc.variant(1));

        Ok(Content::sequence([
            HElem::new(self.indent(styles).into()).pack(),
            sup,
            HElem::new(number_gap.into()).with_weak(true).pack(),
            self.note.body_content().unwrap().clone(),
        ]))
    }
}

impl ShowSet for Packed<FootnoteEntry> {
    fn show_set(&self, _: StyleChain) -> Styles {
        let mut out = Styles::new();
        out.set(ParElem::set_leading(Em::new(0.5).into()));
        out.set(TextElem::set_size(TextSize(Em::new(0.85).into())));
        out
    }
}

cast! {
    FootnoteElem,
    v: Content => v.unpack::<Self>().unwrap_or_else(Self::with_content)
}
