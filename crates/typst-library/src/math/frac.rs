use typst_syntax::Spanned;

use crate::diag::bail;
use crate::foundations::{elem, Content, Value};
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
