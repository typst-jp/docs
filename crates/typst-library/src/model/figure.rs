use std::borrow::Cow;
use std::num::NonZeroUsize;
use std::str::FromStr;

use ecow::EcoString;
use typst_utils::NonZeroExt;

use crate::diag::{SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Content, Element, NativeElement, Packed, Selector, ShowSet, Smart, StyleChain,
    Styles, Synthesize, cast, elem, scope, select_where,
};
use crate::introspection::{
    Count, Counter, CounterKey, CounterUpdate, Locatable, Location, Tagged,
};
use crate::layout::{
    AlignElem, Alignment, BlockElem, Em, Length, OuterVAlignment, PlacementScope,
    VAlignment,
};
use crate::model::{Numbering, NumberingPattern, Outlinable, Refable, Supplement};
use crate::text::{Lang, Locale, TextElem};
use crate::visualize::ImageElem;

/// 任意でキャプションを持つ図表。
///
/// 自動的にその種類を検出し、それぞれに応じて番号付けします。
/// 例えば、画像を含む図表は表を含む図表とは別々に番号が付けられます。
///
/// # 例
/// 以下の例は、画像を含む基本的な図表を示しています。
/// ```example
/// @glacier shows a glacier. Glaciers
/// are complex systems.
///
/// #figure(
///   image("glacier.jpg", width: 80%),
///   caption: [A curious figure.],
/// ) <glacier>
/// ```
///
/// 図表に [tables]($table) を挿入してキャプションを付けることもできます。
/// 図表は表を含むことを検出し、自動的に別のカウンターを使用します。
///
/// ```example
/// #figure(
///   table(
///     columns: 4,
///     [t], [1], [2], [3],
///     [y], [0.3s], [0.4s], [0.8s],
///   ),
///   caption: [Timing results],
/// )
/// ```
///
/// この動作は、図表の種類である `kind` を明示的に指定することで上書き可能です。
/// 同じ種類の図表は全て共通のカウンターを共有します。
///
/// # 図表の動作
/// デフォルトでは、図表はコンテンツの流れの中に配置されます。
/// 図表をページの上部または下部に配置するには、[`placement`]($figure.placement)引数を使用します。
///
/// 図表が大きすぎてそのコンテンツがページをまたいで分割可能な場合（例えば大きな表が含まれている場合）、このshowルールで図表自体もページをまたいで分割可能です。
/// ```typ
/// #show figure: set block(breakable: true)
/// ```
///
/// 分割できるブロックと分割できないブロックの詳細については、[block]($block.breakable)のドキュメントを参照してください。
///
/// # キャプションの改変
/// 図表のキャプションの外観は、関連するキャプション機能で改変できます。
/// 以下の例では、全てのキャプションを斜体で強調しています。
///
/// ```example
/// #show figure.caption: emph
///
/// #figure(
///   rect[Hello],
///   caption: [I am emphasized!],
/// )
/// ```
///
/// [`where`]($function.where)セレクターを使うことで、このようなルールを特定の種類の図表に適用可能です。
/// 例えば、図表の種類が表の場合はキャプションを表の上に配置し、他の種類ではキャプションを下に配置するには、次のようなshow-setルールを記述します。
///
/// ```example
/// #show figure.where(
///   kind: table
/// ): set figure.caption(position: top)
///
/// #figure(
///   table(columns: 2)[A][B][C][D],
///   caption: [I'm up here],
/// )
/// ```
///
/// # Accessibility
/// You can use the [`alt`]($figure.alt) parameter to provide an [alternative
/// description]($guides/accessibility/#textual-representations) of the figure
/// for screen readers and other Assistive Technology (AT). Refer to [its
/// documentation]($figure.alt) to learn more.
///
/// You can use figures to add alternative descriptions to paths, shapes, or
/// visualizations that do not have their own `alt` parameter. If your graphic
/// is purely decorative and does not have a semantic meaning, consider wrapping
/// it in [`pdf.artifact`] instead, which will hide it from AT when exporting to
/// PDF.
///
/// AT will always read the figure at the point where it appears in the
/// document, regardless of its [`placement`]($figure.placement). Put its markup
/// where it would make the most sense in the reading order.
#[elem(scope, Locatable, Tagged, Synthesize, Count, ShowSet, Refable, Outlinable)]
pub struct FigureElem {
    /// 図表のコンテンツ。多くの場合、 [image] が使われます。
    #[required]
    pub body: Content,

    /// An alternative description of the figure.
    ///
    /// When you add an alternative description, AT will read both it and the
    /// caption (if any). However, the content of the figure itself will be
    /// skipped.
    ///
    /// When the body of your figure is an [image]($image) with its own `alt`
    /// text set, this parameter should not be used on the figure element.
    /// Likewise, do not use this parameter when the figure contains a table,
    /// code, or other content that is already accessible. In such cases, the
    /// content of the figure will be read by AT, and adding an alternative
    /// description would lead to a loss of information.
    ///
    /// You can learn how to write good alternative descriptions in the
    /// [Accessibility Guide]($guides/accessibility/#textual-representations).
    pub alt: Option<EcoString>,

    /// ページ上における図表の配置。
    ///
    /// - `{none}`： 他のコンテンツと同様に書かれた場所に置かれる。
    /// - `{auto}`： `{top}` か `{bottom}` の近い方に置かれる。
    /// - `{top}`： ページの上部に置かれる。
    /// - `{bottom}`： ページの下部に置かれる。
    ///
    /// 本文のコンテンツと図表の間隔は`place`関数の [`clearance`]($place.clearance) 引数によって制御します。
    ///
    /// ```example
    /// #set page(height: 200pt)
    /// #show figure: set place(
    ///   clearance: 1em,
    /// )
    ///
    /// = Introduction
    /// #figure(
    ///   placement: bottom,
    ///   caption: [A glacier],
    ///   image("glacier.jpg", width: 60%),
    /// )
    /// #lorem(60)
    /// ```
    pub placement: Option<Smart<VAlignment>>,

    /// どの包含スコープに対して図を配置するか。
    ///
    /// これを`{"parent"}`に設定すると、段組みをまたいで、ページの幅を全て使用した図表を作成します。
    ///
    /// もし`placement`を`{none}`とした場合には、何の効果もありません。
    ///
    /// ```example
    /// #set page(height: 250pt, columns: 2)
    ///
    /// = Introduction
    /// #figure(
    ///   placement: bottom,
    ///   scope: "parent",
    ///   caption: [A glacier],
    ///   image("glacier.jpg", width: 60%),
    /// )
    /// #lorem(60)
    /// ```
    pub scope: PlacementScope,

    /// 図表のキャプション。
    pub caption: Option<Packed<FigureCaption>>,

    /// 図表の種類。
    ///
    /// 同じ種類の全ての図表は共通のカウンターを共有します。
    ///
    /// `{auto}` に設定された場合、図形はその中で記述されているものの種類に基づいて、自動的にその種類の決定を試みます。
    /// 自動的に検出される種類は、[table]($table)と[code]($raw)です。
    /// それ以外の場合は[image]と推測されます。
    ///
    /// これを `{auto}` 以外に設定すると、自動検出が上書きされます。
    /// 以下のような場合に便利です。
    /// - [image]や[table]、[code]($raw)以外のカスタム図表を作りたい場合
    /// - コンテンツに関わらず特定のカウンターを強制的に使用したい場合
    ///
    /// 種類は、要素関数または文字列に設定できます。
    /// [`table`]($table)、[`raw`]($raw)、[`image`]($image)以外の要素関数に設定した場合は、図表の補足語（supplement）を手動で指定する必要があります。
    ///
    /// ```example:"Customizing the figure kind"
    /// #figure(
    ///   circle(radius: 10pt),
    ///   caption: [A curious atom.],
    ///   kind: "atom",
    ///   supplement: [Atom],
    /// )
    /// ```
    ///
    /// If you want to modify a counter to skip a number or reset the counter,
    /// you can access the [counter] of each kind of figure with a
    /// [`where`]($function.where) selector:
    ///
    /// - For [tables]($table): `{counter(figure.where(kind: table))}`
    /// - For [images]($image): `{counter(figure.where(kind: image))}`
    /// - For a custom kind: `{counter(figure.where(kind: kind))}`
    ///
    /// ```example:"Modifying the figure counter for specific kinds"
    /// #figure(
    ///   table(columns: 2, $n$, $1$),
    ///   caption: [The first table.],
    /// )
    ///
    /// #counter(
    ///   figure.where(kind: table)
    /// ).update(41)
    ///
    /// #figure(
    ///   table(columns: 2, $n$, $42$),
    ///   caption: [The 42nd table],
    /// )
    ///
    /// #figure(
    ///   rect[Image],
    ///   caption: [Does not affect images],
    /// )
    /// ```
    ///
    /// To conveniently use the correct counter in a show rule, you can access
    /// the `counter` field. There is an example of this in the documentation
    /// [of the `figure.caption` element's `body` field]($figure.caption.body).
    pub kind: Smart<FigureKind>,

    /// 図表の補足語。
    ///
    /// `{auto}` に設定すると、図表は、種類や[テキスト言語]($text.lang)に基づいて、正しい補足語を自動的に決定しようとします。
    /// 独自の図表タイプを使用している場合は、補足語を手動で指定する必要があります。
    ///
    /// 関数が指定された場合、その関数は指定された種類の最初の子孫要素（通常は図の本体）に渡され、コンテンツを返す必要があります。
    ///
    /// ```example
    /// #figure(
    ///   [The contents of my figure!],
    ///   caption: [My custom figure],
    ///   supplement: [Bar],
    ///   kind: "foo",
    /// )
    /// ```
    pub supplement: Smart<Option<Supplement>>,

    /// 番号の付け方。[番号付けのパターンや関数]($numbering)を受け付けます。
    #[default(Some(NumberingPattern::from_str("1").unwrap().into()))]
    pub numbering: Option<Numbering>,

    /// 本文とキャプションの間の垂直方向の隙間。
    #[default(Em::new(0.65).into())]
    pub gap: Length,

    /// 図表を[`outline`]に表示するかどうか。
    #[default(true)]
    pub outlined: bool,

    /// この図表のカウンターにアクセスするための便利なフィールド。
    ///
    /// カウンターは図表の種類 `kind` にのみ依存します。
    /// - (table)[@table]に対して: `{counter(figure.where(kind: table))}`
    /// - (image)[@image]に対して: `{counter(figure.where(kind: image))}`
    /// - 独自の図表kindに対して: `{counter(figure.where(kind: kind))}`
    ///
    /// 数字をスキップしたり、カウンターをリセットしたい場合は、これらのカウンターを修正する必要があります。
    #[synthesized]
    pub counter: Option<Counter>,

    /// The locale of this element (used for the alternative description).
    #[internal]
    #[synthesized]
    pub locale: Locale,
}

#[scope]
impl FigureElem {
    #[elem]
    type FigureCaption;
}

impl FigureElem {
    /// Retrieves the locale separator.
    pub fn resolve_separator(&self, styles: StyleChain) -> Content {
        match self.caption.get_ref(styles) {
            Some(caption) => caption.resolve_separator(styles),
            None => FigureCaption::local_separator_in(styles),
        }
    }
}

impl Synthesize for Packed<FigureElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let span = self.span();
        let location = self.location();
        let elem = self.as_mut();
        let numbering = elem.numbering.get_ref(styles);

        // Determine the figure's kind.
        let kind = elem.kind.get_cloned(styles).unwrap_or_else(|| {
            elem.body
                .query_first_naive(&Selector::can::<dyn Figurable>())
                .map(|elem| FigureKind::Elem(elem.func()))
                .unwrap_or_else(|| FigureKind::Elem(ImageElem::ELEM))
        });

        // Resolve the supplement.
        let supplement = match elem.supplement.get_ref(styles).as_ref() {
            Smart::Auto => {
                // Default to the local name for the kind, if available.
                let name = match &kind {
                    FigureKind::Elem(func) => func
                        .local_name(
                            styles.get(TextElem::lang),
                            styles.get(TextElem::region),
                        )
                        .map(TextElem::packed),
                    FigureKind::Name(_) => None,
                };

                if numbering.is_some() && name.is_none() {
                    bail!(span, "please specify the figure's supplement")
                }

                Some(name.unwrap_or_default())
            }
            Smart::Custom(None) => None,
            Smart::Custom(Some(supplement)) => {
                // Resolve the supplement with the first descendant of the kind or
                // just the body, if none was found.
                let descendant = match kind {
                    FigureKind::Elem(func) => elem
                        .body
                        .query_first_naive(&Selector::Elem(func, None))
                        .map(Cow::Owned),
                    FigureKind::Name(_) => None,
                };

                let target = descendant.unwrap_or_else(|| Cow::Borrowed(&elem.body));
                Some(supplement.resolve(engine, styles, [target])?)
            }
        };

        // Construct the figure's counter.
        let counter = Counter::new(CounterKey::Selector(
            select_where!(FigureElem, kind => kind.clone()),
        ));

        // Fill the figure's caption.
        let mut caption = elem.caption.get_cloned(styles);
        if let Some(caption) = &mut caption {
            caption.synthesize(engine, styles)?;
            caption.kind = Some(kind.clone());
            caption.supplement = Some(supplement.clone());
            caption.numbering = Some(numbering.clone());
            caption.counter = Some(Some(counter.clone()));
            caption.figure_location = Some(location);
        }

        elem.kind.set(Smart::Custom(kind));
        elem.supplement
            .set(Smart::Custom(supplement.map(Supplement::Content)));
        elem.counter = Some(Some(counter));
        elem.caption.set(caption);
        elem.locale = Some(Locale::get_in(styles));

        Ok(())
    }
}

impl ShowSet for Packed<FigureElem> {
    fn show_set(&self, _: StyleChain) -> Styles {
        // Still allows breakable figures with
        // `show figure: set block(breakable: true)`.
        let mut map = Styles::new();
        map.set(BlockElem::breakable, false);
        map.set(AlignElem::alignment, Alignment::CENTER);
        map
    }
}

impl Count for Packed<FigureElem> {
    fn update(&self) -> Option<CounterUpdate> {
        // If the figure is numbered, step the counter by one.
        // This steps the `counter(figure)` which is global to all numbered figures.
        self.numbering()
            .is_some()
            .then(|| CounterUpdate::Step(NonZeroUsize::ONE))
    }
}

impl Refable for Packed<FigureElem> {
    fn supplement(&self) -> Content {
        // After synthesis, this should always be custom content.
        match self.supplement.get_cloned(StyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
            _ => Content::empty(),
        }
    }

    fn counter(&self) -> Counter {
        self.counter
            .clone()
            .flatten()
            .unwrap_or_else(|| Counter::of(FigureElem::ELEM))
    }

    fn numbering(&self) -> Option<&Numbering> {
        self.numbering.get_ref(StyleChain::default()).as_ref()
    }
}

impl Outlinable for Packed<FigureElem> {
    fn outlined(&self) -> bool {
        self.outlined.get(StyleChain::default())
            && (self.caption.get_ref(StyleChain::default()).is_some()
                || self.numbering().is_some())
    }

    fn prefix(&self, numbers: Content) -> Content {
        let supplement = self.supplement();
        if !supplement.is_empty() {
            supplement + TextElem::packed('\u{a0}') + numbers
        } else {
            numbers
        }
    }

    fn body(&self) -> Content {
        self.caption
            .get_ref(StyleChain::default())
            .as_ref()
            .map(|caption| caption.body.clone())
            .unwrap_or_default()
    }
}

/// 図のキャプション。
/// この要素は、全ての図や特定の種類の図のキャプションの外観を改変するために、
/// setルールやshowルールで使用可能です。
///
/// キャプションは、`pos`と`body`に加えて、図の`kind`や`supplement`、`counter`、`numbering`もフィールドとして提供します。
/// これらの要素を[`where`]($function.where)セレクターやshowルールで使用することで、独自のキャプションを構築できます。
///
/// ```example
/// #show figure.caption: emph
///
/// #figure(
///   rect[Hello],
///   caption: [A rectangle],
/// )
/// ```
#[elem(name = "caption", Locatable, Tagged, Synthesize)]
pub struct FigureCaption {
    /// 図表の中のキャプションの位置。`{top}`や`{bottom}`を入力してください。
    ///
    /// ```example
    /// #show figure.where(
    ///   kind: table
    /// ): set figure.caption(position: top)
    ///
    /// #figure(
    ///   table(columns: 2)[A][B],
    ///   caption: [I'm up here],
    /// )
    ///
    /// #figure(
    ///   rect[Hi],
    ///   caption: [I'm down here],
    /// )
    ///
    /// #figure(
    ///   table(columns: 2)[A][B],
    ///   caption: figure.caption(
    ///     position: bottom,
    ///     [I'm down here too!]
    ///   )
    /// )
    /// ```
    #[default(OuterVAlignment::Bottom)]
    pub position: OuterVAlignment,

    /// 番号とキャプション名の間に表示する区切り文字。
    ///
    /// `{auto}`に設定すると、区切り文字は
    /// [language]($text.lang)と[region]($text.region)に応じて決まります。
    ///
    /// ```example
    /// #set figure.caption(separator: [ --- ])
    ///
    /// #figure(
    ///   rect[Hello],
    ///   caption: [A rectangle],
    /// )
    /// ```
    pub separator: Smart<Content>,

    /// キャプション名。
    ///
    /// 独自のキャプションに改変するために
    /// `kind`、`supplement`、`counter`、`numbering`、`location`が同時に使えます。
    ///
    /// ```example
    /// #show figure.caption: it => [
    ///   #underline(it.body) |
    ///   #it.supplement
    ///   #context it.counter.display(it.numbering)
    /// ]
    ///
    /// #figure(
    ///   rect[Hello],
    ///   caption: [A rectangle],
    /// )
    /// ```
    #[required]
    pub body: Content,

    /// The figure's supplement.
    #[synthesized]
    pub kind: FigureKind,

    /// The figure's supplement.
    #[synthesized]
    pub supplement: Option<Content>,

    /// How to number the figure.
    #[synthesized]
    pub numbering: Option<Numbering>,

    /// The counter for the figure.
    #[synthesized]
    pub counter: Option<Counter>,

    /// The figure's location.
    #[internal]
    #[synthesized]
    pub figure_location: Option<Location>,
}

impl FigureCaption {
    /// Realizes the textual caption content.
    pub fn realize(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Content> {
        let mut realized = self.body.clone();

        if let (
            Some(Some(mut supplement)),
            Some(Some(numbering)),
            Some(Some(counter)),
            Some(Some(location)),
        ) = (
            self.supplement.clone(),
            &self.numbering,
            &self.counter,
            &self.figure_location,
        ) {
            let numbers = counter.display_at_loc(engine, *location, styles, numbering)?;
            if !supplement.is_empty() {
                supplement += TextElem::packed('\u{a0}');
            }
            realized = supplement + numbers + self.resolve_separator(styles) + realized;
        }

        Ok(realized)
    }

    /// Retrieves the locale separator.
    fn resolve_separator(&self, styles: StyleChain) -> Content {
        self.separator
            .get_cloned(styles)
            .unwrap_or_else(|| Self::local_separator_in(styles))
    }

    /// Gets the default separator in the given language and (optionally)
    /// region.
    fn local_separator_in(styles: StyleChain) -> Content {
        styles.get_cloned(Self::separator).unwrap_or_else(|| {
            TextElem::packed(match styles.get(TextElem::lang) {
                Lang::CHINESE => "\u{2003}",
                Lang::FRENCH => ".\u{a0}– ",
                Lang::RUSSIAN => ". ",
                Lang::ENGLISH | _ => ": ",
            })
        })
    }
}

impl Synthesize for Packed<FigureCaption> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        let elem = self.as_mut();
        elem.separator.set(Smart::Custom(elem.resolve_separator(styles)));
        Ok(())
    }
}

cast! {
    FigureCaption,
    v: Content => v.unpack::<Self>().unwrap_or_else(Self::new),
}

/// The `kind` parameter of a [`FigureElem`].
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FigureKind {
    /// The kind is an element function.
    Elem(Element),
    /// The kind is a name.
    Name(EcoString),
}

cast! {
    FigureKind,
    self => match self {
        Self::Elem(v) => v.into_value(),
        Self::Name(v) => v.into_value(),
    },
    v: Element => Self::Elem(v),
    v: EcoString => Self::Name(v),
}

/// An element that can be auto-detected in a figure.
///
/// This trait is used to determine the type of a figure.
pub trait Figurable {}
