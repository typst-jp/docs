use crate::foundations::{cast, elem, Content, Func, Smart};
use crate::layout::{Abs, Angle, Length, Ratio, Rel};
use crate::math::Mathy;
use crate::visualize::Stroke;

/// 数式の一部分上に対角線を表示。
///
/// 項の除去を示すのによく使われます。
///
/// # 例
/// ```example
/// >>> #set page(width: 140pt)
/// Here, we can simplify:
/// $ (a dot b dot cancel(x)) /
///     cancel(x) $
/// ```
#[elem(Mathy)]
pub struct CancelElem {
    /// 線が配置されるべきコンテンツ。
    #[required]
    pub body: Content,

    /// 「キャンセル」したい要素全体をまたぐ対角線を基準とした相対的な線の長さ。
    /// `{100%}`という値は、要素の対角線と正確に一致する長さになります。
    ///
    /// ```example
    /// >>> #set page(width: 140pt)
    /// $ a + cancel(x, length: #200%)
    ///     - cancel(x, length: #200%) $
    /// ```
    #[resolve]
    #[default(Rel::new(Ratio::one(), Abs::pt(3.0).into()))]
    pub length: Rel<Length>,

    /// 打ち消し線を（y軸に関して）反転させるべきかどうか。
    /// デフォルト角度設定では、反転により打ち消し線が右上ではなく左上を指すようになります。
    ///
    /// ```example
    /// >>> #set page(width: 140pt)
    /// $ (a cancel((b + c), inverted: #true)) /
    ///     cancel(b + c, inverted: #true) $
    /// ```
    #[default(false)]
    pub inverted: bool,

    /// 要素上で交差する、相対する2つの打ち消し線を描画するかどうか。
    /// これは`inverted`を上書きします。
    ///
    /// ```example
    /// >>> #set page(width: 140pt)
    /// $ cancel(Pi, cross: #true) $
    /// ```
    #[default(false)]
    pub cross: bool,

    /// 打ち消し線をどれくらい回転させるか。
    ///
    /// - 角度が与えられた場合、y軸方向を0度として時計回りに与えられた角度だけ線が回転します。
    /// - `{auto}`の場合、デフォルトの角度を用いた線となります。すなわち、コンテンツのボックスの右上がりの対角線に沿うものになります。
    /// - `angle => angle`の形の関数が与えられた場合、y軸方向を0度として、その関数が返す角度で線が回転します。
    /// 関数は入力としてデフォルトの角度を受け取ります。
    ///
    /// ```example
    /// >>> #set page(width: 140pt)
    /// $ cancel(Pi)
    ///   cancel(Pi, angle: #0deg)
    ///   cancel(Pi, angle: #45deg)
    ///   cancel(Pi, angle: #90deg)
    ///   cancel(1/(1+x), angle: #(a => a + 45deg))
    ///   cancel(1/(1+x), angle: #(a => a + 90deg)) $
    /// ```
    pub angle: Smart<CancelAngle>,

    /// 打ち消し線の[ストローク]($stroke)をどうするか。
    ///
    /// ```example
    /// >>> #set page(width: 140pt)
    /// $ cancel(
    ///   sum x,
    ///   stroke: #(
    ///     paint: red,
    ///     thickness: 1.5pt,
    ///     dash: "dashed",
    ///   ),
    /// ) $
    /// ```
    #[resolve]
    #[fold]
    #[default(Stroke {
        // Default stroke has 0.5pt for better visuals.
        thickness: Smart::Custom(Abs::pt(0.5).into()),
        ..Default::default()
    })]
    pub stroke: Stroke,
}

/// Defines the cancel line.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CancelAngle {
    Angle(Angle),
    Func(Func),
}

cast! {
    CancelAngle,
    self => match self {
        Self::Angle(v) => v.into_value(),
        Self::Func(v) => v.into_value()
    },
    v: Angle => CancelAngle::Angle(v),
    v: Func => CancelAngle::Func(v),
}
