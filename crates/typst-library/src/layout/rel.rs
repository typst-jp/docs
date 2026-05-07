use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ecow::{EcoString, eco_format};
use typst_utils::Numeric;

use crate::foundations::{Fold, Repr, Resolve, StyleChain, cast, ty};
use crate::layout::{Abs, Em, Length, Ratio};

/// 既知の長さに対する相対的な長さ。
///
/// この型は[length]と[ratio]の組み合わせです。
/// これは長さと百分率の加減算の結果になります。
/// 相対長さが想定されているあらゆる箇所で長さまたは百分率を単体でも指定可能です。
///
/// # ページに対する相対的な長さ
/// よくある利用例として、（[ブロック]($block)、[長方形]($rect)などの）レイアウト要素の幅や高さをページ幅に対する百分率の値を用いて設定する場合があります。
/// 以下の例では長方形の幅が`{25%}`に設定されており、ページの_内部_幅（幅からマージンを引いたもの）の4分の1を占めます。
///
/// ```example
/// #rect(width: 25%)
/// ```
///
/// 相対長さが想定されているあらゆる箇所で長さや百分率を単体で指定できますが、両者を自由に組み合わせられます。
/// ```example
/// #rect(width: 25% + 1cm)
/// ```
///
/// ページの_全体_幅を占めるように要素のサイズを設定したい場合、いくつかの選択肢があります（具体的な用途に大きく依存します）。
///
/// 1. ページのマージンを`{0pt}`に設定する（`[#set page(margin: 0pt)]`）
/// 2. 既知のページ全体の幅に百分率を乗算する（`{21cm * 69%}`）
/// 3. マージンを相殺するパディングを使用する（`[#pad(x: -2.5cm, ...)]`）
/// 4. ページの[background](page.background)もしくは[foreground](page.foreground)フィールドを使用する。
///    これらはマージンを考慮しません（コンテンツは文書のフローの外側に描画されることに注意してください。
///    コンテンツの位置を制御するには[place]のドキュメントを参照してください）。
///
/// # コンテナに対する相対的な長さ
/// （[長方形]($rect)などの）レイアウト要素がページの直接の子孫ではなく、（[ブロック]($block)などの）別のレイアウトコンテナの中に入れ子になっている場合、相対的な幅はコンテナを基準とした値になります。
///
/// ```example
/// #block(
///   width: 100pt,
///   fill: aqua,
///   rect(width: 50%),
/// )
/// ```
///
/// # スクリプト記述
/// 相対長さは[百分率]($ratio)、[整数]($int)、[浮動小数点数]($float)と乗算できます。
///
/// 相対長さは以下のフィールドを持ちます。
/// - `length`: 長さ成分。
/// - `ratio`: 百分率成分。
///
/// ```example
/// #(100% - 50pt).length \
/// #(100% - 50pt).ratio
/// ```
#[ty(cast, name = "relative", title = "Relative Length")]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Rel<T: Numeric = Length> {
    /// 相対成分。
    pub rel: Ratio,
    /// 絶対成分。
    pub abs: T,
}

impl<T: Numeric> Rel<T> {
    /// The zero relative.
    pub fn zero() -> Self {
        Self { rel: Ratio::zero(), abs: T::zero() }
    }

    /// A relative with a ratio of `100%` and no absolute part.
    pub fn one() -> Self {
        Self { rel: Ratio::one(), abs: T::zero() }
    }

    /// Create a new relative from its parts.
    pub const fn new(rel: Ratio, abs: T) -> Self {
        Self { rel, abs }
    }

    /// Whether both parts are zero.
    pub fn is_zero(self) -> bool {
        self.rel.is_zero() && self.abs == T::zero()
    }

    /// Whether the relative part is one and the absolute part is zero.
    pub fn is_one(self) -> bool {
        self.rel.is_one() && self.abs == T::zero()
    }

    /// Evaluate this relative to the given `whole`.
    pub fn relative_to(self, whole: T) -> T {
        self.rel.of(whole) + self.abs
    }

    /// Map the absolute part with `f`.
    pub fn map<F, U>(self, f: F) -> Rel<U>
    where
        F: FnOnce(T) -> U,
        U: Numeric,
    {
        Rel { rel: self.rel, abs: f(self.abs) }
    }
}

impl Rel<Length> {
    /// Try to divide two relative lengths.
    pub fn try_div(self, other: Self) -> Option<f64> {
        if self.rel.is_zero() && other.rel.is_zero() {
            self.abs.try_div(other.abs)
        } else if self.abs.is_zero() && other.abs.is_zero() {
            Some(self.rel / other.rel)
        } else {
            None
        }
    }
}

impl<T: Numeric + Debug> Debug for Rel<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (self.rel.is_zero(), self.abs.is_zero()) {
            (false, false) => write!(f, "{:?} + {:?}", self.rel, self.abs),
            (false, true) => self.rel.fmt(f),
            (true, _) => self.abs.fmt(f),
        }
    }
}

impl<T: Numeric + Repr> Repr for Rel<T> {
    fn repr(&self) -> EcoString {
        eco_format!("{} + {}", self.rel.repr(), self.abs.repr())
    }
}

impl From<Abs> for Rel<Length> {
    fn from(abs: Abs) -> Self {
        Rel::from(Length::from(abs))
    }
}

impl From<Em> for Rel<Length> {
    fn from(em: Em) -> Self {
        Rel::from(Length::from(em))
    }
}

impl<T: Numeric> From<T> for Rel<T> {
    fn from(abs: T) -> Self {
        Self { rel: Ratio::zero(), abs }
    }
}

impl<T: Numeric> From<Ratio> for Rel<T> {
    fn from(rel: Ratio) -> Self {
        Self { rel, abs: T::zero() }
    }
}

impl<T: Numeric + PartialOrd> PartialOrd for Rel<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rel.is_zero() && other.rel.is_zero() {
            self.abs.partial_cmp(&other.abs)
        } else if self.abs.is_zero() && other.abs.is_zero() {
            self.rel.partial_cmp(&other.rel)
        } else {
            None
        }
    }
}

impl<T: Numeric> Neg for Rel<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self { rel: -self.rel, abs: -self.abs }
    }
}

impl<T: Numeric> Add for Rel<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            rel: self.rel + other.rel,
            abs: self.abs + other.abs,
        }
    }
}

impl<T: Numeric> Sub for Rel<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl<T: Numeric> Mul<f64> for Rel<T> {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self { rel: self.rel * other, abs: self.abs * other }
    }
}

impl<T: Numeric> Mul<Rel<T>> for f64 {
    type Output = Rel<T>;

    fn mul(self, other: Rel<T>) -> Self::Output {
        other * self
    }
}

impl<T: Numeric> Div<f64> for Rel<T> {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self { rel: self.rel / other, abs: self.abs / other }
    }
}

impl<T: Numeric + AddAssign> AddAssign for Rel<T> {
    fn add_assign(&mut self, other: Self) {
        self.rel += other.rel;
        self.abs += other.abs;
    }
}

impl<T: Numeric + SubAssign> SubAssign for Rel<T> {
    fn sub_assign(&mut self, other: Self) {
        self.rel -= other.rel;
        self.abs -= other.abs;
    }
}

impl<T: Numeric + MulAssign<f64>> MulAssign<f64> for Rel<T> {
    fn mul_assign(&mut self, other: f64) {
        self.rel *= other;
        self.abs *= other;
    }
}

impl<T: Numeric + DivAssign<f64>> DivAssign<f64> for Rel<T> {
    fn div_assign(&mut self, other: f64) {
        self.rel /= other;
        self.abs /= other;
    }
}

impl<T: Numeric> Add<T> for Ratio {
    type Output = Rel<T>;

    fn add(self, other: T) -> Self::Output {
        Rel::from(self) + Rel::from(other)
    }
}

impl<T: Numeric> Add<T> for Rel<T> {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        self + Rel::from(other)
    }
}

impl<T: Numeric> Add<Ratio> for Rel<T> {
    type Output = Self;

    fn add(self, other: Ratio) -> Self::Output {
        self + Rel::from(other)
    }
}

impl<T> Resolve for Rel<T>
where
    T: Resolve + Numeric,
    <T as Resolve>::Output: Numeric,
{
    type Output = Rel<<T as Resolve>::Output>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        self.map(|abs| abs.resolve(styles))
    }
}

impl<T> Fold for Rel<T>
where
    T: Numeric + Fold,
{
    fn fold(self, outer: Self) -> Self {
        Self { rel: self.rel, abs: self.abs.fold(outer.abs) }
    }
}

cast! {
    Rel<Abs>,
    self => self.map(Length::from).into_value(),
}
