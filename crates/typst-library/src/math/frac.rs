use typst_syntax::Spanned;

use crate::diag::bail;
use crate::foundations::{Cast, Content, Value, elem};
use crate::math::Mathy;

<<<<<<< HEAD
/// 分数。
///
/// # 例
=======
/// A mathematical fraction.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// $ 1/2 < (x+1)/2 $
/// $ ((x+1)) / 2 = frac(a, b) $
/// ```
///
<<<<<<< HEAD
/// # 構文
/// この関数には専用の構文もあります。
/// 隣接する式をスラッシュで区切ると、分数になります。
/// また、丸括弧で複数の式要素を囲うと、単一の式として扱えます。
/// そのような丸括弧は出力からは削除されますが、複数重ねてネストすることで、丸括弧も表示させられます。
#[elem(title = "Fraction", Mathy)]
pub struct FracElem {
    /// 分数の分子。
    #[required]
    pub num: Content,

    /// 分数の分母。
    #[required]
    pub denom: Content,

    /// 分数のレイアウト方法。
    ///
    /// ```example:"スタイル"
=======
/// # Syntax
/// This function also has dedicated syntax: Use a slash to turn neighbouring
/// expressions into a fraction. Multiple atoms can be grouped into a single
/// expression using round grouping parentheses. Such parentheses are removed
/// from the output, but you can nest multiple to force them.
#[elem(title = "Fraction", Mathy)]
pub struct FracElem {
    /// The fraction's numerator.
    #[required]
    pub num: Content,

    /// The fraction's denominator.
    #[required]
    pub denom: Content,

    /// How the fraction should be laid out.
    ///
    /// ```example:"Styles"
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    /// $ frac(x, y, style: "vertical") $
    /// $ frac(x, y, style: "skewed") $
    /// $ frac(x, y, style: "horizontal") $
    /// ```
    ///
<<<<<<< HEAD
    /// ```example:"デフォルトの設定"
=======
    /// ```example:"Setting the default"
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    /// #set math.frac(style: "skewed")
    /// $ a / b $
    /// ```
    ///
<<<<<<< HEAD
    /// ```example:"グループ化括弧の扱い"
=======
    /// ```example:"Handling of grouping parentheses"
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    /// // Grouping parentheses are removed.
    /// #set math.frac(style: "vertical")
    /// $ (a + b) / b $
    ///
    /// // Grouping parentheses are removed.
    /// #set math.frac(style: "skewed")
    /// $ (a + b) / b $
    ///
    /// // Grouping parentheses are retained.
    /// #set math.frac(style: "horizontal")
    /// $ (a + b) / b $
    /// ```
    ///
<<<<<<< HEAD
    /// ```example:"インライン数式とブロック数式でのスタイル"
=======
    /// ```example:"Different styles in inline vs block equations"
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    /// // This changes the style for inline equations only.
    /// #show math.equation.where(block: false): set math.frac(style: "horizontal")
    ///
    /// This $(x-y)/z = 3$ is inline math, and this is block math:
    /// $ (x-y)/z = 3 $
    /// ```
    #[default(FracStyle::Vertical)]
    pub style: FracStyle,

    /// Whether the numerator was originally surrounded by parentheses
    /// that were stripped by the parser.
    #[internal]
    #[parse(None)]
    #[default(false)]
    pub num_deparenthesized: bool,

    /// Whether the denominator was originally surrounded by parentheses
    /// that were stripped by the parser.
    #[internal]
    #[parse(None)]
    #[default(false)]
    pub denom_deparenthesized: bool,
}

/// Fraction style
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum FracStyle {
<<<<<<< HEAD
    /// 分子と分母を上下に配置し、横棒で区切ります。
    #[default]
    Vertical,
    /// 分子と分母をスラッシュで区切ります。
    Skewed,
    /// 分子と分母をインラインで配置し、丸括弧をそのまま保持します。
    Horizontal,
}

/// 二項係数。
///
/// # 例
=======
    /// Stacked numerator and denominator with a bar.
    #[default]
    Vertical,
    /// Numerator and denominator separated by a slash.
    Skewed,
    /// Numerator and denominator placed inline and parentheses are not
    /// absorbed.
    Horizontal,
}

/// A binomial expression.
///
/// # Example
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
/// ```example
/// $ binom(n, k) $
/// $ binom(n, k_1, k_2, k_3, ..., k_m) $
/// ```
#[elem(title = "Binomial", Mathy)]
pub struct BinomElem {
<<<<<<< HEAD
    /// 二項係数の上側の数。
    #[required]
    pub upper: Content,

    /// 二項係数の下側の数。
=======
    /// The binomial's upper index.
    #[required]
    pub upper: Content,

    /// The binomial's lower index.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
    #[required]
    #[variadic]
    #[parse(
        let values = args.all::<Spanned<Value>>()?;
        if values.is_empty() {
            // Prevents one element binomials
            bail!(args.span, "missing argument: lower");
        }
        values.into_iter().map(|spanned| spanned.v.display()).collect()
    )]
    pub lower: Vec<Content>,
}
