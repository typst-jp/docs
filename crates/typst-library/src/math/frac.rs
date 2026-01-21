use typst_syntax::Spanned;

use crate::diag::bail;
<<<<<<< HEAD
use crate::foundations::{elem, Content, Value};
use crate::math::Mathy;

/// 分数。
///
/// # 例
=======
use crate::foundations::{Cast, Content, Value, elem};
use crate::math::Mathy;

/// A mathematical fraction.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
/// そのような丸括弧は出力からは削除されますが、複数重ねてネストすることで、丸括弧を表示させることも可能です。
#[elem(title = "Fraction", Mathy)]
pub struct FracElem {
    /// 分数の分子。
    #[required]
    pub num: Content,

    /// 分数の分母。
    #[required]
    pub denom: Content,
}

/// 二項係数。
///
/// # 例
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
    /// $ frac(x, y, style: "vertical") $
    /// $ frac(x, y, style: "skewed") $
    /// $ frac(x, y, style: "horizontal") $
    /// ```
    ///
    /// ```example:"Setting the default"
    /// #set math.frac(style: "skewed")
    /// $ a / b $
    /// ```
    ///
    /// ```example:"Handling of grouping parentheses"
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
    /// ```example:"Different styles in inline vs block equations"
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
