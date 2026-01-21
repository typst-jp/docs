use std::borrow::Cow;
use std::num::NonZeroUsize;
use std::str::FromStr;

use ecow::EcoString;
use typst_utils::NonZeroExt;

<<<<<<< HEAD
use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, select_where, Content, Element, NativeElement, Packed, Selector,
    Show, ShowSet, Smart, StyleChain, Styles, Synthesize, TargetElem,
};
use crate::html::{tag, HtmlElem};
use crate::introspection::{
    Count, Counter, CounterKey, CounterUpdate, Locatable, Location,
};
use crate::layout::{
    AlignElem, Alignment, BlockBody, BlockElem, Em, HAlignment, Length, OuterVAlignment,
    PlaceElem, PlacementScope, VAlignment, VElem,
};
use crate::model::{
    Numbering, NumberingPattern, Outlinable, ParbreakElem, Refable, Supplement,
};
use crate::text::{Lang, Region, TextElem};
use crate::visualize::ImageElem;

/// 任意でキャプションを持つ図表。
///
/// 自動的にその種類を検出し、それぞれに応じて番号付けします。
/// 例えば、画像を含む図表は表を含む図表とは別々に番号が付けられます。
///
/// # 例
/// 以下の例は、画像を含む基本的な図表を示しています。
=======
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

/// A figure with an optional caption.
///
/// Automatically detects its kind to select the correct counting track. For
/// example, figures containing images will be numbered separately from figures
/// containing tables.
///
/// # Examples
/// The example below shows a basic figure with an image:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// 図表に [tables]($table) を挿入してキャプションを付けることもできます。
/// 図表は表を含むこと検出し、自動的に別のカウンターを使用します。
=======
/// You can also insert [tables]($table) into figures to give them a caption.
/// The figure will detect this and automatically use a separate counter.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// この動作は、図表の種類である `kind` を明示的に指定することで上書き可能です。
/// 同じ種類の図表は全て共通のカウンターを共有します。
///
/// # 図表の動作
/// デフォルトでは、図表はコンテンツの流れの中に配置されます。
/// 図表をページの上部または下部に配置するには、[`placement`]($figure.placement)引数を使用します。
///
/// 図表が大きすぎてそのコンテンツがページをまたいで分割可能な場合（例えば大きな表が含まれている場合）、このshowルールで図表自体もページをまたいで分割可能です。
=======
/// This behaviour can be overridden by explicitly specifying the figure's
/// `kind`. All figures of the same kind share a common counter.
///
/// # Figure behaviour
/// By default, figures are placed within the flow of content. To make them
/// float to the top or bottom of the page, you can use the
/// [`placement`]($figure.placement) argument.
///
/// If your figure is too large and its contents are breakable across pages
/// (e.g. if it contains a large table), then you can make the figure itself
/// breakable across pages as well with this show rule:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```typ
/// #show figure: set block(breakable: true)
/// ```
///
<<<<<<< HEAD
/// 分割できるブロックと分割できないブロックの詳細については、[block]($block.breakable)のドキュメントを参照してください。
///
/// # キャプションの改変
/// 図表のキャプションの外観は、関連するキャプション機能で改変できます。
/// 以下の例では、全てのキャプションを斜体で強調しています。
=======
/// See the [block]($block.breakable) documentation for more information about
/// breakable and non-breakable blocks.
///
/// # Caption customization
/// You can modify the appearance of the figure's caption with its associated
/// [`caption`]($figure.caption) function. In the example below, we emphasize
/// all captions:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// [`where`]($function.where)セレクターを使うことで、このようなルールを特定の種類の図表に適用可能です。
/// 例えば、図表の種類が表の場合はキャプションを表の上に配置し、他の種類ではキャプションを下に配置するには、次のようなshow-setルールを記述します。
=======
/// By using a [`where`]($function.where) selector, we can scope such rules to
/// specific kinds of figures. For example, to position the caption above
/// tables, but keep it below for all other kinds of figures, we could write the
/// following show-set rule:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
#[elem(scope, Locatable, Synthesize, Count, Show, ShowSet, Refable, Outlinable)]
pub struct FigureElem {
    /// 図表のコンテンツ。多くの場合、 [image] が使われます。
    #[required]
    pub body: Content,

    /// ページ上における図表の配置。
    ///
    /// - `{none}`: 他のコンテンツと同様に書かれた場所に置かれる。
    /// - `{auto}`: `{top}` か `{bottom}` の近い方に置かれる。
    /// - `{top}`: ページの上部に置かれる。
    /// - `{bottom}`: ページの下部に置かれる。
    ///
    /// 本文のコンテンツと図表の間隔は`place`関数の [`clearance`]($place.clearance) 引数によって制御します。
    ///
    /// ```example
    /// #set page(height: 200pt)
=======
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
    /// The content of the figure. Often, an [image].
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

    /// The figure's placement on the page.
    ///
    /// - `{none}`: The figure stays in-flow exactly where it was specified
    ///   like other content.
    /// - `{auto}`: The figure picks `{top}` or `{bottom}` depending on which
    ///   is closer.
    /// - `{top}`: The figure floats to the top of the page.
    /// - `{bottom}`: The figure floats to the bottom of the page.
    ///
    /// The gap between the main flow content and the floating figure is
    /// controlled by the [`clearance`]($place.clearance) argument on the
    /// `place` function.
    ///
    /// ```example
    /// #set page(height: 200pt)
    /// #show figure: set place(
    ///   clearance: 1em,
    /// )
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// どの包含スコープに対して図を配置するか。
    ///
    /// これを`{"parent"}`に設定すると、段組みをまたいで、ページの幅を全て使用した図表を作成します。
    ///
    /// もし`placement`を`{none}`とした場合には、何の効果もありません。
=======
    /// Relative to which containing scope the figure is placed.
    ///
    /// Set this to `{"parent"}` to create a full-width figure in a two-column
    /// document.
    ///
    /// Has no effect if `placement` is `{none}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// 図表のキャプション。
    #[borrowed]
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
    /// [`{table}`]($table)、[`{raw}`](raw)、[`{image}`](image)以外の要素関数に設定した場合は、図表の補足語（supplement）を手動で指定する必要があります。
    ///
    /// ```example
=======
    /// The figure's caption.
    pub caption: Option<Packed<FigureCaption>>,

    /// The kind of figure this is.
    ///
    /// All figures of the same kind share a common counter.
    ///
    /// If set to `{auto}`, the figure will try to automatically determine its
    /// kind based on the type of its body. Automatically detected kinds are
    /// [tables]($table) and [code]($raw). In other cases, the inferred kind is
    /// that of an [image].
    ///
    /// Setting this to something other than `{auto}` will override the
    /// automatic detection. This can be useful if
    /// - you wish to create a custom figure type that is not an
    ///   [image], a [table] or [code]($raw),
    /// - you want to force the figure to use a specific counter regardless of
    ///   its content.
    ///
    /// You can set the kind to be an element function or a string. If you set
    /// it to an element function other than [`table`], [`raw`], or [`image`],
    /// you will need to manually specify the figure's supplement.
    ///
    /// ```example:"Customizing the figure kind"
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// #figure(
    ///   circle(radius: 10pt),
    ///   caption: [A curious atom.],
    ///   kind: "atom",
    ///   supplement: [Atom],
    /// )
    /// ```
<<<<<<< HEAD
    pub kind: Smart<FigureKind>,

    /// 図表の補足語。
    ///
    /// `{auto}` に設定すると、図表は、種類や[テキスト言語]($text.lang)に基づいて、正しい補足語を自動的に決定しようとします。
    /// 独自の図表タイプを使用している場合は、補足語を手動で指定する必要があります。
    ///
    /// 関数が指定された場合、その関数は指定された種類の最初の子孫要素（通常は図の本体）に渡され、コンテンツを返す必要があります。
=======
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

    /// The figure's supplement.
    ///
    /// If set to `{auto}`, the figure will try to automatically determine the
    /// correct supplement based on the `kind` and the active
    /// [text language]($text.lang). If you are using a custom figure type, you
    /// will need to manually specify the supplement.
    ///
    /// If a function is specified, it is passed the first descendant of the
    /// specified `kind` (typically, the figure's body) and should return
    /// content.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #figure(
    ///   [The contents of my figure!],
    ///   caption: [My custom figure],
    ///   supplement: [Bar],
    ///   kind: "foo",
    /// )
    /// ```
<<<<<<< HEAD
    #[borrowed]
    pub supplement: Smart<Option<Supplement>>,

    /// 番号の付け方。[番号付けのパターンや関数]($numbering)を受け付けます。
    #[default(Some(NumberingPattern::from_str("1").unwrap().into()))]
    #[borrowed]
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
=======
    pub supplement: Smart<Option<Supplement>>,

    /// How to number the figure. Accepts a
    /// [numbering pattern or function]($numbering) taking a single number.
    #[default(Some(NumberingPattern::from_str("1").unwrap().into()))]
    pub numbering: Option<Numbering>,

    /// The vertical gap between the body and caption.
    #[default(Em::new(0.65).into())]
    pub gap: Length,

    /// Whether the figure should appear in an [`outline`] of figures.
    #[default(true)]
    pub outlined: bool,

    /// Convenience field to get access to the counter for this figure.
    ///
    /// The counter only depends on the `kind`:
    /// - For [tables]($table): `{counter(figure.where(kind: table))}`
    /// - For [images]($image): `{counter(figure.where(kind: image))}`
    /// - For a custom kind: `{counter(figure.where(kind: kind))}`
    ///
    /// These are the counters you'll need to modify if you want to skip a
    /// number or reset the counter.
    #[synthesized]
    pub counter: Option<Counter>,

    /// The locale of this element (used for the alternative description).
    #[internal]
    #[synthesized]
    pub locale: Locale,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

#[scope]
impl FigureElem {
    #[elem]
    type FigureCaption;
}

<<<<<<< HEAD
=======
impl FigureElem {
    /// Retrieves the locale separator.
    pub fn resolve_separator(&self, styles: StyleChain) -> Content {
        match self.caption.get_ref(styles) {
            Some(caption) => caption.resolve_separator(styles),
            None => FigureCaption::local_separator_in(styles),
        }
    }
}

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
impl Synthesize for Packed<FigureElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let span = self.span();
        let location = self.location();
        let elem = self.as_mut();
<<<<<<< HEAD
        let numbering = elem.numbering(styles);

        // Determine the figure's kind.
        let kind = elem.kind(styles).unwrap_or_else(|| {
            elem.body
                .query_first(&Selector::can::<dyn Figurable>())
                .map(|elem| FigureKind::Elem(elem.func()))
                .unwrap_or_else(|| FigureKind::Elem(ImageElem::elem()))
        });

        // Resolve the supplement.
        let supplement = match elem.supplement(styles).as_ref() {
=======
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Smart::Auto => {
                // Default to the local name for the kind, if available.
                let name = match &kind {
                    FigureKind::Elem(func) => func
                        .local_name(
<<<<<<< HEAD
                            TextElem::lang_in(styles),
                            TextElem::region_in(styles),
=======
                            styles.get(TextElem::lang),
                            styles.get(TextElem::region),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
                    FigureKind::Elem(func) => {
                        elem.body.query_first(&Selector::Elem(func, None)).map(Cow::Owned)
                    }
=======
                    FigureKind::Elem(func) => elem
                        .body
                        .query_first_naive(&Selector::Elem(func, None))
                        .map(Cow::Owned),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                    FigureKind::Name(_) => None,
                };

                let target = descendant.unwrap_or_else(|| Cow::Borrowed(&elem.body));
                Some(supplement.resolve(engine, styles, [target])?)
            }
        };

        // Construct the figure's counter.
        let counter = Counter::new(CounterKey::Selector(
<<<<<<< HEAD
            select_where!(FigureElem, Kind => kind.clone()),
        ));

        // Fill the figure's caption.
        let mut caption = elem.caption(styles).clone();
        if let Some(caption) = &mut caption {
            caption.synthesize(engine, styles)?;
            caption.push_kind(kind.clone());
            caption.push_supplement(supplement.clone());
            caption.push_numbering(numbering.clone());
            caption.push_counter(Some(counter.clone()));
            caption.push_figure_location(location);
        }

        elem.push_kind(Smart::Custom(kind));
        elem.push_supplement(Smart::Custom(supplement.map(Supplement::Content)));
        elem.push_counter(Some(counter));
        elem.push_caption(caption);
=======
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        Ok(())
    }
}

<<<<<<< HEAD
impl Show for Packed<FigureElem> {
    #[typst_macros::time(name = "figure", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let span = self.span();
        let target = TargetElem::target_in(styles);
        let mut realized = self.body.clone();

        // Build the caption, if any.
        if let Some(caption) = self.caption(styles).clone() {
            let (first, second) = match caption.position(styles) {
                OuterVAlignment::Top => (caption.pack(), realized),
                OuterVAlignment::Bottom => (realized, caption.pack()),
            };
            let mut seq = Vec::with_capacity(3);
            seq.push(first);
            if !target.is_html() {
                let v = VElem::new(self.gap(styles).into()).with_weak(true);
                seq.push(v.pack().spanned(span))
            }
            seq.push(second);
            realized = Content::sequence(seq)
        }

        // Ensure that the body is considered a paragraph.
        realized += ParbreakElem::shared().clone().spanned(span);

        if target.is_html() {
            return Ok(HtmlElem::new(tag::figure)
                .with_body(Some(realized))
                .pack()
                .spanned(span));
        }

        // Wrap the contents in a block.
        realized = BlockElem::new()
            .with_body(Some(BlockBody::Content(realized)))
            .pack()
            .spanned(span);

        // Wrap in a float.
        if let Some(align) = self.placement(styles) {
            realized = PlaceElem::new(realized)
                .with_alignment(align.map(|align| HAlignment::Center + align))
                .with_scope(self.scope(styles))
                .with_float(true)
                .pack()
                .spanned(span);
        } else if self.scope(styles) == PlacementScope::Parent {
            bail!(
                span,
                "parent-scoped placement is only available for floating figures";
                hint: "you can enable floating placement with `figure(placement: auto, ..)`"
            );
        }

        Ok(realized)
    }
}

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
impl ShowSet for Packed<FigureElem> {
    fn show_set(&self, _: StyleChain) -> Styles {
        // Still allows breakable figures with
        // `show figure: set block(breakable: true)`.
        let mut map = Styles::new();
<<<<<<< HEAD
        map.set(BlockElem::set_breakable(false));
        map.set(AlignElem::set_alignment(Alignment::CENTER));
=======
        map.set(BlockElem::breakable, false);
        map.set(AlignElem::alignment, Alignment::CENTER);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
        match (**self).supplement(StyleChain::default()).as_ref() {
            Smart::Custom(Some(Supplement::Content(content))) => content.clone(),
=======
        match self.supplement.get_cloned(StyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            _ => Content::empty(),
        }
    }

    fn counter(&self) -> Counter {
<<<<<<< HEAD
        (**self)
            .counter()
            .cloned()
            .flatten()
            .unwrap_or_else(|| Counter::of(FigureElem::elem()))
    }

    fn numbering(&self) -> Option<&Numbering> {
        (**self).numbering(StyleChain::default()).as_ref()
=======
        self.counter
            .clone()
            .flatten()
            .unwrap_or_else(|| Counter::of(FigureElem::ELEM))
    }

    fn numbering(&self) -> Option<&Numbering> {
        self.numbering.get_ref(StyleChain::default()).as_ref()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

impl Outlinable for Packed<FigureElem> {
    fn outlined(&self) -> bool {
<<<<<<< HEAD
        (**self).outlined(StyleChain::default())
            && (self.caption(StyleChain::default()).is_some()
=======
        self.outlined.get(StyleChain::default())
            && (self.caption.get_ref(StyleChain::default()).is_some()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
        self.caption(StyleChain::default())
=======
        self.caption
            .get_ref(StyleChain::default())
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            .as_ref()
            .map(|caption| caption.body.clone())
            .unwrap_or_default()
    }
}

<<<<<<< HEAD
/// 図のキャプション。
/// この要素は、全ての図や特定の種類の図のキャプションの外観を改変するために、
/// setルールやshowルールで使用可能です。
///
/// キャプションは、`pos`と`body`に加えて、図の`kind`や`supplement`、`counter`、`numbering`もフィールドとして提供します。
/// これらの要素を[`where`]($function.where)セレクターやshowルールで使用することで、独自のキャプションを構築できます。
=======
/// The caption of a figure. This element can be used in set and show rules to
/// customize the appearance of captions for all figures or figures of a
/// specific kind.
///
/// In addition to its `position` and `body`, the `caption` also provides the
/// figure's `kind`, `supplement`, `counter`, and `numbering` as fields. These
/// parts can be used in [`where`]($function.where) selectors and show rules to
/// build a completely custom caption.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #show figure.caption: emph
///
/// #figure(
///   rect[Hello],
///   caption: [A rectangle],
/// )
/// ```
<<<<<<< HEAD
#[elem(name = "caption", Synthesize, Show)]
pub struct FigureCaption {
    /// 図表の仲のキャプションの位置。`{top}`や`{bottom}`を入力してください。
=======
#[elem(name = "caption", Locatable, Tagged, Synthesize)]
pub struct FigureCaption {
    /// The caption's position in the figure. Either `{top}` or `{bottom}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// 番号とキャプション名の間に表示する区切り文字。
    ///
    /// `{auto}`に設定すると、区切り文字は
    /// [language]($text.lang)と[region]($text.region)に応じて決まります。
=======
    /// The separator which will appear between the number and body.
    ///
    /// If set to `{auto}`, the separator will be adapted to the current
    /// [language]($text.lang) and [region]($text.region).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// キャプション名。
    ///
    /// 独自のキャプションに改変するために
    /// `kind`、`supplement`、`counter`、`numbering`、`location`が同時に使えます。
=======
    /// The caption's body.
    ///
    /// Can be used alongside `kind`, `supplement`, `counter`, `numbering`, and
    /// `location` to completely customize the caption.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
    /// Gets the default separator in the given language and (optionally)
    /// region.
    fn local_separator(lang: Lang, _: Option<Region>) -> &'static str {
        match lang {
            Lang::CHINESE => "\u{2003}",
            Lang::FRENCH => ".\u{a0}– ",
            Lang::RUSSIAN => ". ",
            Lang::ENGLISH | _ => ": ",
        }
    }

    fn get_separator(&self, styles: StyleChain) -> Content {
        self.separator(styles).unwrap_or_else(|| {
            TextElem::packed(Self::local_separator(
                TextElem::lang_in(styles),
                TextElem::region_in(styles),
            ))
        })
    }
}

impl Synthesize for Packed<FigureCaption> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        let elem = self.as_mut();
        elem.push_separator(Smart::Custom(elem.get_separator(styles)));
        Ok(())
    }
}

impl Show for Packed<FigureCaption> {
    #[typst_macros::time(name = "figure.caption", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
=======
    /// Realizes the textual caption content.
    pub fn realize(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Content> {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        let mut realized = self.body.clone();

        if let (
            Some(Some(mut supplement)),
            Some(Some(numbering)),
            Some(Some(counter)),
            Some(Some(location)),
        ) = (
<<<<<<< HEAD
            self.supplement().cloned(),
            self.numbering(),
            self.counter(),
            self.figure_location(),
=======
            self.supplement.clone(),
            &self.numbering,
            &self.counter,
            &self.figure_location,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        ) {
            let numbers = counter.display_at_loc(engine, *location, styles, numbering)?;
            if !supplement.is_empty() {
                supplement += TextElem::packed('\u{a0}');
            }
<<<<<<< HEAD
            realized = supplement + numbers + self.get_separator(styles) + realized;
        }

        Ok(if TargetElem::target_in(styles).is_html() {
            HtmlElem::new(tag::figcaption)
                .with_body(Some(realized))
                .pack()
                .spanned(self.span())
        } else {
            BlockElem::new()
                .with_body(Some(BlockBody::Content(realized)))
                .pack()
                .spanned(self.span())
=======
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        })
    }
}

<<<<<<< HEAD
=======
impl Synthesize for Packed<FigureCaption> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        let elem = self.as_mut();
        elem.separator.set(Smart::Custom(elem.resolve_separator(styles)));
        Ok(())
    }
}

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
