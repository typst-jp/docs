use std::num::NonZeroUsize;

use typst_utils::NonZeroExt;
use unicode_math_class::MathClass;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
    elem, Content, NativeElement, Packed, Show, ShowSet, Smart, StyleChain, Styles,
    Synthesize,
};
use crate::introspection::{Count, Counter, CounterUpdate, Locatable};
use crate::layout::{
    AlignElem, Alignment, BlockElem, InlineElem, OuterHAlignment, SpecificAlignment,
    VAlignment,
};
use crate::math::{MathSize, MathVariant};
use crate::model::{Numbering, Outlinable, ParLine, Refable, Supplement};
use crate::text::{FontFamily, FontList, FontWeight, LocalName, TextElem};

/// 数式。
///
/// 数式は、テキスト内にインラインで表示することも、独立したブロックとして表示することもできます。
/// 数式をブロックレベルにするには、
/// 開始ドル記号の直後と終了ドル記号の直前に少なくとも1つのスペースを挿入してください。
///
/// # 例
/// ```example
/// #set text(font: "New Computer Modern")
///
/// Let $a$, $b$, and $c$ be the side
/// lengths of right-angled triangle.
/// Then, we know that:
/// $ a^2 + b^2 = c^2 $
///
/// Prove by induction:
/// $ sum_(k=1)^n k = (n(n+1)) / 2 $
/// ```
///
/// デフォルトでは、ブロックレベルの数式はページをまたいで分割されません。
/// これは`{show math.equation: set block(breakable: true)}`を使用して変更できます。
///
/// # 構文
/// この関数には専用の構文もあります。
/// 数式のマークアップをドル記号で囲むことで、数式を作成します。
/// 式の先頭と末尾にそれぞれ少なくとも1つのスペースを挿入すると、
/// 数式は独立したブロックとして扱われ、水平中央に配置されます。
/// 数式の構文の詳細については、[数式のメインページ]($category/math)を参照してください。
#[elem(Locatable, Synthesize, Show, ShowSet, Count, LocalName, Refable, Outlinable)]
pub struct EquationElem {
    /// 数式が独立したブロックとして表示されるかどうか。
    #[default(false)]
    pub block: bool,

    /// ブロックレベル数式への[番号付け]($numbering)方法。
    ///
    /// ```example
    /// #set math.equation(numbering: "(1)")
    ///
    /// We define:
    /// $ phi.alt := (1 + sqrt(5)) / 2 $ <ratio>
    ///
    /// With @ratio, we get:
    /// $ F_n = floor(1 / sqrt(5) phi.alt^n) $
    /// ```
    #[borrowed]
    pub numbering: Option<Numbering>,

    /// 数式番号の配置。
    ///
    /// デフォルトでは、数式の配置は`{end + horizon}`です。
    /// 水平方向の成分には`{right}`、`{left}`、
    /// またはテキスト方向の`{start}`と`{end}`を使用できます。
    /// 垂直方向の成分には、`{top}`、`{horizon}`、または`{bottom}`を使用できます。
    ///
    /// ```example
    /// #set math.equation(numbering: "(1)", number-align: bottom)
    ///
    /// We can calculate:
    /// $ E &= sqrt(m_0^2 + p^2) \
    ///     &approx 125 "GeV" $
    /// ```
    #[default(SpecificAlignment::Both(OuterHAlignment::End, VAlignment::Horizon))]
    pub number_align: SpecificAlignment<OuterHAlignment, VAlignment>,

    /// 数式に用いる補足語。
    ///
    /// 見出しを参照する際、補足語が参照番号の前に追加されます。
    ///
    /// 関数を指定した場合、参照された数式が引数として渡され、
    /// その関数は表示されるコンテンツを返す必要があります。
    ///
    /// ```example
    /// #set math.equation(numbering: "(1)", supplement: [Eq.])
    ///
    /// We define:
    /// $ phi.alt := (1 + sqrt(5)) / 2 $ <ratio>
    ///
    /// With @ratio, we get:
    /// $ F_n = floor(1 / sqrt(5) phi.alt^n) $
    /// ```
    pub supplement: Smart<Option<Supplement>>,

    /// 数式のコンテンツ。
    #[required]
    pub body: Content,

    /// The size of the glyphs.
    #[internal]
    #[default(MathSize::Text)]
    #[ghost]
    pub size: MathSize,

    /// The style variant to select.
    #[internal]
    #[ghost]
    pub variant: MathVariant,

    /// Affects the height of exponents.
    #[internal]
    #[default(false)]
    #[ghost]
    pub cramped: bool,

    /// Whether to use bold glyphs.
    #[internal]
    #[default(false)]
    #[ghost]
    pub bold: bool,

    /// Whether to use italic glyphs.
    #[internal]
    #[ghost]
    pub italic: Smart<bool>,

    /// A forced class to use for all fragment.
    #[internal]
    #[ghost]
    pub class: Option<MathClass>,

    /// Values of `scriptPercentScaleDown` and `scriptScriptPercentScaleDown`
    /// respectively in the current font's MathConstants table.
    #[internal]
    #[default((70, 50))]
    #[ghost]
    pub script_scale: (i16, i16),
}

impl Synthesize for Packed<EquationElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let supplement = match self.as_ref().supplement(styles) {
            Smart::Auto => TextElem::packed(Self::local_name_in(styles)),
            Smart::Custom(None) => Content::empty(),
            Smart::Custom(Some(supplement)) => {
                supplement.resolve(engine, styles, [self.clone().pack()])?
            }
        };

        self.push_supplement(Smart::Custom(Some(Supplement::Content(supplement))));
        Ok(())
    }
}

impl Show for Packed<EquationElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        if self.block(styles) {
            Ok(BlockElem::multi_layouter(
                self.clone(),
                engine.routines.layout_equation_block,
            )
            .pack()
            .spanned(self.span()))
        } else {
            Ok(InlineElem::layouter(self.clone(), engine.routines.layout_equation_inline)
                .pack()
                .spanned(self.span()))
        }
    }
}

impl ShowSet for Packed<EquationElem> {
    fn show_set(&self, styles: StyleChain) -> Styles {
        let mut out = Styles::new();
        if self.block(styles) {
            out.set(AlignElem::set_alignment(Alignment::CENTER));
            out.set(BlockElem::set_breakable(false));
            out.set(ParLine::set_numbering(None));
            out.set(EquationElem::set_size(MathSize::Display));
        } else {
            out.set(EquationElem::set_size(MathSize::Text));
        }
        out.set(TextElem::set_weight(FontWeight::from_number(450)));
        out.set(TextElem::set_font(FontList(vec![FontFamily::new(
            "New Computer Modern Math",
        )])));
        out
    }
}

impl Count for Packed<EquationElem> {
    fn update(&self) -> Option<CounterUpdate> {
        (self.block(StyleChain::default()) && self.numbering().is_some())
            .then(|| CounterUpdate::Step(NonZeroUsize::ONE))
    }
}

impl LocalName for Packed<EquationElem> {
    const KEY: &'static str = "equation";
}

impl Refable for Packed<EquationElem> {
    fn supplement(&self) -> Content {
        // After synthesis, this should always be custom content.
        match (**self).supplement(StyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
            _ => Content::empty(),
        }
    }

    fn counter(&self) -> Counter {
        Counter::of(EquationElem::elem())
    }

    fn numbering(&self) -> Option<&Numbering> {
        (**self).numbering(StyleChain::default()).as_ref()
    }
}

impl Outlinable for Packed<EquationElem> {
    fn outlined(&self) -> bool {
        self.block(StyleChain::default()) && self.numbering().is_some()
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
        Content::empty()
    }
}
