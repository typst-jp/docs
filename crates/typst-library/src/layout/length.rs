use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg};

use comemo::Tracked;
use ecow::{eco_format, EcoString};
use typst_syntax::Span;
use typst_utils::Numeric;

use crate::diag::{bail, HintedStrResult, SourceResult};
use crate::foundations::{func, scope, ty, Context, Fold, Repr, Resolve, StyleChain};
use crate::layout::{Abs, Em};

/// 文脈に応じた単位で表現される場合もある、大きさまたは距離。
///
/// Typstは以下の長さの単位をサポートしています。
///
/// - ポイント: `{72pt}`
/// - ミリメートル: `{254mm}`
/// - センチメートル: `{2.54cm}`
/// - インチ: `{1in}`
/// - 相対フォントサイズ: `{2.5em}`
///
/// 長さは整数や浮動小数点数で乗除算できます。
///
/// # 例
/// ```example
/// #rect(width: 20pt)
/// #rect(width: 2em)
/// #rect(width: 1in)
///
/// #(3em + 5pt).em \
/// #(20pt).em \
/// #(40em + 2pt).abs \
/// #(5em).abs
/// ```
///
/// # フィールド
/// - `abs`: 現在の長さの単なる数値部分（すなわち`em`部分を除いたもの）。
/// - `em`: [float]としての、このlengthでの`em`単位の大きさ。
#[ty(scope, cast)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Length {
    /// The absolute part.
    pub abs: Abs,
    /// The font-relative part.
    pub em: Em,
}

impl Length {
    /// The zero length.
    pub const fn zero() -> Self {
        Self { abs: Abs::zero(), em: Em::zero() }
    }

    /// Try to compute the absolute value of the length.
    pub fn try_abs(self) -> Option<Self> {
        (self.abs.is_zero() || self.em.is_zero())
            .then(|| Self { abs: self.abs.abs(), em: self.em.abs() })
    }

    /// Try to divide two lengths.
    pub fn try_div(self, other: Self) -> Option<f64> {
        if self.abs.is_zero() && other.abs.is_zero() {
            Some(self.em / other.em)
        } else if self.em.is_zero() && other.em.is_zero() {
            Some(self.abs / other.abs)
        } else {
            None
        }
    }

    /// Convert to an absolute length at the given font size.
    pub fn at(self, font_size: Abs) -> Abs {
        self.abs + self.em.at(font_size)
    }

    /// Fails with an error if the length has a non-zero font-relative part.
    fn ensure_that_em_is_zero(&self, span: Span, unit: &str) -> SourceResult<()> {
        if self.em == Em::zero() {
            return Ok(());
        }

        bail!(
            span,
            "cannot convert a length with non-zero em units (`{}`) to {unit}",
            self.repr();
            hint: "use `length.to-absolute()` to resolve its em component \
                   (requires context)";
            hint: "or use `length.abs.{unit}()` instead to ignore its em component"
        )
    }
}

#[scope]
impl Length {
    /// このlengthをポイントに変換します。
    ///
    /// このlengthの`em`単位の値が（単に`2pt`ではなく`5em + 2pt`のように）非ゼロの場合にエラーが発生して失敗します。
    /// 長さの`em`成分を無視するために（`(5em + 2pt).abs.pt()`のように）`abs`フィールドを使用してください（したがって数値部分のみが変換されます）。
    #[func(name = "pt", title = "Points")]
    pub fn to_pt(&self, span: Span) -> SourceResult<f64> {
        self.ensure_that_em_is_zero(span, "pt")?;
        Ok(self.abs.to_pt())
    }

    /// このlengthをミリメートルに変換します。
    ///
    /// このlengthの`em`単位の値が非ゼロの場合にエラーが発生して失敗します。
    /// 詳細は[`pt`]($length.pt)メソッドを参照して下さい。
    #[func(name = "mm", title = "Millimeters")]
    pub fn to_mm(&self, span: Span) -> SourceResult<f64> {
        self.ensure_that_em_is_zero(span, "mm")?;
        Ok(self.abs.to_mm())
    }

    /// このlengthをセンチメートルに変換します。
    ///
    /// このlengthの`em`単位の値が非ゼロの場合にエラーが発生して失敗します。
    /// 詳細は[`pt`]($length.pt)メソッドを参照して下さい。
    #[func(name = "cm", title = "Centimeters")]
    pub fn to_cm(&self, span: Span) -> SourceResult<f64> {
        self.ensure_that_em_is_zero(span, "cm")?;
        Ok(self.abs.to_cm())
    }

    /// このlengthをインチに変換します。
    ///
    /// このlengthの`em`単位の値が非ゼロの場合にエラーが発生して失敗します。
    /// 詳細は[`pt`]($length.pt)メソッドを参照して下さい。
    #[func(name = "inches")]
    pub fn to_inches(&self, span: Span) -> SourceResult<f64> {
        self.ensure_that_em_is_zero(span, "inches")?;
        Ok(self.abs.to_inches())
    }

    /// このlengthを絶対的な長さに変換します。
    ///
    /// ```example
    /// #set text(size: 12pt)
    /// #context [
    ///   #(6pt).to-absolute() \
    ///   #(6pt + 10em).to-absolute() \
    ///   #(10em).to-absolute()
    /// ]
    ///
    /// #set text(size: 6pt)
    /// #context [
    ///   #(6pt).to-absolute() \
    ///   #(6pt + 10em).to-absolute() \
    ///   #(10em).to-absolute()
    /// ]
    /// ```
    #[func]
    pub fn to_absolute(&self, context: Tracked<Context>) -> HintedStrResult<Length> {
        Ok(self.resolve(context.styles()?).into())
    }
}

impl Debug for Length {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (self.abs.is_zero(), self.em.is_zero()) {
            (false, false) => write!(f, "{:?} + {:?}", self.abs, self.em),
            (true, false) => self.em.fmt(f),
            (_, true) => self.abs.fmt(f),
        }
    }
}

impl Repr for Length {
    fn repr(&self) -> EcoString {
        match (self.abs.is_zero(), self.em.is_zero()) {
            (false, false) => eco_format!("{} + {}", self.abs.repr(), self.em.repr()),
            (true, false) => self.em.repr(),
            (_, true) => self.abs.repr(),
        }
    }
}

impl Numeric for Length {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_finite(self) -> bool {
        self.abs.is_finite() && self.em.is_finite()
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.em.is_zero() && other.em.is_zero() {
            self.abs.partial_cmp(&other.abs)
        } else if self.abs.is_zero() && other.abs.is_zero() {
            self.em.partial_cmp(&other.em)
        } else {
            None
        }
    }
}

impl From<Abs> for Length {
    fn from(abs: Abs) -> Self {
        Self { abs, em: Em::zero() }
    }
}

impl From<Em> for Length {
    fn from(em: Em) -> Self {
        Self { abs: Abs::zero(), em }
    }
}

impl Neg for Length {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { abs: -self.abs, em: -self.em }
    }
}

impl Add for Length {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { abs: self.abs + rhs.abs, em: self.em + rhs.em }
    }
}

typst_utils::sub_impl!(Length - Length -> Length);

impl Mul<f64> for Length {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { abs: self.abs * rhs, em: self.em * rhs }
    }
}

impl Mul<Length> for f64 {
    type Output = Length;

    fn mul(self, rhs: Length) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Length {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self { abs: self.abs / rhs, em: self.em / rhs }
    }
}

typst_utils::assign_impl!(Length += Length);
typst_utils::assign_impl!(Length -= Length);
typst_utils::assign_impl!(Length *= f64);
typst_utils::assign_impl!(Length /= f64);

impl Resolve for Length {
    type Output = Abs;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        self.abs + self.em.resolve(styles)
    }
}

impl Fold for Length {
    fn fold(self, _: Self) -> Self {
        self
    }
}
