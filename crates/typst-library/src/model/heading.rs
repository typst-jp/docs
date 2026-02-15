use std::num::NonZeroUsize;

use ecow::EcoString;
use typst_utils::NonZeroExt;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
    Content, NativeElement, Packed, ShowSet, Smart, StyleChain, Styles, Synthesize, elem,
};
use crate::introspection::{Count, Counter, CounterUpdate, Locatable, Tagged};
use crate::layout::{BlockElem, Em, Length};
use crate::model::{Numbering, Outlinable, Refable, Supplement};
use crate::text::{FontWeight, LocalName, TextElem, TextSize};

/// セクションの見出し。
///
/// 見出しを使うことで、文書をセクションとして構造化できます。
/// 各見出しには1から始まる_レベル_があり、上限はありません。
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
/// [`body`フィールド]($heading.body)にアクセスして見出しの見た目を
/// 完全にカスタマイズする[showルール]($styling/#show-rules)を書く場合は、
/// 内容を[`block`]($block)で包むようにしてください（見出しにはビルトインの
/// show-setルールにより[`sticky`]($block.sticky)が暗黙的に適用されます）。
/// これにより、見出しがページ末に残り本文が次ページに送られる「孤立」状態を防げます。
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
///
/// # アクセシビリティ
/// 見出しはアクセシビリティにとって重要です。スクリーンリーダーなどの
/// 支援技術（AT）利用者が文書内を移動しやすくなり、見出し間の移動や
/// 全体の見出し一覧の把握が可能になります。
///
/// 見出しレベルを飛ばさないことが重要です。最初は第1レベルから始め、
/// 例えば直前がレベル3なら、次はレベル3（同じ深さ）、レベル4（1つ深く）、
/// あるいはレベル1または2（上位の見出し）にします。
///
/// # HTMLエクスポート
/// 前述のとおり、最上位の見出しは文書タイトルではなく最上位セクションを示します。
/// これは文書内で1つだけにすべきHTMLの`<h1>`とは異なります。
///
/// そのためHTMLエクスポートでは、[`title`]要素が`<h1>`になり、見出しは
/// `<h2>`以下になります（レベル1は`<h2>`、レベル2は`<h3>`という具合）。
#[elem(Locatable, Tagged, Synthesize, Count, ShowSet, LocalName, Refable, Outlinable)]
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
    /// [番号付けパターンまたは関数]($numbering)を指定できます（複数の数値を受け取ります）。
    ///
    /// ```example
    /// #set heading(numbering: "1.a.")
    ///
    /// = A section
    /// == A subsection
    /// === A sub-subsection
    /// ```
    pub numbering: Option<Numbering>,

    /// 解決済みのプレーンテキストの番号。
    ///
    /// このフィールドは内部用で、PDFのしおり作成にのみ使われます。
    /// エクスポート時は`World`、`Engine`、`styles`にアクセスできないため、
    /// カウンターや番号付けパターンを具体的な文字列に解決するために必要です。
    ///
    /// `numbering`が`None`の場合は設定されません。
    #[internal]
    #[synthesized]
    pub numbers: EcoString,

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
    /// デフォルト値の`{auto}`では、見出しが[テキスト方向]($text.dir)の
    /// [start]($direction.start)に揃えられている場合、番号の幅をインデントとして
    /// 使用します。中央揃えやそれ以外の配置ではインデントしません。
    ///
    /// ```example
    /// #set heading(numbering: "1.")
    /// = A very, very, very, very, very, very long heading
    ///
    /// #show heading: set align(center)
    /// == A very long heading\ with center alignment
    /// ```
    #[default(Smart::Auto)]
    pub hanging_indent: Smart<Length>,

    /// 見出しのタイトル。
    #[required]
    pub body: Content,
}

impl HeadingElem {
    pub fn resolve_level(&self, styles: StyleChain) -> NonZeroUsize {
        self.level.get(styles).unwrap_or_else(|| {
            NonZeroUsize::new(self.offset.get(styles) + self.depth.get(styles).get())
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
        let supplement = match self.supplement.get_ref(styles) {
            Smart::Auto => TextElem::packed(Self::local_name_in(styles)),
            Smart::Custom(None) => Content::empty(),
            Smart::Custom(Some(supplement)) => {
                supplement.resolve(engine, styles, [self.clone().pack()])?
            }
        };

        if let Some((numbering, location)) =
            self.numbering.get_ref(styles).as_ref().zip(self.location())
        {
            self.numbers = Some(
                self.counter()
                    .display_at_loc(engine, location, styles, numbering)?
                    .plain_text(),
            );
        }

        let elem = self.as_mut();
        elem.level.set(Smart::Custom(elem.resolve_level(styles)));
        elem.supplement
            .set(Smart::Custom(Some(Supplement::Content(supplement))));
        Ok(())
    }
}

impl ShowSet for Packed<HeadingElem> {
    fn show_set(&self, styles: StyleChain) -> Styles {
        let level = self.resolve_level(styles).get();
        let scale = match level {
            1 => 1.4,
            2 => 1.2,
            _ => 1.0,
        };

        let size = Em::new(scale);
        let above = Em::new(if level == 1 { 1.8 } else { 1.44 }) / scale;
        let below = Em::new(0.75) / scale;

        let mut out = Styles::new();
        out.set(TextElem::size, TextSize(size.into()));
        out.set(TextElem::weight, FontWeight::BOLD);
        out.set(BlockElem::above, Smart::Custom(above.into()));
        out.set(BlockElem::below, Smart::Custom(below.into()));
        out.set(BlockElem::sticky, true);
        out
    }
}

impl Count for Packed<HeadingElem> {
    fn update(&self) -> Option<CounterUpdate> {
        self.numbering
            .get_ref(StyleChain::default())
            .is_some()
            .then(|| CounterUpdate::Step(self.resolve_level(StyleChain::default())))
    }
}

impl Refable for Packed<HeadingElem> {
    fn supplement(&self) -> Content {
        // After synthesis, this should always be custom content.
        match self.supplement.get_cloned(StyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
            _ => Content::empty(),
        }
    }

    fn counter(&self) -> Counter {
        Counter::of(HeadingElem::ELEM)
    }

    fn numbering(&self) -> Option<&Numbering> {
        self.numbering.get_ref(StyleChain::default()).as_ref()
    }
}

impl Outlinable for Packed<HeadingElem> {
    fn outlined(&self) -> bool {
        self.outlined.get(StyleChain::default())
    }

    fn level(&self) -> NonZeroUsize {
        self.resolve_level(StyleChain::default())
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
