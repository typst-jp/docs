use std::num::NonZeroUsize;
use std::str::FromStr;

<<<<<<< HEAD
use comemo::{Track, Tracked};
=======
use comemo::Tracked;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use smallvec::SmallVec;
use typst_syntax::Span;
use typst_utils::{Get, NonZeroExt};

<<<<<<< HEAD
use crate::diag::{bail, error, At, HintedStrResult, SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, func, scope, select_where, Args, Construct, Content, Context, Func,
    LocatableSelector, NativeElement, Packed, Resolve, Show, ShowSet, Smart, StyleChain,
    Styles,
};
use crate::introspection::{
    Counter, CounterKey, Introspector, Locatable, Location, Locator, LocatorLink,
=======
use crate::diag::{At, HintedStrResult, SourceResult, StrResult, bail, error};
use crate::engine::Engine;
use crate::foundations::{
    Args, Construct, Content, Context, Func, LocatableSelector, NativeElement, Packed,
    Resolve, ShowSet, Smart, StyleChain, Styles, cast, elem, func, scope, select_where,
};
use crate::introspection::{
    Counter, CounterKey, Introspector, Locatable, Location, Locator, LocatorLink, Tagged,
    Unqueriable,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
};
use crate::layout::{
    Abs, Axes, BlockBody, BlockElem, BoxElem, Dir, Em, Fr, HElem, Length, Region, Rel,
    RepeatElem, Sides,
};
<<<<<<< HEAD
use crate::math::EquationElem;
use crate::model::{Destination, HeadingElem, NumberingPattern, ParElem, Refable};
use crate::text::{LocalName, SpaceElem, TextElem};

/// 目次や図表などのアウトライン。
///
/// この関数は、指定した[`depth`]($outline.depth)までに登場する要素を文書内から抽出し、その一覧（アウトライン）を生成します。
/// 各要素には、その見出しやキャプションとともに、その要素の番号やページ番号がアウトライン形式で表示されます。
///
/// # 例
=======
use crate::model::{HeadingElem, NumberingPattern, ParElem, Refable};
use crate::pdf::PdfMarkerTag;
use crate::text::{LocalName, SpaceElem, TextElem};

/// A table of contents, figures, or other elements.
///
/// This function generates a list of all occurrences of an element in the
/// document, up to a given [`depth`]($outline.depth). The element's numbering
/// and page number will be displayed in the outline alongside its title or
/// caption.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// # 見出し以外のアウトライン { #alternative-outlines }
/// デフォルト設定では、この関数は目次（セクション見出しのアウトライン）を生成します。
/// `target`パラメーターを設定することで、見出し以外の要素のアウトラインも生成できます。
///
/// 下の例では、`target`を`{figure.where(kind: image)}`に設定して、画像を含む図のみをアウトライン表示しています。
/// 同様に`{figure.where(kind: table)}`と設定すれば、表のアウトラインを生成できます。
///
/// [`where`]($function.where)セレクターを使わずに`figure`のみの指定もできますが、その場合は画像や表、またその他の素材も含む _全て_ の図表がアウトラインに表示されます。
=======
/// # Alternative outlines
/// In its default configuration, this function generates a table of contents.
/// By setting the `target` parameter, the outline can be used to generate a
/// list of other kinds of elements than headings.
///
/// In the example below, we list all figures containing images by setting
/// `target` to `{figure.where(kind: image)}`. Just the same, we could have set
/// it to `{figure.where(kind: table)}` to generate a list of tables.
///
/// We could also set it to just `figure`, without using a [`where`]($function.where)
/// selector, but then the list would contain _all_ figures, be it ones
/// containing images, tables, or other material.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// # アウトラインのスタイル { #styling-the-outline }
/// 基本的に、アウトライン本体やその項目に対してプロパティを設定することでスタイルを変更できます。
/// これにより、アウトラインの[タイトル]($outline.title)、項目の[インデント]($outline.indent)、項目のテキストとページ番号の間の[空白の埋め方]($outline.entry.fill)などをカスタマイズできます。
///
/// アウトラインの[項目]($outline.entry)を設定を調整することで、より高度なカスタマイズも可能です。
/// アウトラインは、対象となる各要素に対して1つの項目を生成します。
///
/// ## 項目同士の間隔調整 { #entry-spacing }
/// アウトラインの各項目は[ブロック要素]($block)であるため、通常のブロック間隔設定を用いて、項目同士の間隔を調整できます。
=======
/// # Styling the outline
/// At the most basic level, you can style the outline by setting properties on
/// it and its entries. This way, you can customize the outline's
/// [title]($outline.title), how outline entries are
/// [indented]($outline.indent), and how the space between an entry's text and
/// its page number should be [filled]($outline.entry.fill).
///
/// Richer customization is possible through configuration of the outline's
/// [entries]($outline.entry). The outline generates one entry for each outlined
/// element.
///
/// ## Spacing the entries { #entry-spacing }
/// Outline entries are [blocks]($block), so you can adjust the spacing between
/// them with normal block-spacing rules:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
/// ## アウトライン項目の構築 { #building-an-entry }
/// 項目の外観を完全に制御するために、`outline.entry`を変更するshowルールも書けます。
/// ただし、アウトライン項目を適切に書式設定・インデントするための処理は非常に複雑であり、アウトライン項目自体が持つフィールドは「レベル」と「対象要素」の2つのみです。
///
/// そのため、必要な部分だけを組み合わせて項目を構築できるよう、さまざまな補助関数が提供されています。
///
/// アウトライン項目に対する既定のshowルールは次のようになっています[^1]。:
=======
/// ## Building an outline entry from its parts { #building-an-entry }
/// For full control, you can also write a transformational show rule on
/// `outline.entry`. However, the logic for properly formatting and indenting
/// outline entries is quite complex and the outline entry itself only contains
/// two fields: The level and the outlined element.
///
/// For this reason, various helper functions are provided. You can mix and
/// match these to compose an entry from just the parts you like.
///
/// The default show rule for an outline entry looks like this[^1]:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```typ
/// #show outline.entry: it => link(
///   it.element.location(),
///   it.indented(it.prefix(), it.inner()),
/// )
/// ```
///
<<<<<<< HEAD
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
=======
/// - The [`indented`]($outline.entry.indented) function takes an optional
///   prefix and inner content and automatically applies the proper indentation
///   to it, such that different entries align nicely and long headings wrap
///   properly.
///
/// - The [`prefix`]($outline.entry.prefix) function formats the element's
///   numbering (if any). It also appends a supplement for certain elements.
///
/// - The [`inner`]($outline.entry.inner) function combines the element's
///   [`body`]($outline.entry.body), the filler, and the
///   [`page` number]($outline.entry.page).
///
/// You can use these individual functions to format the outline entry in
/// different ways. Let's say, you'd like to fully remove the filler and page
/// numbers. To achieve this, you could write a show rule like this:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
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
=======
/// [^1]: The outline of equations is the exception to this rule as it does not
///       have a body and thus does not use indented layout.
#[elem(scope, keywords = ["Table of Contents", "toc"], ShowSet, LocalName, Locatable, Tagged)]
pub struct OutlineElem {
    /// The title of the outline.
    ///
    /// - When set to `{auto}`, an appropriate title for the
    ///   [text language]($text.lang) will be used.
    /// - When set to `{none}`, the outline will not have a title.
    /// - A custom title can be set by passing content.
    ///
    /// The outline's heading will not be numbered by default, but you can
    /// force it to be with a show-set rule:
    /// `{show outline: set heading(numbering: "1.")}`
    pub title: Smart<Option<Content>>,

    /// The type of element to include in the outline.
    ///
    /// To list figures containing a specific kind of element, like an image or
    /// a table, you can specify the desired kind in a [`where`]($function.where)
    /// selector. See the section on [alternative outlines]($outline/#alternative-outlines)
    /// for more details.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
    #[default(LocatableSelector(HeadingElem::elem().select()))]
    #[borrowed]
    pub target: LocatableSelector,

    /// アウトラインに含める要素の最大レベル。
    /// この引数が`{none}`の場合は、全ての要素が含まれます。
=======
    #[default(LocatableSelector(HeadingElem::ELEM.select()))]
    pub target: LocatableSelector,

    /// The maximum level up to which elements are included in the outline. When
    /// this argument is `{none}`, all elements are included.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
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
=======
    /// How to indent the outline's entries.
    ///
    /// - `{auto}`: Indents the numbering/prefix of a nested entry with the
    ///   title of its parent entry. If the entries are not numbered (e.g., via
    ///   [heading numbering]($heading.numbering)), this instead simply inserts
    ///   a fixed amount of `{1.2em}` indent per level.
    ///
    /// - [Relative length]($relative): Indents the entry by the specified
    ///   length per nesting level. Specifying `{2em}`, for instance, would
    ///   indent top-level headings by `{0em}` (not nested), second level
    ///   headings by `{2em}` (nested once), third-level headings by `{4em}`
    ///   (nested twice) and so on.
    ///
    /// - [Function]($function): You can further customize this setting with a
    ///   function. That function receives the nesting level as a parameter
    ///   (starting at 0 for top-level headings/elements) and should return a
    ///   (relative) length. For example, `{n => n * 2em}` would be equivalent
    ///   to just specifying `{2em}`.
    ///
    /// ```example
    /// >>> #show heading: none
    /// #set heading(numbering: "I-I.")
    /// #set outline(title: none)
    ///
    /// #outline()
    /// #line(length: 100%)
    /// #outline(indent: 3em)
    ///
    /// = Software engineering technologies
    /// == Requirements
    /// == Tools and technologies
    /// === Code editors
    /// == Analyzing alternatives
    /// = Designing software components
    /// = Testing and integration
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// ```
    pub indent: Smart<OutlineIndent>,
}

#[scope]
impl OutlineElem {
    #[elem]
    type OutlineEntry;
}

<<<<<<< HEAD
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
=======
impl Packed<OutlineElem> {
    /// Produces the heading for the outline, if any.
    pub fn realize_title(&self, styles: StyleChain) -> Option<Content> {
        let span = self.span();
        self.title
            .get_cloned(styles)
            .unwrap_or_else(|| {
                Some(
                    TextElem::packed(Packed::<OutlineElem>::local_name_in(styles))
                        .spanned(span),
                )
            })
            .map(|title| {
                HeadingElem::new(title)
                    .with_depth(NonZeroUsize::ONE)
                    .pack()
                    .spanned(span)
            })
    }

    /// Realizes the entries in a flat fashion.
    pub fn realize_flat(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Vec<Packed<OutlineEntry>>> {
        let mut entries = vec![];
        for result in self.realize_iter(engine, styles) {
            let (entry, _, included) = result?;
            if included {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

    /// Realizes the entries in a tree fashion.
    pub fn realize_tree(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Vec<OutlineNode>> {
        let flat = self.realize_iter(engine, styles).collect::<SourceResult<Vec<_>>>()?;
        Ok(OutlineNode::build_tree(flat))
    }

    /// Realizes the entries as a lazy iterator.
    fn realize_iter(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> impl Iterator<Item = SourceResult<(Packed<OutlineEntry>, NonZeroUsize, bool)>>
    {
        let span = self.span();
        let elems = engine.introspector.query(&self.target.get_ref(styles).0);
        let depth = self.depth.get(styles).unwrap_or(NonZeroUsize::MAX);
        elems.into_iter().map(move |elem| {
            let Some(outlinable) = elem.with::<dyn Outlinable>() else {
                bail!(self.span(), "cannot outline {}", elem.func().name());
            };
            let level = outlinable.level();
            let include = outlinable.outlined() && level <= depth;
            let entry = Packed::new(OutlineEntry::new(level, elem)).spanned(span);
            Ok((entry, level, include))
        })
    }
}

/// A node in a tree of outline entry.
#[derive(Debug)]
pub struct OutlineNode<T = Packed<OutlineEntry>> {
    /// The entry itself.
    pub entry: T,
    /// The entry's level.
    pub level: NonZeroUsize,
    /// Its descendants.
    pub children: Vec<OutlineNode<T>>,
}

impl<T> OutlineNode<T> {
    /// Turns a flat list of entries into a tree.
    ///
    /// Each entry in the iterator should be accompanied by
    /// - a level
    /// - a boolean indicating whether it is included (`true`) or skipped (`false`)
    pub fn build_tree(
        flat: impl IntoIterator<Item = (T, NonZeroUsize, bool)>,
    ) -> Vec<Self> {
        // Stores the level of the topmost skipped ancestor of the next included
        // heading.
        let mut last_skipped_level = None;
        let mut tree: Vec<OutlineNode<T>> = vec![];

        for (entry, level, include) in flat {
            if include {
                let mut children = &mut tree;

                // Descend the tree through the latest included heading of each
                // level until either:
                // - reaching a node whose children would be siblings of this
                //   heading (=> add the current heading as a child of this
                //   node)
                // - reaching a node with no children (=> this heading probably
                //   skipped a few nesting levels in Typst, or one or more
                //   ancestors of this heading weren't included, so add it as a
                //   child of this node, which is its deepest included ancestor)
                // - or, if the latest heading(s) was(/were) skipped, then stop
                //   if reaching a node whose children would be siblings of the
                //   latest skipped heading of lowest level (=> those skipped
                //   headings would be ancestors of the current heading, so add
                //   it as a sibling of the least deep skipped ancestor among
                //   them, as those ancestors weren't added to the tree, and the
                //   current heading should not be mistakenly added as a
                //   descendant of a siblibg of that ancestor.)
                //
                // That is, if you had an included heading of level N, a skipped
                // heading of level N, a skipped heading of level N + 1, and
                // then an included heading of level N + 2, that last one is
                // included as a level N heading (taking the place of its
                // topmost skipped ancestor), so that it is not mistakenly added
                // as a descendant of the previous level N heading.
                while children.last().is_some_and(|last| {
                    last_skipped_level.is_none_or(|l| last.level < l)
                        && last.level < level
                }) {
                    children = &mut children.last_mut().unwrap().children;
                }

                // Since this heading was bookmarked, the next heading (if it is
                // a child of this one) won't have a skipped direct ancestor.
                last_skipped_level = None;
                children.push(OutlineNode { entry, level, children: vec![] });
            } else if last_skipped_level.is_none_or(|l| level < l) {
                // Only the topmost / lowest-level skipped heading matters when
                // we have consecutive skipped headings, hence the condition
                // above.
                last_skipped_level = Some(level);
            }
        }

        tree
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

impl ShowSet for Packed<OutlineElem> {
    fn show_set(&self, styles: StyleChain) -> Styles {
        let mut out = Styles::new();
<<<<<<< HEAD
        out.set(HeadingElem::set_outlined(false));
        out.set(HeadingElem::set_numbering(None));
        out.set(ParElem::set_justify(false));
        out.set(BlockElem::set_above(Smart::Custom(ParElem::leading_in(styles).into())));
        // Makes the outline itself available to its entries. Should be
        // superseded by a proper ancestry mechanism in the future.
        out.set(OutlineEntry::set_parent(Some(self.clone())));
=======
        out.set(HeadingElem::outlined, false);
        out.set(HeadingElem::numbering, None);
        out.set(ParElem::justify, false);
        out.set(BlockElem::above, Smart::Custom(styles.get(ParElem::leading).into()));
        // Makes the outline itself available to its entries. Should be
        // superseded by a proper ancestry mechanism in the future.
        out.set(OutlineEntry::parent, Some(self.clone()));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
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
=======
/// Represents an entry line in an outline.
///
/// With show-set and show rules on outline entries, you can richly customize
/// the outline's appearance. See the
/// [section on styling the outline]($outline/#styling-the-outline) for details.
#[elem(scope, name = "entry", title = "Outline Entry", Locatable, Tagged)]
pub struct OutlineEntry {
    /// The nesting level of this outline entry. Starts at `{1}` for top-level
    /// entries.
    #[required]
    pub level: NonZeroUsize,

    /// The element this entry refers to. Its location will be available
    /// through the [`location`]($content.location) method on the content
    /// and can be [linked]($link) to.
    #[required]
    pub element: Content,

    /// Content to fill the space between the title and the page number. Can be
    /// set to `{none}` to disable filling.
    ///
    /// The `fill` will be placed into a fractionally sized box that spans the
    /// space between the entry's body and the page number. When using show
    /// rules to override outline entries, it is thus recommended to wrap the
    /// fill in a [`box`] with fractional width, i.e.
    /// `{box(width: 1fr, it.fill)}`.
    ///
    /// When using [`repeat`], the [`gap`]($repeat.gap) property can be useful
    /// to tweak the visual weight of the fill.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set outline.entry(fill: line(length: 100%))
    /// #outline()
    ///
    /// = A New Beginning
    /// ```
<<<<<<< HEAD
    #[borrowed]
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
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
=======
#[scope]
impl OutlineEntry {
    /// A helper function for producing an indented entry layout: Lays out a
    /// prefix and the rest of the entry in an indent-aware way.
    ///
    /// If the parent outline's [`indent`]($outline.indent) is `{auto}`, the
    /// inner content of all entries at level `N` is aligned with the prefix of
    /// all entries at level `N + 1`, leaving at least `gap` space between the
    /// prefix and inner parts. Furthermore, the `inner` contents of all entries
    /// at the same level are aligned.
    ///
    /// If the outline's indent is a fixed value or a function, the prefixes are
    /// indented, but the inner contents are simply offset from the prefix by
    /// the specified `gap`, rather than aligning outline-wide. For a visual
    /// explanation, see [`outline.indent`].
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func(contextual)]
    pub fn indented(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
<<<<<<< HEAD
        /// `prefix`は、レベルが1段低い項目の`inner`コンテンツと揃うように配置されます。
        ///
        /// デフォルトのshowルールでは`it.prefix()`ですが、自由にカスタマイズできます。
        prefix: Option<Content>,
        /// 項目のフォーマットされた内部コンテンツ。
        ///
        /// デフォルトのshowルールでは`it.inner()`ですが、自由にカスタマイズできます。
        inner: Content,
        /// プレフィックスと内部コンテンツの間の間隔です。
=======
        /// The `prefix` is aligned with the `inner` content of entries that
        /// have level one less.
        ///
        /// In the default show rule, this is just `it.prefix()`, but it can be
        /// freely customized.
        prefix: Option<Content>,
        /// The formatted inner content of the entry.
        ///
        /// In the default show rule, this is just `it.inner()`, but it can be
        /// freely customized.
        inner: Content,
        /// The gap between the prefix and the inner content.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[named]
        #[default(Em::new(0.5).into())]
        gap: Length,
    ) -> SourceResult<Content> {
        let styles = context.styles().at(span)?;
<<<<<<< HEAD
        let outline = Self::parent_in(styles)
=======
        let outline = styles
            .get_ref(Self::parent)
            .as_ref()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            .ok_or("must be called within the context of an outline")
            .at(span)?;
        let outline_loc = outline.location().unwrap();

        let prefix_width = prefix
            .as_ref()
            .map(|prefix| measure_prefix(engine, prefix, outline_loc, styles))
            .transpose()?;
        let prefix_inset = prefix_width.map(|w| w + gap.resolve(styles));

<<<<<<< HEAD
        let indent = outline.indent(styles);
=======
        let indent = outline.indent.get_ref(styles);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
                prefix,
=======
                PdfMarkerTag::Label(prefix),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                HElem::new((hanging_indent - prefix_width).into()).pack(),
                inner,
            ]);
            Content::sequence(seq)
        } else {
            inner
        };

        let inset = Sides::default().with(
<<<<<<< HEAD
            TextElem::dir_in(styles).start(),
=======
            styles.resolve(TextElem::dir).start(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Some(base_indent + Rel::from(hanging_indent.unwrap_or_default())),
        );

        Ok(BlockElem::new()
            .with_inset(inset)
            .with_body(Some(BlockBody::Content(body)))
            .pack()
            .spanned(span))
    }

<<<<<<< HEAD
    /// 要素の番号付け（存在する場合）の出力形式。
    ///
    /// また、図や数式の場合は、通常のアウトラインと同様に要素の補足語も追加します。
    /// 例えば、見出しであれば`1.1`と出力されますが、図であれば`Figure 1`と出力されます。
=======
    /// Formats the element's numbering (if any).
    ///
    /// This also appends the element's supplement in case of figures or
    /// equations. For instance, it would output `1.1` for a heading, but
    /// `Figure 1` for a figure, as is usual for outlines.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
    /// 項目の規定の内部コンテンツを生成。
    ///
    /// これには、本文とフィラー、ページ番号が含まれます。
=======
    /// Creates the default inner content of the entry.
    ///
    /// This includes the body, the fill, and page number.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func(contextual)]
    pub fn inner(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<Content> {
<<<<<<< HEAD
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
=======
        let body = self.body().at(span)?;
        let page = self.page(engine, context, span)?;
        self.build_inner(context, span, body, page)
    }

    /// The content which is displayed in place of the referred element at its
    /// entry in the outline. For a heading, this is its
    /// [`body`]($heading.body); for a figure a caption and for equations, it is
    /// empty.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn body(&self) -> StrResult<Content> {
        Ok(self.outlinable()?.body())
    }

<<<<<<< HEAD
    /// 要素の項目のページ番号。
    /// これは参照されるページに対する番号付け設定の書式で出力されます。
=======
    /// The page number of this entry's element, formatted with the numbering
    /// set for the referenced page.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
=======
    pub fn build_inner(
        &self,
        context: Tracked<Context>,
        span: Span,
        body: Content,
        page: Content,
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
        let rtl = styles.resolve(TextElem::dir) == Dir::RTL;
        if rtl {
            // "Right-to-Left Embedding"
            seq.push(TextElem::packed("\u{202B}"));
        }

        seq.push(body);

        if rtl {
            // "Pop Directional Formatting"
            seq.push(TextElem::packed("\u{202C}"));
        }

        // Add the filler between the section name and page number.
        if let Some(filler) = self.fill.get_cloned(styles) {
            seq.push(SpaceElem::shared().clone());
            seq.push(
                BoxElem::new()
                    .with_body(Some(filler))
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
        seq.push(page);

        Ok(Content::sequence(seq))
    }

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    fn outlinable(&self) -> StrResult<&dyn Outlinable> {
        self.element
            .with::<dyn Outlinable>()
            .ok_or_else(|| error!("cannot outline {}", self.element.func().name()))
    }

<<<<<<< HEAD
    fn element_location(&self) -> HintedStrResult<Location> {
        let elem = &self.element;
        elem.location().ok_or_else(|| {
            if elem.can::<dyn Locatable>() && elem.can::<dyn Outlinable>() {
=======
    /// Returns the location of the outlined element.
    pub fn element_location(&self) -> HintedStrResult<Location> {
        let elem = &self.element;
        elem.location().ok_or_else(|| {
            if elem.can::<dyn Outlinable>() {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
    let elems = introspector.query(&select_where!(PrefixInfo, Key => outline_loc));
=======
    let elems = introspector.query(&select_where!(PrefixInfo, key => outline_loc));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
#[elem(Construct, Locatable, Show)]
struct PrefixInfo {
=======
#[elem(Construct, Unqueriable, Locatable)]
pub(crate) struct PrefixInfo {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD

impl Show for Packed<PrefixInfo> {
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(Content::empty())
    }
}
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
