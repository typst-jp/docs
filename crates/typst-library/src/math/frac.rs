use typst_syntax::Spanned;

use crate::diag::bail;
use crate::foundations::{Cast, Content, Value, elem};
use crate::math::Mathy;

/// 分数。
///
/// # 例
/// ```example
/// $ 1/2 < (x+1)/2 $
/// $ ((x+1)) / 2 = frac(a, b) $
/// ```
///
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

    /// 分数のレイアウト方法。
    ///
    /// ```example:"スタイル"
    /// $ frac(x, y, style: "vertical") $
    /// $ frac(x, y, style: "skewed") $
    /// $ frac(x, y, style: "horizontal") $
    /// ```
    ///
    /// ```example:"デフォルトの設定"
    /// #set math.frac(style: "skewed")
    /// $ a / b $
    /// ```
    ///
    /// ```example:"グループ化括弧の扱い"
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
    /// ```example:"インライン数式とブロック数式でのスタイル"
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
/// ```example
/// $ binom(n, k) $
/// $ binom(n, k_1, k_2, k_3, ..., k_m) $
/// ```
#[elem(title = "Binomial", Mathy)]
pub struct BinomElem {
    /// 二項係数の上側の数。
    #[required]
    pub upper: Content,

    /// 二項係数の下側の数。
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
