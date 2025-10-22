use std::num::NonZeroUsize;

use ecow::eco_format;
use typst_utils::{Get, NonZeroExt};

use crate::diag::{warning, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    elem, Content, NativeElement, Packed, Resolve, Show, ShowSet, Smart, StyleChain,
    Styles, Synthesize, TargetElem,
};
use crate::html::{attr, tag, HtmlElem};
use crate::introspection::{
    Count, Counter, CounterUpdate, Locatable, Locator, LocatorLink,
};
use crate::layout::{Abs, Axes, BlockBody, BlockElem, Em, HElem, Length, Region, Sides};
use crate::model::{Numbering, Outlinable, Refable, Supplement};
use crate::text::{FontWeight, LocalName, SpaceElem, TextElem, TextSize};

/// セクションの見出し。
///
/// 見出しを使うことで、文書をセクションとして構造化できます。
/// 各見出しには1から始まる _レベル_ があり、上限はありません。
/// このレベルは、以下に続くコンテンツの論理的な役割（セクション、サブセクションなど）を示します。
/// 最上位のレベルの見出しは、文書の最上位のレベルのセクションを示します（文書のタイトルではありません）。
///
/// Typstでは、見出しに自動的に番号をつけることができます。
/// 番号付けを有効にするには、
/// 見出しにどのような[番号付けパターンまたは関数]($numbering)を用いて番号付けを行うかを指定してください。
///
/// 番号付けとは別に、Typstは全ての見出しの[目次]($outline)を自動的に生成することもできます。
/// 1つ以上の見出しをこの目次から除外するには、
/// `outlined`パラメーターを`{false}`に設定してください。
///
/// # 例
/// ```example
/// #set heading(numbering: "1.a)")
///
/// = Introduction
/// In recent years, ...
///
/// == Preliminaries
/// To start, ...
/// ```
///
/// # 構文
/// 見出しには専用の構文があります。
/// 行の先頭に等号を1つ以上入力し、その後にスペースを入力することで見出しを作成できます。
/// 等号の数は、見出しの論理的なネストの深さを決定します。
/// `{offset}`フィールドを設定すると、見出しの最初の深さを設定できます。
#[elem(Locatable, Synthesize, Count, Show, ShowSet, LocalName, Refable, Outlinable)]
pub struct HeadingElem {
    /// 1から始まる、見出しの絶対的なネストの深さ。
    /// `{auto}`に設定した場合は、`{offset + depth}`から計算されます。
    ///
    /// これは主に[showルール]($styling/#show-rules)で利用する際に役立ちます
    /// （[`where`]($function.where)セレクターを使う場合や
    /// 表示された見出しのレベルに直接アクセスする場合など）。
    ///
    /// ```example
    /// #show heading.where(level: 2): set text(red)
    ///
    /// = Level 1
    /// == Level 2
    ///
    /// #set heading(offset: 1)
    /// = Also level 2
    /// == Level 3
    /// ```
    pub level: Smart<NonZeroUsize>,

    /// 1から始まる、見出しの相対的なネストの深さ。
    /// この値は`{offset}`と組み合わせて、実際の`{level}`を計算するのに用いられます。
    ///
    /// これは見出し構文によって設定され、例えば`[== Heading]`は論理的な深さが2の見出しを作成しますが、
    /// 実際のレベルは`{offset + 2}`になります。
    /// 見出しを手動で作成する場合、
    /// 通常は絶対レベルを設定するよりもこちらを使用することをおすすめします。
    #[default(NonZeroUsize::ONE)]
    pub depth: NonZeroUsize,

    /// 各見出しの`{level}`の開始オフセットであり、
    /// 相対的な`{depth}`を絶対的な`{level}`に変換するために使用されます。
    ///
    /// ```example
    /// = Level 1
    ///
    /// #set heading(offset: 1, numbering: "1.1")
    /// = Level 2
    ///
    /// #heading(offset: 2, depth: 2)[
    ///   I'm level 4
    /// ]
    /// ```
    #[default(0)]
    pub offset: usize,

    /// 見出しを番号付けする方法。
    /// [番号付けパターンまたは関数]($numbering)を指定できます。
    ///
    /// ```example
    /// #set heading(numbering: "1.a.")
    ///
    /// = A section
    /// == A subsection
    /// === A sub-subsection
    /// ```
    #[borrowed]
    pub numbering: Option<Numbering>,

    /// 見出しに用いる補足語。
    ///
    /// 見出しを参照する際、補足語が参照番号の前に追加されます。
    ///
    /// 関数を指定した場合、参照された見出しが引数として渡され、
    /// その関数は表示されるコンテンツを返す必要があります。
    ///
    /// ```example
    /// #set heading(numbering: "1.", supplement: [Chapter])
    ///
    /// = Introduction <intro>
    /// In @intro, we see how to turn
    /// Sections into Chapters. And
    /// in @intro[Part], it is done
    /// manually.
    /// ```
    pub supplement: Smart<Option<Supplement>>,

    /// 見出しを[目次]($outline)に表示するかどうか。
    ///
    /// なお、このプロパティを`{true}`に設定すると、
    /// PDFへのエクスポート時に、見出しがPDFの目次にしおりとしても表示されます。
    /// この動作を変更するには、`bookmarked`プロパティを使用してください。
    ///
    /// ```example
    /// #outline()
    ///
    /// #heading[Normal]
    /// This is a normal heading.
    ///
    /// #heading(outlined: false)[Hidden]
    /// This heading does not appear
    /// in the outline.
    /// ```
    #[default(true)]
    pub outlined: bool,

    /// エクスポートされたPDFの目次に見出しをしおりとして表示するかどうか。
    /// PNGなどの他のエクスポート形式には影響しません。
    ///
    /// デフォルト値の`{auto}`は、`outlined`プロパティが`{true}`に設定されている見出し、
    /// すなわちTypstの[目次]($outline)にも記載される見出しのみが、
    /// PDFエクスポート時の目次に表示されることを示します。
    /// このプロパティを`{true}`（しおりあり）または`{false}`（しおりなし）に設定すると、
    /// この動作を無視します。
    ///
    /// ```example
    /// #heading[Normal heading]
    /// This heading will be shown in
    /// the PDF's bookmark outline.
    ///
    /// #heading(bookmarked: false)[Not bookmarked]
    /// This heading won't be
    /// bookmarked in the resulting
    /// PDF.
    /// ```
    #[default(Smart::Auto)]
    pub bookmarked: Smart<bool>,

    /// 見出しの最初の行を除く全ての行に適用されるインデント。
    ///
    /// デフォルト値の`{auto}`では、
    /// 見出しの先頭行に続く行が番号の幅にあわせてインデントされます。
    ///
    /// ```example
    /// #set heading(numbering: "1.")
    /// #heading[A very, very, very, very, very, very long heading]
    /// ```
    #[default(Smart::Auto)]
    pub hanging_indent: Smart<Length>,

    /// 見出しのタイトル。
    #[required]
    pub body: Content,
}

impl HeadingElem {
    pub fn resolve_level(&self, styles: StyleChain) -> NonZeroUsize {
        self.level(styles).unwrap_or_else(|| {
            NonZeroUsize::new(self.offset(styles) + self.depth(styles).get())
                .expect("overflow to 0 on NoneZeroUsize + usize")
        })
    }
}

impl Synthesize for Packed<HeadingElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let supplement = match (**self).supplement(styles) {
            Smart::Auto => TextElem::packed(Self::local_name_in(styles)),
            Smart::Custom(None) => Content::empty(),
            Smart::Custom(Some(supplement)) => {
                supplement.resolve(engine, styles, [self.clone().pack()])?
            }
        };

        let elem = self.as_mut();
        elem.push_level(Smart::Custom(elem.resolve_level(styles)));
        elem.push_supplement(Smart::Custom(Some(Supplement::Content(supplement))));
        Ok(())
    }
}

impl Show for Packed<HeadingElem> {
    #[typst_macros::time(name = "heading", span = self.span())]
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let html = TargetElem::target_in(styles).is_html();

        const SPACING_TO_NUMBERING: Em = Em::new(0.3);

        let span = self.span();
        let mut realized = self.body.clone();

        let hanging_indent = self.hanging_indent(styles);
        let mut indent = match hanging_indent {
            Smart::Custom(length) => length.resolve(styles),
            Smart::Auto => Abs::zero(),
        };

        if let Some(numbering) = (**self).numbering(styles).as_ref() {
            let location = self.location().unwrap();
            let numbering = Counter::of(HeadingElem::elem())
                .display_at_loc(engine, location, styles, numbering)?
                .spanned(span);

            if hanging_indent.is_auto() && !html {
                let pod = Region::new(Axes::splat(Abs::inf()), Axes::splat(false));

                // We don't have a locator for the numbering here, so we just
                // use the measurement infrastructure for now.
                let link = LocatorLink::measure(location);
                let size = (engine.routines.layout_frame)(
                    engine,
                    &numbering,
                    Locator::link(&link),
                    styles,
                    pod,
                )?
                .size();

                indent = size.x + SPACING_TO_NUMBERING.resolve(styles);
            }

            let spacing = if html {
                SpaceElem::shared().clone()
            } else {
                HElem::new(SPACING_TO_NUMBERING.into()).with_weak(true).pack()
            };

            realized = numbering + spacing + realized;
        }

        Ok(if html {
            // HTML's h1 is closer to a title element. There should only be one.
            // Meanwhile, a level 1 Typst heading is a section heading. For this
            // reason, levels are offset by one: A Typst level 1 heading becomes
            // a `<h2>`.
            let level = self.resolve_level(styles).get();
            if level >= 6 {
                engine.sink.warn(warning!(span,
                    "heading of level {} was transformed to \
                    <div role=\"heading\" aria-level=\"{}\">, which is not \
                    supported by all assistive technology",
                    level, level + 1;
                    hint: "HTML only supports <h1> to <h6>, not <h{}>", level + 1;
                    hint: "you may want to restructure your document so that \
                          it doesn't contain deep headings"));
                HtmlElem::new(tag::div)
                    .with_body(Some(realized))
                    .with_attr(attr::role, "heading")
                    .with_attr(attr::aria_level, eco_format!("{}", level + 1))
                    .pack()
                    .spanned(span)
            } else {
                let t = [tag::h2, tag::h3, tag::h4, tag::h5, tag::h6][level - 1];
                HtmlElem::new(t).with_body(Some(realized)).pack().spanned(span)
            }
        } else {
            let block = if indent != Abs::zero() {
                let body = HElem::new((-indent).into()).pack() + realized;
                let inset = Sides::default()
                    .with(TextElem::dir_in(styles).start(), Some(indent.into()));
                BlockElem::new()
                    .with_body(Some(BlockBody::Content(body)))
                    .with_inset(inset)
            } else {
                BlockElem::new().with_body(Some(BlockBody::Content(realized)))
            };
            block.pack().spanned(span)
        })
    }
}

impl ShowSet for Packed<HeadingElem> {
    fn show_set(&self, styles: StyleChain) -> Styles {
        let level = (**self).resolve_level(styles).get();
        let scale = match level {
            1 => 1.4,
            2 => 1.2,
            _ => 1.0,
        };

        let size = Em::new(scale);
        let above = Em::new(if level == 1 { 1.8 } else { 1.44 }) / scale;
        let below = Em::new(0.75) / scale;

        let mut out = Styles::new();
        out.set(TextElem::set_size(TextSize(size.into())));
        out.set(TextElem::set_weight(FontWeight::BOLD));
        out.set(BlockElem::set_above(Smart::Custom(above.into())));
        out.set(BlockElem::set_below(Smart::Custom(below.into())));
        out.set(BlockElem::set_sticky(true));
        out
    }
}

impl Count for Packed<HeadingElem> {
    fn update(&self) -> Option<CounterUpdate> {
        (**self)
            .numbering(StyleChain::default())
            .is_some()
            .then(|| CounterUpdate::Step((**self).resolve_level(StyleChain::default())))
    }
}

impl Refable for Packed<HeadingElem> {
    fn supplement(&self) -> Content {
        // After synthesis, this should always be custom content.
        match (**self).supplement(StyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
            _ => Content::empty(),
        }
    }

    fn counter(&self) -> Counter {
        Counter::of(HeadingElem::elem())
    }

    fn numbering(&self) -> Option<&Numbering> {
        (**self).numbering(StyleChain::default()).as_ref()
    }
}

impl Outlinable for Packed<HeadingElem> {
    fn outlined(&self) -> bool {
        (**self).outlined(StyleChain::default())
    }

    fn level(&self) -> NonZeroUsize {
        (**self).resolve_level(StyleChain::default())
    }

    fn prefix(&self, numbers: Content) -> Content {
        numbers
    }

    fn body(&self) -> Content {
        self.body.clone()
    }
}

impl LocalName for Packed<HeadingElem> {
    const KEY: &'static str = "heading";
}
