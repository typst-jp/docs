use std::num::NonZeroUsize;
use std::str::FromStr;

use comemo::{Track, Tracked};
use smallvec::SmallVec;
use typst_syntax::Span;
use typst_utils::{Get, NonZeroExt};

use crate::diag::{bail, error, At, HintedStrResult, SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, func, scope, select_where, Args, Construct, Content, Context, Func,
    LocatableSelector, NativeElement, Packed, Resolve, Show, ShowSet, Smart, StyleChain,
    Styles,
};
use crate::introspection::{
    Counter, CounterKey, Introspector, Locatable, Location, Locator, LocatorLink,
};
use crate::layout::{
    Abs, Axes, BlockBody, BlockElem, BoxElem, Dir, Em, Fr, HElem, Length, Region, Rel,
    RepeatElem, Sides,
};
use crate::math::EquationElem;
use crate::model::{Destination, HeadingElem, NumberingPattern, ParElem, Refable};
use crate::text::{LocalName, SpaceElem, TextElem};

/// 目次や図表などのアウトライン。
///
/// この関数は、指定した[`depth`]($outline.depth)までに登場する要素を文書内から抽出し、その一覧（アウトライン）を生成します。
/// 各要素には、その見出しやキャプションとともに、その要素の番号やページ番号がアウトライン形式で表示されます。
///
/// # 例
/// ```example
/// #set heading(numbering: "1.")
/// #outline()
///
/// = Introduction
/// #lorem(5)
///
/// = Methods
/// == Setup
/// #lorem(10)
/// ```
///
/// # 見出し以外のアウトライン { #alternative-outlines }
/// デフォルト設定では、この関数は目次（セクション見出しのアウトライン）を生成します。
/// `target`パラメーターを設定することで、見出し以外の要素のアウトラインも生成できます。
///
/// 下の例では、`target`を`{figure.where(kind: image)}`に設定して、画像を含む図のみをアウトライン表示しています。
/// 同様に`{figure.where(kind: table)}`と設定すれば、表のアウトラインを生成できます。
///
/// [`where`]($function.where)セレクターを使わずに`figure`のみの指定もできますが、その場合は画像や表、またその他の素材も含む _全て_ の図表がアウトラインに表示されます。
///
/// ```example
/// #outline(
///   title: [List of Figures],
///   target: figure.where(kind: image),
/// )
///
/// #figure(
///   image("tiger.jpg"),
///   caption: [A nice figure!],
/// )
/// ```
///
/// # アウトラインのスタイル { #styling-the-outline }
/// 基本的に、アウトライン本体やその項目に対してプロパティを設定することでスタイルを変更できます。
/// これにより、アウトラインの[タイトル]($outline.title)、項目の[インデント]($outline.indent)、項目のテキストとページ番号の間の[空白の埋め方]($outline.entry.fill)などをカスタマイズできます。
///
/// アウトラインの[項目]($outline.entry)を設定を調整することで、より高度なカスタマイズも可能です。
/// アウトラインは、対象となる各要素に対して1つの項目を生成します。
///
/// ## 項目同士の間隔調整 { #entry-spacing }
/// アウトラインの各項目は[ブロック要素]($block)であるため、通常のブロック間隔設定を用いて、項目同士の間隔を調整できます。
///
/// ```example
/// #show outline.entry.where(
///   level: 1
/// ): set block(above: 1.2em)
///
/// #outline()
///
/// = About ACME Corp.
/// == History
/// === Origins
/// = Products
/// == ACME Tools
/// ```
///
/// ## アウトライン項目の構築 { #building-an-entry }
/// 項目の外観を完全に制御するために、`outline.entry`を変更するshowルールも書けます。
/// ただし、アウトライン項目を適切に書式設定・インデントするための処理は非常に複雑であり、アウトライン項目自体が持つフィールドは「レベル」と「対象要素」の2つのみです。
///
/// そのため、必要な部分だけを組み合わせて項目を構築できるよう、さまざまな補助関数が提供されています。
///
/// アウトライン項目に対する既定のshowルールは次のようになっています[^1]。:
/// ```typ
/// #show outline.entry: it => link(
///   it.element.location(),
///   it.indented(it.prefix(), it.inner()),
/// )
/// ```
///
/// - [`indented`]($outline.entry.indented)関数は、任意のプレフィックスと内部コンテンツを受け取り、適切なインデントを自動的に適用します。
///   これにより、異なる項目同士がきれいに揃い、長い見出しも正しく折り返されます。
///
/// - [`prefix`]($outline.entry.prefix)関数は、要素の番号（存在する場合）を整形します。
///   また、特定の要素には補足語も付加します。
///
/// - [`inner`]($outline.entry.inner)関数は、要素の[`body`]($outline.entry.body)と[`page` number]($outline.entry.page)、およびそれらの間を埋めるフィラー（点線など）を組み合わせます。
///
/// これらの関数を個別に使うことで、アウトライン項目の書式を変更できます。
/// 例えば、フィラーやページ番号を完全に削除したい場合は、次のようなshowルールを書くことができます。
///
/// ```example
/// #show outline.entry: it => link(
///   it.element.location(),
///   // Keep just the body, dropping
///   // the fill and the page.
///   it.indented(it.prefix(), it.body()),
/// )
///
/// #outline()
///
/// = About ACME Corp.
/// == History
/// ```
///
/// [^1]: 数式のアウトラインはこのルールの例外で、本文を持たないためインデント付きのレイアウトは使用しません。
#[elem(scope, keywords = ["Table of Contents", "toc"], Show, ShowSet, LocalName, Locatable)]
pub struct OutlineElem {
    /// アウトラインのタイトル。
    ///
    /// - `{auto}`と設定すると[text language]($text.lang)に従ったタイトル名となります。
    /// - `{none}`と設定すると、タイトルなしとなります。
    /// - 独自のタイトルはコンテンツとして設定できます。
    ///
    /// アウトライン自体はデフォルトでは見出しとして番号付けされません。
    /// 強制的に番号付けしたい場合は、`{show outline: set heading(numbering: "1.")}`のようにshow-setルールを使います。
    pub title: Smart<Option<Content>>,

    /// アウトラインにする要素の種類。
    ///
    /// 特定の種類の要素（画像や表など）のみを含む図表をアウトライン表示したい場合は、[`where`]($function.where)セレクターで目的の種類を指定できます。
    /// 詳細は[見出し以外のアウトライン]($outline/#alternative-outlines)のセクションをご参照ください。
    ///
    /// ```example
    /// #outline(
    ///   title: [List of Tables],
    ///   target: figure.where(kind: table),
    /// )
    ///
    /// #figure(
    ///   table(
    ///     columns: 4,
    ///     [t], [1], [2], [3],
    ///     [y], [0.3], [0.7], [0.5],
    ///   ),
    ///   caption: [Experiment results],
    /// )
    /// ```
    #[default(LocatableSelector(HeadingElem::elem().select()))]
    #[borrowed]
    pub target: LocatableSelector,

    /// アウトラインに含める要素の最大レベル。
    /// この引数が`{none}`の場合は、全ての要素が含まれます。
    ///
    /// ```example
    /// #set heading(numbering: "1.")
    /// #outline(depth: 2)
    ///
    /// = Yes
    /// Top-level section.
    ///
    /// == Still
    /// Subsection.
    ///
    /// === Nope
    /// Not included.
    /// ```
    pub depth: Option<NonZeroUsize>,

    /// どのようにアウトライン項目をインデントするか。
    ///
    /// - `{auto}`: 入れ子になった項目の番号やプレフィックスを、親項目のタイトル位置に揃えてインデントします。
    ///   例えば[見出しの番号付け]($heading.numbering)で項目が番号付きとしない設定をしている場合には、レベルに応じて単純に固定幅`{1.2em}`のインデントを追加します。
    ///
    /// - [相対長さ]($relative): ネストレベルごとに指定した長さ分だけインデントします。
    ///   具体例として`{2em}`と指定すると、最上位レベル（ネストなし）のインデントは`{0em}`、第2レベル（1段階のネスト）のインデントは`{2em}`、第3レベル（2段階のネスト）は`{4em}`といった具合に設定されます。
    ///
    /// - [関数]($function): 関数を使ってさらに細かくカスタマイズできます。
    ///   関数はネストレベルが引数として渡され（最上位要素は0）、相対長さを返します。
    ///   例えば`{n => n * 2em}`とすれば単に`{2em}`を指定した場合と同じ結果となります。
    ///
    /// ```example
    /// #set heading(numbering: "1.a.")
    ///
    /// #outline(
    ///   title: [Contents (Automatic)],
    ///   indent: auto,
    /// )
    ///
    /// #outline(
    ///   title: [Contents (Length)],
    ///   indent: 2em,
    /// )
    ///
    /// = About ACME Corp.
    /// == History
    /// === Origins
    /// #lorem(10)
    ///
    /// == Products
    /// #lorem(10)
    /// ```
    pub indent: Smart<OutlineIndent>,
}

#[scope]
impl OutlineElem {
    #[elem]
    type OutlineEntry;
}

impl Show for Packed<OutlineElem> {
    #[typst_macros::time(name = "outline", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let span = self.span();

        // Build the outline title.
        let mut seq = vec![];
        if let Some(title) = self.title(styles).unwrap_or_else(|| {
            Some(TextElem::packed(Self::local_name_in(styles)).spanned(span))
        }) {
            seq.push(
                HeadingElem::new(title)
                    .with_depth(NonZeroUsize::ONE)
                    .pack()
                    .spanned(span),
            );
        }

        let elems = engine.introspector.query(&self.target(styles).0);
        let depth = self.depth(styles).unwrap_or(NonZeroUsize::MAX);

        // Build the outline entries.
        for elem in elems {
            let Some(outlinable) = elem.with::<dyn Outlinable>() else {
                bail!(span, "cannot outline {}", elem.func().name());
            };

            let level = outlinable.level();
            if outlinable.outlined() && level <= depth {
                let entry = OutlineEntry::new(level, elem);
                seq.push(entry.pack().spanned(span));
            }
        }

        Ok(Content::sequence(seq))
    }
}

impl ShowSet for Packed<OutlineElem> {
    fn show_set(&self, styles: StyleChain) -> Styles {
        let mut out = Styles::new();
        out.set(HeadingElem::set_outlined(false));
        out.set(HeadingElem::set_numbering(None));
        out.set(ParElem::set_justify(false));
        out.set(BlockElem::set_above(Smart::Custom(ParElem::leading_in(styles).into())));
        // Makes the outline itself available to its entries. Should be
        // superseded by a proper ancestry mechanism in the future.
        out.set(OutlineEntry::set_parent(Some(self.clone())));
        out
    }
}

impl LocalName for Packed<OutlineElem> {
    const KEY: &'static str = "outline";
}

/// Defines how an outline is indented.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum OutlineIndent {
    /// Indents by the specified length per level.
    Rel(Rel),
    /// Resolve the indent for a specific level through the given function.
    Func(Func),
}

impl OutlineIndent {
    /// Resolve the indent for an entry with the given level.
    fn resolve(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        level: NonZeroUsize,
        span: Span,
    ) -> SourceResult<Rel> {
        let depth = level.get() - 1;
        match self {
            Self::Rel(length) => Ok(*length * depth as f64),
            Self::Func(func) => func.call(engine, context, [depth])?.cast().at(span),
        }
    }
}

cast! {
    OutlineIndent,
    self => match self {
        Self::Rel(v) => v.into_value(),
        Self::Func(v) => v.into_value()
    },
    v: Rel<Length> => Self::Rel(v),
    v: Func => Self::Func(v),
}

/// Marks an element as being able to be outlined.
pub trait Outlinable: Refable {
    /// Whether this element should be included in the outline.
    fn outlined(&self) -> bool;

    /// The nesting level of this element.
    fn level(&self) -> NonZeroUsize {
        NonZeroUsize::ONE
    }

    /// Constructs the default prefix given the formatted numbering.
    fn prefix(&self, numbers: Content) -> Content;

    /// The body of the entry.
    fn body(&self) -> Content;
}

/// アウトライン内の項目。
///
/// show-setルールやshowルールをアウトライン項目に適用することで、アウトラインの見た目を柔軟にカスタマイズできます。
/// 詳細は[アウトラインのスタイルのセクション]($outline/#styling-the-outline)をご参照ください。
#[elem(scope, name = "entry", title = "Outline Entry", Show)]
pub struct OutlineEntry {
    /// アウトライン項目のネストレベル。
    /// 最上位のネストレベルは`{1}`から始まります。
    #[required]
    pub level: NonZeroUsize,

    /// この項目が参照している要素。
    /// 要素の位置は、コンテンツの[`location`]($content.location)メソッドで取得でき、これに対する[linked]($link)も使用可能です。
    #[required]
    pub element: Content,

    /// タイトルとページ番号の間を埋めるためのコンテンツ。
    /// コンテンツで埋めない場合には`{none}`を指定できます。
    ///
    /// `fill`は、項目の本文とページ番号の間をまたぐ可変幅のボックスに配置されます。
    /// そのため、アウトライン項目をshowルールで上書きする場合は、fillを`{box(width: 1fr, it.fill}`のように可変長の[`box`]でラップすることが推奨されます。
    ///
    /// [`repeat`]を使う場合には、[`gap`]($repeat.gap)プロパティを調整すると、fillの見た目を微調整できます。
    ///
    /// ```example
    /// #set outline.entry(fill: line(length: 100%))
    /// #outline()
    ///
    /// = A New Beginning
    /// ```
    #[borrowed]
    #[default(Some(
        RepeatElem::new(TextElem::packed("."))
            .with_gap(Em::new(0.15).into())
            .pack()
    ))]
    pub fill: Option<Content>,

    /// Lets outline entries access the outline they are part of. This is a bit
    /// of a hack and should be superseded by a proper ancestry mechanism.
    #[ghost]
    #[internal]
    pub parent: Option<Packed<OutlineElem>>,
}

impl Show for Packed<OutlineEntry> {
    #[typst_macros::time(name = "outline.entry", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let span = self.span();
        let context = Context::new(None, Some(styles));
        let context = context.track();

        let prefix = self.prefix(engine, context, span)?;
        let inner = self.inner(engine, context, span)?;
        let block = if self.element.is::<EquationElem>() {
            let body = prefix.unwrap_or_default() + inner;
            BlockElem::new()
                .with_body(Some(BlockBody::Content(body)))
                .pack()
                .spanned(span)
        } else {
            self.indented(engine, context, span, prefix, inner, Em::new(0.5).into())?
        };

        let loc = self.element_location().at(span)?;
        Ok(block.linked(Destination::Location(loc)))
    }
}

#[scope]
impl OutlineEntry {
    /// インデント付きの項目レイアウトを作成するための補助関数。
    /// プレフィックスと項目本文を、インデントを考慮して配置します。
    ///
    /// 親アウトラインの[`indent`]($outline.indent)が`{auto}`の場合、レベル`N`の項目の内部（inner）コンテンツは、レベル`N + 1`の項目のプレフィックスに揃えられ、プレフィックスと内部コンテンツの間には最低でも`gap`分のスペースが空けられます。
    /// さらに、同じレベルの全ての項目の`inner`コンテンツも整列されます。
    ///
    /// アウトラインのインデントが固定値または関数に設定されている場合、プレフィックスはインデントされますが、内部コンテンツはアウトライン全体で整列されるのではなく、指定された`gap`分だけプレフィックスからオフセットされます。
    #[func(contextual)]
    pub fn indented(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// `prefix`は、レベルが1段低い項目の`inner`コンテンツと揃うように配置されます。
        ///
        /// デフォルトのshowルールでは`it.prefix()`ですが、自由にカスタマイズできます。
        prefix: Option<Content>,
        /// 項目のフォーマットされた内部コンテンツ。
        ///
        /// デフォルトのshowルールでは`it.inner()`ですが、自由にカスタマイズできます。
        inner: Content,
        /// プレフィックスと内部コンテンツの間の間隔です。
        #[named]
        #[default(Em::new(0.5).into())]
        gap: Length,
    ) -> SourceResult<Content> {
        let styles = context.styles().at(span)?;
        let outline = Self::parent_in(styles)
            .ok_or("must be called within the context of an outline")
            .at(span)?;
        let outline_loc = outline.location().unwrap();

        let prefix_width = prefix
            .as_ref()
            .map(|prefix| measure_prefix(engine, prefix, outline_loc, styles))
            .transpose()?;
        let prefix_inset = prefix_width.map(|w| w + gap.resolve(styles));

        let indent = outline.indent(styles);
        let (base_indent, hanging_indent) = match &indent {
            Smart::Auto => compute_auto_indents(
                engine.introspector,
                outline_loc,
                styles,
                self.level,
                prefix_inset,
            ),
            Smart::Custom(amount) => {
                let base = amount.resolve(engine, context, self.level, span)?;
                (base, prefix_inset)
            }
        };

        let body = if let (
            Some(prefix),
            Some(prefix_width),
            Some(prefix_inset),
            Some(hanging_indent),
        ) = (prefix, prefix_width, prefix_inset, hanging_indent)
        {
            // Save information about our prefix that other outline entries
            // can query for (within `compute_auto_indent`) to align
            // themselves).
            let mut seq = Vec::with_capacity(5);
            if indent.is_auto() {
                seq.push(PrefixInfo::new(outline_loc, self.level, prefix_inset).pack());
            }

            // Dedent the prefix by the amount of hanging indent and then skip
            // ahead so that the inner contents are aligned.
            seq.extend([
                HElem::new((-hanging_indent).into()).pack(),
                prefix,
                HElem::new((hanging_indent - prefix_width).into()).pack(),
                inner,
            ]);
            Content::sequence(seq)
        } else {
            inner
        };

        let inset = Sides::default().with(
            TextElem::dir_in(styles).start(),
            Some(base_indent + Rel::from(hanging_indent.unwrap_or_default())),
        );

        Ok(BlockElem::new()
            .with_inset(inset)
            .with_body(Some(BlockBody::Content(body)))
            .pack()
            .spanned(span))
    }

    /// 要素の番号付け（存在する場合）の出力形式。
    ///
    /// また、図や数式の場合は、通常のアウトラインと同様に要素の補足語も追加します。
    /// 例えば、見出しであれば`1.1`と出力されますが、図であれば`Figure 1`と出力されます。
    #[func(contextual)]
    pub fn prefix(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<Option<Content>> {
        let outlinable = self.outlinable().at(span)?;
        let Some(numbering) = outlinable.numbering() else { return Ok(None) };
        let loc = self.element_location().at(span)?;
        let styles = context.styles().at(span)?;
        let numbers =
            outlinable.counter().display_at_loc(engine, loc, styles, numbering)?;
        Ok(Some(outlinable.prefix(numbers)))
    }

    /// 項目の規定の内部コンテンツを生成。
    ///
    /// これには、本文とフィラー、ページ番号が含まれます。
    #[func(contextual)]
    pub fn inner(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<Content> {
        let styles = context.styles().at(span)?;

        let mut seq = vec![];

        // Isolate the entry body in RTL because the page number is typically
        // LTR. I'm not sure whether LTR should conceptually also be isolated,
        // but in any case we don't do it for now because the text shaping
        // pipeline does tend to choke a bit on default ignorables (in
        // particular the CJK-Latin spacing).
        //
        // See also:
        // - https://github.com/typst/typst/issues/4476
        // - https://github.com/typst/typst/issues/5176
        let rtl = TextElem::dir_in(styles) == Dir::RTL;
        if rtl {
            // "Right-to-Left Embedding"
            seq.push(TextElem::packed("\u{202B}"));
        }

        seq.push(self.body().at(span)?);

        if rtl {
            // "Pop Directional Formatting"
            seq.push(TextElem::packed("\u{202C}"));
        }

        // Add the filler between the section name and page number.
        if let Some(filler) = self.fill(styles) {
            seq.push(SpaceElem::shared().clone());
            seq.push(
                BoxElem::new()
                    .with_body(Some(filler.clone()))
                    .with_width(Fr::one().into())
                    .pack()
                    .spanned(span),
            );
            seq.push(SpaceElem::shared().clone());
        } else {
            seq.push(HElem::new(Fr::one().into()).pack().spanned(span));
        }

        // Add the page number. The word joiner in front ensures that the page
        // number doesn't stand alone in its line.
        seq.push(TextElem::packed("\u{2060}"));
        seq.push(self.page(engine, context, span)?);

        Ok(Content::sequence(seq))
    }

    /// アウトライン内で参照される要素の代わりに表示される内容。
    /// 見出しの場合は[`body`]($heading.body)、図表の場合はキャプション、数式の場合は空欄となります。
    #[func]
    pub fn body(&self) -> StrResult<Content> {
        Ok(self.outlinable()?.body())
    }

    /// 要素の項目のページ番号。
    /// これは参照されるページに対する番号付け設定の書式で出力されます。
    #[func(contextual)]
    pub fn page(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<Content> {
        let loc = self.element_location().at(span)?;
        let styles = context.styles().at(span)?;
        let numbering = engine
            .introspector
            .page_numbering(loc)
            .cloned()
            .unwrap_or_else(|| NumberingPattern::from_str("1").unwrap().into());
        Counter::new(CounterKey::Page).display_at_loc(engine, loc, styles, &numbering)
    }
}

impl OutlineEntry {
    fn outlinable(&self) -> StrResult<&dyn Outlinable> {
        self.element
            .with::<dyn Outlinable>()
            .ok_or_else(|| error!("cannot outline {}", self.element.func().name()))
    }

    fn element_location(&self) -> HintedStrResult<Location> {
        let elem = &self.element;
        elem.location().ok_or_else(|| {
            if elem.can::<dyn Locatable>() && elem.can::<dyn Outlinable>() {
                error!(
                    "{} must have a location", elem.func().name();
                    hint: "try using a show rule to customize the outline.entry instead",
                )
            } else {
                error!("cannot outline {}", elem.func().name())
            }
        })
    }
}

cast! {
    OutlineEntry,
    v: Content => v.unpack::<Self>().map_err(|_| "expected outline entry")?
}

/// Measures the width of a prefix.
fn measure_prefix(
    engine: &mut Engine,
    prefix: &Content,
    loc: Location,
    styles: StyleChain,
) -> SourceResult<Abs> {
    let pod = Region::new(Axes::splat(Abs::inf()), Axes::splat(false));
    let link = LocatorLink::measure(loc);
    Ok((engine.routines.layout_frame)(engine, prefix, Locator::link(&link), styles, pod)?
        .width())
}

/// Compute the base indent and hanging indent for an auto-indented outline
/// entry of the given level, with the given prefix inset.
fn compute_auto_indents(
    introspector: Tracked<Introspector>,
    outline_loc: Location,
    styles: StyleChain,
    level: NonZeroUsize,
    prefix_inset: Option<Abs>,
) -> (Rel, Option<Abs>) {
    let indents = query_prefix_widths(introspector, outline_loc);

    let fallback = Em::new(1.2).resolve(styles);
    let get = |i: usize| indents.get(i).copied().flatten().unwrap_or(fallback);

    let last = level.get() - 1;
    let base: Abs = (0..last).map(get).sum();
    let hang = prefix_inset.map(|p| p.max(get(last)));

    (base.into(), hang)
}

/// Determines the maximum prefix inset (prefix width + gap) at each outline
/// level, for the outline with the given `loc`. Levels for which there is no
/// information available yield `None`.
#[comemo::memoize]
fn query_prefix_widths(
    introspector: Tracked<Introspector>,
    outline_loc: Location,
) -> SmallVec<[Option<Abs>; 4]> {
    let mut widths = SmallVec::<[Option<Abs>; 4]>::new();
    let elems = introspector.query(&select_where!(PrefixInfo, Key => outline_loc));
    for elem in &elems {
        let info = elem.to_packed::<PrefixInfo>().unwrap();
        let level = info.level.get();
        if widths.len() < level {
            widths.resize(level, None);
        }
        widths[level - 1].get_or_insert(info.inset).set_max(info.inset);
    }
    widths
}

/// Helper type for introspection-based prefix alignment.
#[elem(Construct, Locatable, Show)]
struct PrefixInfo {
    /// The location of the outline this prefix is part of. This is used to
    /// scope prefix computations to a specific outline.
    #[required]
    key: Location,

    /// The level of this prefix's entry.
    #[required]
    #[internal]
    level: NonZeroUsize,

    /// The width of the prefix, including the gap.
    #[required]
    #[internal]
    inset: Abs,
}

impl Construct for PrefixInfo {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

impl Show for Packed<PrefixInfo> {
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(Content::empty())
    }
}
