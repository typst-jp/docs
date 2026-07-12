//! Calculations and processing of numeric values.

use std::cmp;
use std::cmp::Ordering;

use az::SaturatingAs;
use typst_syntax::{Span, Spanned};
use typst_utils::{round_int_with_precision, round_with_precision};

use crate::diag::{At, HintedString, SourceResult, StrResult, bail};
use crate::foundations::{Decimal, IntoValue, Module, Scope, Value, cast, func, ops};
use crate::layout::{Angle, Fr, Length, Ratio};

/// A module with calculation definitions.
pub fn module() -> Module {
    let mut scope = Scope::new();
    scope.define_func::<abs>();
    scope.define_func::<pow>();
    scope.define_func::<exp>();
    scope.define_func::<sqrt>();
    scope.define_func::<root>();
    scope.define_func::<sin>();
    scope.define_func::<cos>();
    scope.define_func::<tan>();
    scope.define_func::<asin>();
    scope.define_func::<acos>();
    scope.define_func::<atan>();
    scope.define_func::<atan2>();
    scope.define_func::<sinh>();
    scope.define_func::<cosh>();
    scope.define_func::<tanh>();
    scope.define_func::<log>();
    scope.define_func::<ln>();
    scope.define_func::<fact>();
    scope.define_func::<perm>();
    scope.define_func::<binom>();
    scope.define_func::<gcd>();
    scope.define_func::<lcm>();
    scope.define_func::<floor>();
    scope.define_func::<ceil>();
    scope.define_func::<trunc>();
    scope.define_func::<fract>();
    scope.define_func::<round>();
    scope.define_func::<clamp>();
    scope.define_func::<min>();
    scope.define_func::<max>();
    scope.define_func::<even>();
    scope.define_func::<odd>();
    scope.define_func::<rem>();
    scope.define_func::<div_euclid>();
    scope.define_func::<rem_euclid>();
    scope.define_func::<quo>();
    scope.define_func::<norm>();
    scope.define("inf", f64::INFINITY);
    scope.define("pi", std::f64::consts::PI);
    scope.define("tau", std::f64::consts::TAU);
    scope.define("e", std::f64::consts::E);
    Module::new("calc", scope)
}

/// 絶対値。
///
/// ```example
/// #calc.abs(-5) \
/// #calc.abs(5pt - 2cm) \
/// #calc.abs(2fr) \
/// #calc.abs(decimal("-342.440"))
/// ```
#[func(title = "Absolute")]
pub fn abs(
    /// 絶対値を計算する値。
    value: ToAbs,
) -> Value {
    value.0
}

/// A value of which the absolute value can be taken.
pub struct ToAbs(Value);

cast! {
    ToAbs,
    v: i64 => Self(v.abs().into_value()),
    v: f64 => Self(v.abs().into_value()),
    v: Length => Self(Value::Length(v.try_abs()
        .ok_or("cannot take absolute value of this length")?)),
    v: Angle => Self(Value::Angle(v.abs())),
    v: Ratio => Self(Value::Ratio(v.abs())),
    v: Fr => Self(Value::Fraction(v.abs())),
    v: Decimal => Self(Value::Decimal(v.abs()))
}

/// 冪乗。
///
/// ```example
/// #calc.pow(2, 3) \
/// #calc.pow(decimal("2.5"), 2)
/// ```
#[func(title = "Power")]
pub fn pow(
    span: Span,
    /// 冪乗の底（てい）。
    ///
    /// 値が[`decimal`]の場合、指数は[整数]($int)でなければなりません。
    base: DecNum,
    /// 冪乗の指数。
    exponent: Spanned<Num>,
) -> SourceResult<DecNum> {
    match exponent.v {
        _ if exponent.v.float() == 0.0 && base.is_zero() => {
            bail!(span, "zero to the power of zero is undefined")
        }
        Num::Int(i) if i32::try_from(i).is_err() => {
            bail!(exponent.span, "exponent is too large")
        }
        Num::Float(f) if !f.is_normal() && f != 0.0 => {
            bail!(exponent.span, "exponent may not be infinite, subnormal, or NaN")
        }
        _ => {}
    };

    match (base, exponent.v) {
        (DecNum::Int(a), Num::Int(b)) if b >= 0 => a
            .checked_pow(b as u32)
            .map(DecNum::Int)
            .ok_or_else(too_large)
            .at(span),
        (DecNum::Decimal(a), Num::Int(b)) => {
            a.checked_powi(b).map(DecNum::Decimal).ok_or_else(too_large).at(span)
        }
        (a, b) => {
            let Some(a) = a.float() else {
                return Err(cant_apply_to_decimal_and_float()).at(span);
            };

            let result = if a == std::f64::consts::E {
                b.float().exp()
            } else if a == 2.0 {
                b.float().exp2()
            } else if let Num::Int(b) = b {
                a.powi(b as i32)
            } else {
                a.powf(b.float())
            };

            if result.is_nan() {
                bail!(span, "the result is not a real number")
            }

            Ok(DecNum::Float(result))
        }
    }
}

/// eの冪乗。
///
/// ```example
/// #calc.exp(1)
/// ```
#[func(title = "Exponential")]
pub fn exp(
    span: Span,
    /// 冪乗の指数。
    exponent: Spanned<Num>,
) -> SourceResult<f64> {
    match exponent.v {
        Num::Int(i) if i32::try_from(i).is_err() => {
            bail!(exponent.span, "exponent is too large")
        }
        Num::Float(f) if !f.is_normal() && f != 0.0 => {
            bail!(exponent.span, "exponent may not be infinite, subnormal, or NaN")
        }
        _ => {}
    }

    let result = exponent.v.float().exp();
    if result.is_nan() {
        bail!(span, "the result is not a real number")
    }

    Ok(result)
}

/// 平方根。
///
/// ```example
/// #calc.sqrt(16) \
/// #calc.sqrt(2.5)
/// ```
#[func(title = "Square Root")]
pub fn sqrt(
    /// 平方根を計算する数値。負の値は取れません。
    value: Spanned<Num>,
) -> SourceResult<f64> {
    if value.v.float() < 0.0 {
        bail!(value.span, "cannot take square root of negative number");
    }
    Ok(value.v.float().sqrt())
}

/// n乗根。
///
/// 負の値の場合、nは奇数でなければなりません。
///
/// ```example
/// #calc.root(16.0, 4) \
/// #calc.root(27.0, 3)
/// ```
#[func]
pub fn root(
    /// 根を取る対象の式。
    radicand: f64,
    /// 被開方数の何乗根を取るか。
    index: Spanned<i64>,
) -> SourceResult<f64> {
    if index.v == 0 {
        bail!(index.span, "cannot take the 0th root of a number");
    } else if radicand < 0.0 {
        if index.v % 2 == 0 {
            bail!(
                index.span,
                "negative numbers do not have a real nth root when n is even"
            );
        } else {
            Ok(-(-radicand).powf(1.0 / index.v as f64))
        }
    } else {
        Ok(radicand.powf(1.0 / index.v as f64))
    }
}

/// サイン（正弦）の計算。
///
/// 整数または浮動小数点数で呼び出された場合、それらはラジアンとして解釈されます。
///
/// ```example
/// #calc.sin(1.5) \
/// #calc.sin(90deg)
/// ```
#[func(title = "Sine")]
pub fn sin(
    /// サインを計算する角度。
    angle: AngleLike,
) -> f64 {
    match angle {
        AngleLike::Angle(a) => a.sin(),
        AngleLike::Int(n) => (n as f64).sin(),
        AngleLike::Float(n) => n.sin(),
    }
}

/// コサイン（余弦）の計算。
///
/// 整数または浮動小数点数で呼び出された場合、それらはラジアンとして解釈されます。
///
/// ```example
/// #calc.cos(1.5) \
/// #calc.cos(90deg)
/// ```
#[func(title = "Cosine")]
pub fn cos(
    /// コサインを計算する角度。
    angle: AngleLike,
) -> f64 {
    match angle {
        AngleLike::Angle(a) => a.cos(),
        AngleLike::Int(n) => (n as f64).cos(),
        AngleLike::Float(n) => n.cos(),
    }
}

/// タンジェント（正接）の計算。
///
/// 整数または浮動小数点数に対して呼び出された場合、それらはラジアンとして解釈されます。
///
/// ```example
/// #calc.tan(1.5) \
/// #calc.tan(90deg)
/// ```
#[func(title = "Tangent")]
pub fn tan(
    /// タンジェントを計算する角度。
    angle: AngleLike,
) -> f64 {
    match angle {
        AngleLike::Angle(a) => a.tan(),
        AngleLike::Int(n) => (n as f64).tan(),
        AngleLike::Float(n) => n.tan(),
    }
}

/// アークサイン（逆正弦）の計算。
///
/// ```example
/// #calc.asin(0) \
/// #calc.asin(1)
/// ```
#[func(title = "Arcsine")]
pub fn asin(
    /// アークサインを計算する値。値は-1から1の間でなければなりません。
    value: Spanned<Num>,
) -> SourceResult<Angle> {
    let val = value.v.float();
    if val < -1.0 || val > 1.0 {
        bail!(value.span, "value must be between -1 and 1");
    }
    Ok(Angle::rad(val.asin()))
}

/// アークコサイン（逆余弦）の計算。
///
/// ```example
/// #calc.acos(0) \
/// #calc.acos(1)
/// ```
#[func(title = "Arccosine")]
pub fn acos(
    /// アークコサインを計算する値。値は-1から1の間でなければなりません。
    value: Spanned<Num>,
) -> SourceResult<Angle> {
    let val = value.v.float();
    if val < -1.0 || val > 1.0 {
        bail!(value.span, "value must be between -1 and 1");
    }
    Ok(Angle::rad(val.acos()))
}

/// アークタンジェント（逆正接）の計算。
///
/// ```example
/// #calc.atan(0) \
/// #calc.atan(1)
/// ```
#[func(title = "Arctangent")]
pub fn atan(
    /// アークタンジェントを計算する値。
    value: Num,
) -> Angle {
    Angle::rad(value.float().atan())
}

/// 四象限アークタンジェントの計算。
///
/// 引数の順序は`(y, x)`ではなく`(x, y)`です。
///
/// ```example
/// #calc.atan2(1, 1) \
/// #calc.atan2(-2, -3)
/// ```
#[func(title = "Four-quadrant Arctangent")]
pub fn atan2(
    /// X座標。
    x: Num,
    /// Y座標。
    y: Num,
) -> Angle {
    Angle::rad(f64::atan2(y.float(), x.float()))
}

/// ハイパーボリックサイン（双曲線正弦）を計算。
///
/// ```example
/// #calc.sinh(0) \
/// #calc.sinh(1.5)
/// ```
#[func(title = "Hyperbolic Sine")]
pub fn sinh(
    /// ハイパーボリックサインを計算する双曲角。
    value: f64,
) -> f64 {
    value.sinh()
}

/// ハイパーボリックコサイン（双曲線余弦）を計算。
///
/// ```example
/// #calc.cosh(0) \
/// #calc.cosh(1.5)
/// ```
#[func(title = "Hyperbolic Cosine")]
pub fn cosh(
    /// ハイパーボリックコサインを計算する双曲角。
    value: f64,
) -> f64 {
    value.cosh()
}

/// ハイパーボリックタンジェント（双曲線正接）を計算。
///
/// ```example
/// #calc.tanh(0) \
/// #calc.tanh(1.5)
/// ```
#[func(title = "Hyperbolic Tangent")]
pub fn tanh(
    /// ハイパーボリックタンジェントを計算する双曲角。
    value: f64,
) -> f64 {
    value.tanh()
}

/// 数値の対数。
///
/// 底（てい）が指定されていない場合、対数は10を底として計算されます。
///
/// ```example
/// #calc.log(100)
/// ```
#[func(title = "Logarithm")]
pub fn log(
    span: Span,
    /// 対数を計算する数値。正の値（0を除く）である必要があります。
    value: Spanned<Num>,
    /// 対数の底（てい）。ゼロであってはなりません。
    #[named]
    #[default(Spanned::new(10.0, Span::detached()))]
    base: Spanned<f64>,
) -> SourceResult<f64> {
    let number = value.v.float();
    if number <= 0.0 {
        bail!(value.span, "value must be strictly positive")
    }

    if !base.v.is_normal() {
        bail!(base.span, "base may not be zero, NaN, infinite, or subnormal")
    }

    let result = if base.v == std::f64::consts::E {
        number.ln()
    } else if base.v == 2.0 {
        number.log2()
    } else if base.v == 10.0 {
        number.log10()
    } else {
        number.log(base.v)
    };

    if result.is_infinite() || result.is_nan() {
        bail!(span, "the result is not a real number")
    }

    Ok(result)
}

/// 数値の自然対数。
///
/// ```example
/// #calc.ln(calc.e)
/// ```
#[func(title = "Natural Logarithm")]
pub fn ln(
    span: Span,
    /// 対数を計算する数値。正の値（0を除く）である必要があります。
    value: Spanned<Num>,
) -> SourceResult<f64> {
    let number = value.v.float();
    if number <= 0.0 {
        bail!(value.span, "value must be strictly positive")
    }

    let result = number.ln();
    if result.is_infinite() {
        bail!(span, "result close to -inf")
    }

    Ok(result)
}

/// 数値の階乗。
///
/// ```example
/// #calc.fact(5)
/// ```
#[func(title = "Factorial")]
pub fn fact(
    /// 階乗を計算する数値。0または正の値である必要があります。
    number: u64,
) -> StrResult<i64> {
    Ok(fact_impl(1, number).ok_or_else(too_large)?)
}

/// 順列の計算。
///
/// 順列、つまり、`n`個の項目から`k`個を、順序を区別して選択する組み合わせの数を返します。
///
/// ```example
/// $ "perm"(n, k) &= n!/((n - k)!) \
///   "perm"(5, 3) &= #calc.perm(5, 3) $
/// ```
#[func(title = "Permutation")]
pub fn perm(
    /// 基数。0または正の値である必要があります。
    base: u64,
    /// 順列の数。0または正の値である必要があります。
    numbers: u64,
) -> StrResult<i64> {
    // By convention.
    if base < numbers {
        return Ok(0);
    }

    Ok(fact_impl(base - numbers + 1, base).ok_or_else(too_large)?)
}

/// Calculates the product of a range of numbers. Used to calculate
/// permutations. Returns None if the result is larger than `i64::MAX`
fn fact_impl(start: u64, end: u64) -> Option<i64> {
    // By convention
    if end + 1 < start {
        return Some(0);
    }

    let real_start: u64 = cmp::max(1, start);
    let mut count: u64 = 1;
    for i in real_start..=end {
        count = count.checked_mul(i)?;
    }

    count.try_into().ok()
}

/// 二項係数の計算。
///
/// `n`個の項目から`k`個を順序を区別せず選択する組み合わせの数を返します。
///
/// ```example
/// #calc.binom(10, 5)
/// ```
#[func(title = "Binomial")]
pub fn binom(
    /// 上側の係数。0または正の値である必要があります。
    n: u64,
    /// 下側の係数。0または正の値である必要があります。
    k: u64,
) -> StrResult<i64> {
    Ok(binom_impl(n, k).ok_or_else(too_large)?)
}

/// Calculates a binomial coefficient, with `n` the upper coefficient and `k`
/// the lower coefficient. Returns `None` if the result is larger than
/// `i64::MAX`
fn binom_impl(n: u64, k: u64) -> Option<i64> {
    if k > n {
        return Some(0);
    }

    // By symmetry
    let real_k = cmp::min(n - k, k);
    if real_k == 0 {
        return Some(1);
    }

    let mut result: u64 = 1;
    for i in 0..real_k {
        result = result.checked_mul(n - i)?.checked_div(i + 1)?;
    }

    result.try_into().ok()
}

/// 2つの整数値の最大公約数。
///
/// ```example
/// #calc.gcd(7, 42)
/// ```
#[func(title = "Greatest Common Divisor")]
pub fn gcd(
    /// 1つ目の整数値。
    a: i64,
    /// 2つ目の整数値。
    b: i64,
) -> i64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a.abs()
}

/// 2つの整数値の最小公倍数。
///
/// ```example
/// #calc.lcm(96, 13)
/// ```
#[func(title = "Least Common Multiple")]
pub fn lcm(
    /// 1つ目の整数値。
    a: i64,
    /// 2つ目の整数値。
    b: i64,
) -> StrResult<i64> {
    if a == b {
        return Ok(a.abs());
    }

    Ok(a.checked_div(gcd(a, b))
        .and_then(|gcd| gcd.checked_mul(b))
        .map(|v| v.abs())
        .ok_or_else(too_large)?)
}

/// 数値を最も近い整数値に切り捨て。
///
/// もしその値がすでに整数であれば、そのまま返されます。
///
/// この関数は常に[整数値]($int)を返し、結果となる[`float`]や[`decimal`]が64ビット符号付き整数の最大値より大きい、または最小値より小さい場合はエラーとなります。
///
/// ```example
/// #calc.floor(500.1)
/// #assert(calc.floor(3) == 3)
/// #assert(calc.floor(3.14) == 3)
/// #assert(calc.floor(decimal("-3.14")) == -4)
/// ```
#[func]
pub fn floor(
    /// 切り下げる数値。
    value: DecNum,
) -> StrResult<i64> {
    match value {
        DecNum::Int(n) => Ok(n),
        DecNum::Float(n) => Ok(crate::foundations::convert_float_to_int(n.floor())
            .map_err(|_| too_large())?),
        DecNum::Decimal(n) => Ok(i64::try_from(n.floor()).map_err(|_| too_large())?),
    }
}

/// 数値を最も近い整数値に切り上げ。
///
/// もしその値がすでに整数であれば、そのまま返されます。
///
/// この関数は常に[整数値]($int)を返し、結果となる[`float`]や[`decimal`]が64ビット符号付き整数の最大値より大きい、または最小値より小さい場合はエラーとなります。
///
/// ```example
/// #calc.ceil(500.1)
/// #assert(calc.ceil(3) == 3)
/// #assert(calc.ceil(3.14) == 4)
/// #assert(calc.ceil(decimal("-3.14")) == -3)
/// ```
#[func]
pub fn ceil(
    /// 切り上げる数値。
    value: DecNum,
) -> StrResult<i64> {
    match value {
        DecNum::Int(n) => Ok(n),
        DecNum::Float(n) => Ok(crate::foundations::convert_float_to_int(n.ceil())
            .map_err(|_| too_large())?),
        DecNum::Decimal(n) => Ok(i64::try_from(n.ceil()).map_err(|_| too_large())?),
    }
}

/// 数値の整数部分を切り出し。
///
/// もしその値がすでに整数であれば、そのまま返されます。
///
/// この関数は常に[整数値]($int)を返し、結果となる[`float`]や[`decimal`]が64ビット符号付き整数の最大値より大きい、または最小値より小さい場合はエラーとなります。
///
/// ```example
/// #calc.trunc(15.9)
/// #assert(calc.trunc(3) == 3)
/// #assert(calc.trunc(-3.7) == -3)
/// #assert(calc.trunc(decimal("8493.12949582390")) == 8493)
/// ```
#[func(title = "Truncate")]
pub fn trunc(
    /// 整数部分を切り出す数値。
    value: DecNum,
) -> StrResult<i64> {
    match value {
        DecNum::Int(n) => Ok(n),
        DecNum::Float(n) => Ok(crate::foundations::convert_float_to_int(n.trunc())
            .map_err(|_| too_large())?),
        DecNum::Decimal(n) => Ok(i64::try_from(n.trunc()).map_err(|_| too_large())?),
    }
}

/// 数値の小数部分を切り出し。
///
/// もしその値が整数であれば、`0`を返します。
///
/// ```example
/// #calc.fract(-3.1)
/// #assert(calc.fract(3) == 0)
/// #assert(calc.fract(decimal("234.23949211")) == decimal("0.23949211"))
/// ```
#[func(title = "Fractional")]
pub fn fract(
    /// 小数部分を切り出す数値。
    value: DecNum,
) -> DecNum {
    match value {
        DecNum::Int(_) => DecNum::Int(0),
        DecNum::Float(n) => DecNum::Float(n.fract()),
        DecNum::Decimal(n) => DecNum::Decimal(n.fract()),
    }
}

/// 数値を四捨五入します。
///
/// オプションで、小数点以下の桁数も指定できます。
///
/// 指定する桁数が負の値の場合、その絶対値が小数点より左側で切り捨てる有効整数桁数を示します。
///
/// この関数は、演算対象と同じ型の値を返します。つまり、[`float`]に`round`を適用すると`float`が、[`decimal`]に適用すると`decimal`が返されます。
/// 関数の出力を明示的に[`int`]にもできますが、その`float`や`decimal`が64ビット符号付き整数の最大値より大きい場合、または最小値より小さい場合はエラーとなることに注意してください。
///
/// さらに、この関数は、整数や`decimal`の最大値または最小値を超えて丸めようとするとエラーになる場合があります。数値が`float`の場合、そのような試みは、最大値や最小値に対してそれぞれ`{float.inf}`と`{-float.inf}`を返します。
/// ```example
/// #calc.round(3.1415, digits: 2)
/// #assert(calc.round(3) == 3)
/// #assert(calc.round(3.14) == 3)
/// #assert(calc.round(3.5) == 4.0)
/// #assert(calc.round(3333.45, digits: -2) == 3300.0)
/// #assert(calc.round(-48953.45, digits: -3) == -49000.0)
/// #assert(calc.round(3333, digits: -2) == 3300)
/// #assert(calc.round(-48953, digits: -3) == -49000)
/// #assert(calc.round(decimal("-6.5")) == decimal("-7"))
/// #assert(calc.round(decimal("7.123456789"), digits: 6) == decimal("7.123457"))
/// #assert(calc.round(decimal("3333.45"), digits: -2) == decimal("3300"))
/// #assert(calc.round(decimal("-48953.45"), digits: -3) == decimal("-49000"))
/// ```
#[func]
pub fn round(
    /// 四捨五入する数値。
    value: DecNum,
    /// 正の値の場合、小数点以下の桁数。
    ///
    /// 負の値の場合、小数点より左側で切り捨てる有効整数桁数。
    #[named]
    #[default(0)]
    digits: i64,
) -> StrResult<DecNum> {
    match value {
        DecNum::Int(n) => Ok(DecNum::Int(
            round_int_with_precision(n, digits.saturating_as::<i16>())
                .ok_or_else(too_large)?,
        )),
        DecNum::Float(n) => {
            Ok(DecNum::Float(round_with_precision(n, digits.saturating_as::<i16>())))
        }
        DecNum::Decimal(n) => Ok(DecNum::Decimal(
            n.round(digits.saturating_as::<i32>()).ok_or_else(too_large)?,
        )),
    }
}

/// 数値を最小値と最大値の間にクランプ。
///
/// ```example
/// #calc.clamp(5, 0, 4)
/// #assert(calc.clamp(5, 0, 10) == 5)
/// #assert(calc.clamp(5, 6, 10) == 6)
/// #assert(calc.clamp(decimal("5.45"), 2, decimal("45.9")) == decimal("5.45"))
/// #assert(calc.clamp(decimal("5.45"), decimal("6.75"), 12) == decimal("6.75"))
/// ```
#[func]
pub fn clamp(
    span: Span,
    /// クランプする数値。
    value: DecNum,
    /// 最小値（この値を含む）。
    min: DecNum,
    /// 最大値（この値を含む）。
    max: Spanned<DecNum>,
) -> SourceResult<DecNum> {
    // Ignore if there are incompatible types (decimal and float) since that
    // will cause `apply3` below to error before calling clamp, avoiding a
    // panic.
    if min
        .apply2(max.v, |min, max| max < min, |min, max| max < min, |min, max| max < min)
        .unwrap_or(false)
    {
        bail!(max.span, "max must be greater than or equal to min")
    }

    value
        .apply3(min, max.v, i64::clamp, f64::clamp, Decimal::clamp)
        .ok_or_else(cant_apply_to_decimal_and_float)
        .at(span)
}

/// 一連の値の最小値を決定。
///
/// ```example
/// #calc.min(1, -3, -5, 20, 3, 6) \
/// #calc.min("typst", "is", "cool")
/// ```
#[func(title = "Minimum")]
pub fn min(
    span: Span,
    /// 最小値を抽出する一連の値。空であってはなりません。
    #[variadic]
    values: Vec<Spanned<Value>>,
) -> SourceResult<Value> {
    minmax(span, values, Ordering::Less)
}

/// 一連の値の最大値を決定。
///
/// ```example
/// #calc.max(1, -3, -5, 20, 3, 6) \
/// #calc.max("typst", "is", "cool")
/// ```
#[func(title = "Maximum")]
pub fn max(
    span: Span,
    ///  最大値を抽出する一連の値。空であってはなりません。
    #[variadic]
    values: Vec<Spanned<Value>>,
) -> SourceResult<Value> {
    minmax(span, values, Ordering::Greater)
}

/// Find the minimum or maximum of a sequence of values.
fn minmax(
    span: Span,
    values: Vec<Spanned<Value>>,
    goal: Ordering,
) -> SourceResult<Value> {
    let mut iter = values.into_iter();
    let Some(Spanned { v: mut extremum, .. }) = iter.next() else {
        bail!(span, "expected at least one value");
    };

    for Spanned { v, span } in iter {
        let ordering = ops::compare(&v, &extremum).at(span)?;
        if ordering == goal {
            extremum = v;
        }
    }

    Ok(extremum)
}

/// 整数値が偶数かどうかを判定。
///
/// ```example
/// #calc.even(4) \
/// #calc.even(5) \
/// #range(10).filter(calc.even)
/// ```
#[func]
pub fn even(
    /// 偶数かどうかをチェックする数値。
    value: i64,
) -> bool {
    value % 2 == 0
}

/// 整数値が奇数かどうかを判断。
///
/// ```example
/// #calc.odd(4) \
/// #calc.odd(5) \
/// #range(10).filter(calc.odd)
/// ```
#[func]
pub fn odd(
    /// 奇数かどうかをチェックする数値。
    value: i64,
) -> bool {
    value % 2 != 0
}

/// 2つの数値の剰余を計算。
///
/// `calc.rem(x, y)`の値は常に`x`と同じ符号を持ち、`y`よりも小さい絶対値になります。
///
/// [`decimal`]が入力され、被除数が除数に比べて絶対値が小さすぎる場合はエラーになることがあります。
///
/// ```example
/// #calc.rem(7, 3) \
/// #calc.rem(7, -3) \
/// #calc.rem(-7, 3) \
/// #calc.rem(-7, -3) \
/// #calc.rem(1.75, 0.5)
/// ```
#[func(title = "Remainder")]
pub fn rem(
    span: Span,
    /// 剰余の被除数。
    dividend: DecNum,
    /// 剰余の除数。
    divisor: Spanned<DecNum>,
) -> SourceResult<DecNum> {
    if divisor.v.is_zero() {
        bail!(divisor.span, "divisor must not be zero");
    }

    dividend
        .apply2(
            divisor.v,
            |a, b| Some(DecNum::Int(a % b)),
            |a, b| Some(DecNum::Float(a % b)),
            |a, b| a.checked_rem(b).map(DecNum::Decimal),
        )
        .ok_or_else(cant_apply_to_decimal_and_float)
        .at(span)?
        .ok_or("dividend too small compared to divisor")
        .at(span)
}

/// 2つの数値のユークリッド除算を実行。
///
/// この計算の結果は、商を被除数が除数の`{n}`倍以上になる整数`{n}`に丸めた値です。
///
/// ```example
/// #calc.div-euclid(7, 3) \
/// #calc.div-euclid(7, -3) \
/// #calc.div-euclid(-7, 3) \
/// #calc.div-euclid(-7, -3) \
/// #calc.div-euclid(1.75, 0.5) \
/// #calc.div-euclid(decimal("1.75"), decimal("0.5"))
/// ```
#[func(title = "Euclidean Division")]
pub fn div_euclid(
    span: Span,
    /// 除算の被除数。
    dividend: DecNum,
    /// 除算の除数。
    divisor: Spanned<DecNum>,
) -> SourceResult<DecNum> {
    if divisor.v.is_zero() {
        bail!(divisor.span, "divisor must not be zero");
    }

    dividend
        .apply2(
            divisor.v,
            |a, b| Some(DecNum::Int(a.div_euclid(b))),
            |a, b| Some(DecNum::Float(a.div_euclid(b))),
            |a, b| a.checked_div_euclid(b).map(DecNum::Decimal),
        )
        .ok_or_else(cant_apply_to_decimal_and_float)
        .at(span)?
        .ok_or_else(too_large)
        .at(span)
}

/// 除算の最小の非負剰余を計算。
///
/// 警告：浮動小数点数の丸め誤差により、被除数が除数よりも極端に小さく、かつ負の値である場合、剰余が除数の絶対値と等しくなる可能性があります。
/// これは浮動小数点数の入力にのみ当てはまります。
///
/// また、[`decimal`]を入力した場合、被除数が除数に比べて桁違いに小さい場合はエラーとなることがあります。
///
/// ```example
/// #calc.rem-euclid(7, 3) \
/// #calc.rem-euclid(7, -3) \
/// #calc.rem-euclid(-7, 3) \
/// #calc.rem-euclid(-7, -3) \
/// #calc.rem-euclid(1.75, 0.5) \
/// #calc.rem-euclid(decimal("1.75"), decimal("0.5"))
/// ```
#[func(title = "Euclidean Remainder", keywords = ["modulo", "modulus"])]
pub fn rem_euclid(
    span: Span,
    /// 剰余の被除数。
    dividend: DecNum,
    /// 剰余の除数。
    divisor: Spanned<DecNum>,
) -> SourceResult<DecNum> {
    if divisor.v.is_zero() {
        bail!(divisor.span, "divisor must not be zero");
    }

    dividend
        .apply2(
            divisor.v,
            |a, b| Some(DecNum::Int(a.rem_euclid(b))),
            |a, b| Some(DecNum::Float(a.rem_euclid(b))),
            |a, b| a.checked_rem_euclid(b).map(DecNum::Decimal),
        )
        .ok_or_else(cant_apply_to_decimal_and_float)
        .at(span)?
        .ok_or("dividend too small compared to divisor")
        .at(span)
}

/// 2つの数値の商（切り捨て除算）を計算します。
///
/// この関数は常に[整数値]($int)を返し、結果となる[`float`]や[`decimal`]が64ビット符号付き整数の最大値より大きい、または最小値より小さい場合はエラーとなることに注意してください。
///
/// ```example
/// $ "quo"(a, b) &= floor(a/b) \
///   "quo"(14, 5) &= #calc.quo(14, 5) \
///   "quo"(3.46, 0.5) &= #calc.quo(3.46, 0.5) $
/// ```
#[func(title = "Quotient")]
pub fn quo(
    span: Span,
    /// 商の被除数。
    dividend: DecNum,
    /// 商の除数。
    divisor: Spanned<DecNum>,
) -> SourceResult<i64> {
    if divisor.v.is_zero() {
        bail!(divisor.span, "divisor must not be zero");
    }

    let divided = dividend
        .apply2(
            divisor.v,
            |a, b| Some(DecNum::Int(a / b)),
            |a, b| Some(DecNum::Float(a / b)),
            |a, b| a.checked_div(b).map(DecNum::Decimal),
        )
        .ok_or_else(cant_apply_to_decimal_and_float)
        .at(span)?
        .ok_or_else(too_large)
        .at(span)?;

    floor(divided).at(span)
}

/// 一連の値のpノルムを計算。
///
/// ```example
/// #calc.norm(1, 2, -3, 0.5) \
/// #calc.norm(p: 3, 1, 2)
/// ```
#[func(title = "𝑝-Norm")]
pub fn norm(
    /// pノルムを計算するためのpの値。
    #[named]
    #[default(Spanned::new(2.0, Span::detached()))]
    p: Spanned<f64>,
    /// pノルムを計算する一連の値。空の場合、`0.0`を返します。
    #[variadic]
    values: Vec<f64>,
) -> SourceResult<f64> {
    if p.v <= 0.0 {
        bail!(p.span, "p must be greater than zero");
    }

    // Create an iterator over the absolute values.
    let abs = values.into_iter().map(f64::abs);

    Ok(if p.v.is_infinite() {
        // When p is infinity, the p-norm is the maximum of the absolute values.
        abs.max_by(|a, b| a.total_cmp(b)).unwrap_or(0.0)
    } else {
        abs.map(|v| v.powf(p.v)).sum::<f64>().powf(1.0 / p.v)
    })
}

/// A value which can be passed to functions that work with integers and floats.
#[derive(Debug, Copy, Clone)]
pub enum Num {
    Int(i64),
    Float(f64),
}

impl Num {
    fn float(self) -> f64 {
        match self {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
        }
    }
}

cast! {
    Num,
    self => match self {
        Self::Int(v) => v.into_value(),
        Self::Float(v) => v.into_value(),
    },
    v: i64 => Self::Int(v),
    v: f64 => Self::Float(v),
}

/// A value which can be passed to functions that work with integers, floats,
/// and decimals.
#[derive(Debug, Copy, Clone)]
pub enum DecNum {
    Int(i64),
    Float(f64),
    Decimal(Decimal),
}

impl DecNum {
    /// Checks if this number is equivalent to zero.
    fn is_zero(self) -> bool {
        match self {
            Self::Int(i) => i == 0,
            Self::Float(f) => f == 0.0,
            Self::Decimal(d) => d.is_zero(),
        }
    }

    /// If this `DecNum` holds an integer or float, returns a float.
    /// Otherwise, returns `None`.
    fn float(self) -> Option<f64> {
        match self {
            Self::Int(i) => Some(i as f64),
            Self::Float(f) => Some(f),
            Self::Decimal(_) => None,
        }
    }

    /// If this `DecNum` holds an integer or decimal, returns a decimal.
    /// Otherwise, returns `None`.
    fn decimal(self) -> Option<Decimal> {
        match self {
            Self::Int(i) => Some(Decimal::from(i)),
            Self::Float(_) => None,
            Self::Decimal(d) => Some(d),
        }
    }

    /// Tries to apply a function to two decimal or numeric arguments.
    ///
    /// Fails with `None` if one is a float and the other is a decimal.
    fn apply2<T>(
        self,
        other: Self,
        int: impl FnOnce(i64, i64) -> T,
        float: impl FnOnce(f64, f64) -> T,
        decimal: impl FnOnce(Decimal, Decimal) -> T,
    ) -> Option<T> {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Some(int(a, b)),
            (Self::Decimal(a), Self::Decimal(b)) => Some(decimal(a, b)),
            (Self::Decimal(a), Self::Int(b)) => Some(decimal(a, Decimal::from(b))),
            (Self::Int(a), Self::Decimal(b)) => Some(decimal(Decimal::from(a), b)),
            (a, b) => Some(float(a.float()?, b.float()?)),
        }
    }

    /// Tries to apply a function to three decimal or numeric arguments.
    ///
    /// Fails with `None` if one is a float and the other is a decimal.
    fn apply3(
        self,
        other: Self,
        third: Self,
        int: impl FnOnce(i64, i64, i64) -> i64,
        float: impl FnOnce(f64, f64, f64) -> f64,
        decimal: impl FnOnce(Decimal, Decimal, Decimal) -> Decimal,
    ) -> Option<Self> {
        match (self, other, third) {
            (Self::Int(a), Self::Int(b), Self::Int(c)) => Some(Self::Int(int(a, b, c))),
            (Self::Decimal(a), b, c) => {
                Some(Self::Decimal(decimal(a, b.decimal()?, c.decimal()?)))
            }
            (a, Self::Decimal(b), c) => {
                Some(Self::Decimal(decimal(a.decimal()?, b, c.decimal()?)))
            }
            (a, b, Self::Decimal(c)) => {
                Some(Self::Decimal(decimal(a.decimal()?, b.decimal()?, c)))
            }
            (a, b, c) => Some(Self::Float(float(a.float()?, b.float()?, c.float()?))),
        }
    }
}

cast! {
    DecNum,
    self => match self {
        Self::Int(v) => v.into_value(),
        Self::Float(v) => v.into_value(),
        Self::Decimal(v) => v.into_value(),
    },
    v: i64 => Self::Int(v),
    v: f64 => Self::Float(v),
    v: Decimal => Self::Decimal(v),
}

/// A value that can be passed to a trigonometric function.
pub enum AngleLike {
    Int(i64),
    Float(f64),
    Angle(Angle),
}

cast! {
    AngleLike,
    v: i64 => Self::Int(v),
    v: f64 => Self::Float(v),
    v: Angle => Self::Angle(v),
}

/// The error message when the result is too large to be represented.
#[cold]
fn too_large() -> &'static str {
    "the result is too large"
}

/// The hinted error message when trying to apply an operation to decimal and
/// float operands.
#[cold]
fn cant_apply_to_decimal_and_float() -> HintedString {
    HintedString::new("cannot apply this operation to a decimal and a float".into())
        .with_hint(
            "if loss of precision is acceptable, explicitly cast the \
             decimal to a float with `float(value)`",
        )
}
