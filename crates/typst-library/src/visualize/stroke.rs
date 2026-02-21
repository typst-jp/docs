use ecow::EcoString;
use typst_utils::{Numeric, Scalar};

use crate::diag::{HintedStrResult, SourceResult};
use crate::foundations::{
    cast, dict, func, scope, ty, Args, Cast, Dict, Fold, FromValue, NoneValue, Repr,
    Resolve, Smart, StyleChain, Value,
};
use crate::layout::{Abs, Length};
use crate::visualize::{Color, Gradient, Paint, Tiling};

/// 線がどのように描画されるかを定義します。
///
/// ストロークはペイント（単一の色またはグラデーション）、太さ、ラインキャップ、線の接続、マイターリミット、および破線パターンを持ちます。
/// これらの値は全てオプションであり実用的なデフォルト値を持ちます。
///
/// # 例
/// ```example
/// #set line(length: 100%)
/// #stack(
///   spacing: 1em,
///   line(stroke: 2pt + red),
///   line(stroke: (paint: blue, thickness: 4pt, cap: "round")),
///   line(stroke: (paint: blue, thickness: 1pt, dash: "dashed")),
///   line(stroke: 2pt + gradient.linear(..color.map.rainbow)),
/// )
/// ```
///
/// # 単純なストローク
/// 色、線の太さ、またはそれら2つの組み合わせによって単純な単一色の線を作成できます。
/// 具体的にはストロークが期待される場所ならどこでも、以下の値をどれでも渡せます。
///
/// - 線の太さを決定するlength。色はデフォルトの黒が継承されます。
/// - ストロークに使用されるcolor。太さはデフォルトの`{1pt}`が継承されます。
/// - `{2pt + red}`のような`+`演算子を用いたcolorとthicknessの組み合わせによるストローク。
///
/// 完全な制御のために、ストロークを期待する任意の関数に対して[dictionary]または`{stroke}`オブジェクトを提供することもできます。
/// dictionaryのキーには、以下のコンストラクタ関数のパラメータであればどれでも含められます。
///
/// # フィールド
/// ストロークオブジェクトでは、コンストラクタ関数で列挙されている任意のフィールドにアクセスできます。
/// 例えば`{(2pt + blue).thickness}`の値は`{2pt}`となります。
/// 一方で`{stroke(red).cap}`の値は指定されていないため`{auto}`となります。`{auto}`に設定されたフィールドの値は継承されます。
#[ty(scope, cast)]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Stroke<T: Numeric = Length> {
    /// ストロークの色。
    pub paint: Smart<Paint>,
    /// ストロークの太さ。
    pub thickness: Smart<T>,
    /// ストロークののラインキャップ。
    pub cap: Smart<LineCap>,
    /// ストロークの線の接続。
    pub join: Smart<LineJoin>,
    /// ストロークの破線パターン。
    pub dash: Smart<Option<DashPattern<T>>>,
    /// マイターリミット。
    pub miter_limit: Smart<Scalar>,
}

impl Stroke {
    /// paintとthicknessからストロークを作成します。
    pub fn from_pair(paint: impl Into<Paint>, thickness: Length) -> Self {
        Self {
            paint: Smart::Custom(paint.into()),
            thickness: Smart::Custom(thickness),
            ..Default::default()
        }
    }
}

#[scope]
impl Stroke {
    /// 値をストロークへ変換するか、または与えられたパラメータからストロークを作成します。
    ///
    /// ほとんどの場合は自動的に変換が行われるため、ストロークを使用する際に値を明示的に変換する必要はありません。
    /// しかし、このコンストラクタを使うことで値がストロークの全てのフィールドを含んでいることを保証できます。
    ///
    /// ```example
    /// #let my-func(x) = {
    ///     x = stroke(x) // Convert to a stroke
    ///     [Stroke has thickness #x.thickness.]
    /// }
    /// #my-func(3pt) \
    /// #my-func(red) \
    /// #my-func(stroke(cap: "round", thickness: 1pt))
    /// ```
    #[func(constructor)]
    pub fn construct(
        args: &mut Args,

        /// ストロークに使用される色またはグラデーション。
        ///
        /// `{auto}`に指定された場合、デフォルトの値である`{black}`が継承されます。
        #[external]
        paint: Smart<Paint>,

        /// ストロークの太さ。
        ///
        /// `{auto}`に指定された場合、デフォルトの値である`{1pt}`が継承されます。
        #[external]
        thickness: Smart<Length>,

        /// ストロークの終端がどのように描画されるか。
        ///
        /// `{auto}`に指定された場合、デフォルトの値である`{"butt"}`が継承されます。
        #[external]
        cap: Smart<LineCap>,

        /// 鋭い線の折り返しがどのように描画されるか。
        ///
        /// `{auto}`に指定された場合、デフォルトの値である`{"miter"}`が継承されます。
        #[external]
        join: Smart<LineJoin>,

        /// 使用する破線パターン。この値は次のいずれかを使用できます。
        ///
        /// - いずれかの事前定義パターン：
        ///   - `{"solid"}` または `{none}`
        ///   - `{"dotted"}`
        ///   - `{"densely-dotted"}`
        ///   - `{"loosely-dotted"}`
        ///   - `{"dashed"}`
        ///   - `{"densely-dashed"}`
        ///   - `{"loosely-dashed"}`
        ///   - `{"dash-dotted"}`
        ///   - `{"densely-dash-dotted"}`
        ///   - `{"loosely-dash-dotted"}`
        /// - 破線とその間隔の長さを交互に持つ[array]。
        ///   長さが線の太さと等しい場合は文字列`{"dot"}`を用いることもできます。
        /// - `array`（上記の配列と同様）とそのパターンが開始される場所を定義する`phase`（型は[length]）をキーに含む[dictionary]。
        ///
        /// `{auto}`に指定された場合、デフォルトの値である`{none}`が継承されます。
        ///
        /// ```example
        /// #set line(length: 100%, stroke: 2pt)
        /// #stack(
        ///   spacing: 1em,
        ///   line(stroke: (dash: "dashed")),
        ///   line(stroke: (dash: (10pt, 5pt, "dot", 5pt))),
        ///   line(stroke: (dash: (array: (10pt, 5pt, "dot", 5pt), phase: 10pt))),
        /// )
        /// ```
        #[external]
        dash: Smart<Option<DashPattern>>,

        /// 突出した鋭い折り返しがマイター接合の代わりにベベルによってレンダリングされる数値を指定します。
        /// 数値が高いほどより鋭い角がベベルではなくマイター接合されます。`join`フィールドが`{"miter"}`の場合のみ有効です。
        ///
        /// 厳密には、マイターリミットとはコーナーの突き出たの長さとストロークの太さの比の最大値です。
        ///
        /// `{auto}`に指定された場合、デフォルトの値である`{4.0}`が継承されます。
        ///
        /// ```example
        /// #let items = (
        ///   curve.move((15pt, 0pt)),
        ///   curve.line((0pt, 30pt)),
        ///   curve.line((30pt, 30pt)),
        ///   curve.line((10pt, 20pt)),
        /// )
        ///
        /// #set curve(stroke: 6pt + blue)
        /// #stack(
        ///   dir: ltr,
        ///   spacing: 1cm,
        ///   curve(stroke: (miter-limit: 1), ..items),
        ///   curve(stroke: (miter-limit: 4), ..items),
        ///   curve(stroke: (miter-limit: 5), ..items),
        /// )
        /// ```
        #[external]
        miter_limit: Smart<f64>,
    ) -> SourceResult<Stroke> {
        if let Some(stroke) = args.eat::<Stroke>()? {
            return Ok(stroke);
        }

        fn take<T: FromValue>(args: &mut Args, arg: &str) -> SourceResult<Smart<T>> {
            Ok(args.named::<Smart<T>>(arg)?.unwrap_or(Smart::Auto))
        }

        let paint = take::<Paint>(args, "paint")?;
        let thickness = take::<Length>(args, "thickness")?;
        let cap = take::<LineCap>(args, "cap")?;
        let join = take::<LineJoin>(args, "join")?;
        let dash = take::<Option<DashPattern>>(args, "dash")?;
        let miter_limit = take::<f64>(args, "miter-limit")?.map(Scalar::new);

        Ok(Self { paint, thickness, cap, join, dash, miter_limit })
    }
}

impl<T: Numeric> Stroke<T> {
    /// Map the contained lengths with `f`.
    pub fn map<F, U: Numeric>(self, f: F) -> Stroke<U>
    where
        F: Fn(T) -> U,
    {
        Stroke {
            paint: self.paint,
            thickness: self.thickness.map(&f),
            cap: self.cap,
            join: self.join,
            dash: self.dash.map(|dash| {
                dash.map(|dash| DashPattern {
                    array: dash
                        .array
                        .into_iter()
                        .map(|l| match l {
                            DashLength::Length(v) => DashLength::Length(f(v)),
                            DashLength::LineWidth => DashLength::LineWidth,
                        })
                        .collect(),
                    phase: f(dash.phase),
                })
            }),
            miter_limit: self.miter_limit,
        }
    }
}

impl Stroke<Abs> {
    /// Unpack the stroke, filling missing fields from the `default`.
    pub fn unwrap_or(self, default: FixedStroke) -> FixedStroke {
        let thickness = self.thickness.unwrap_or(default.thickness);
        let dash = self
            .dash
            .map(|dash| {
                dash.map(|dash| DashPattern {
                    array: dash.array.into_iter().map(|l| l.finish(thickness)).collect(),
                    phase: dash.phase,
                })
            })
            .unwrap_or(default.dash);

        FixedStroke {
            paint: self.paint.unwrap_or(default.paint),
            thickness,
            cap: self.cap.unwrap_or(default.cap),
            join: self.join.unwrap_or(default.join),
            dash,
            miter_limit: self.miter_limit.unwrap_or(default.miter_limit),
        }
    }

    /// Unpack the stroke, filling missing fields with the default values.
    pub fn unwrap_or_default(self) -> FixedStroke {
        // we want to do this; the Clippy lint is not type-aware
        #[allow(clippy::unwrap_or_default)]
        self.unwrap_or(FixedStroke::default())
    }
}

impl<T: Numeric + Repr> Repr for Stroke<T> {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::new();
        let Self { paint, thickness, cap, join, dash, miter_limit } = &self;
        if cap.is_auto() && join.is_auto() && dash.is_auto() && miter_limit.is_auto() {
            match (&self.paint, &self.thickness) {
                (Smart::Custom(paint), Smart::Custom(thickness)) => {
                    r.push_str(&thickness.repr());
                    r.push_str(" + ");
                    r.push_str(&paint.repr());
                }
                (Smart::Custom(paint), Smart::Auto) => r.push_str(&paint.repr()),
                (Smart::Auto, Smart::Custom(thickness)) => r.push_str(&thickness.repr()),
                (Smart::Auto, Smart::Auto) => r.push_str("1pt + black"),
            }
        } else {
            r.push('(');
            let mut sep = "";
            if let Smart::Custom(paint) = &paint {
                r.push_str(sep);
                r.push_str("paint: ");
                r.push_str(&paint.repr());
                sep = ", ";
            }
            if let Smart::Custom(thickness) = &thickness {
                r.push_str(sep);
                r.push_str("thickness: ");
                r.push_str(&thickness.repr());
                sep = ", ";
            }
            if let Smart::Custom(cap) = &cap {
                r.push_str(sep);
                r.push_str("cap: ");
                r.push_str(&cap.repr());
                sep = ", ";
            }
            if let Smart::Custom(join) = &join {
                r.push_str(sep);
                r.push_str("join: ");
                r.push_str(&join.repr());
                sep = ", ";
            }
            if let Smart::Custom(dash) = &dash {
                r.push_str(sep);
                r.push_str("dash: ");
                if let Some(dash) = dash {
                    r.push_str(&dash.repr());
                } else {
                    r.push_str(&NoneValue.repr());
                }
                sep = ", ";
            }
            if let Smart::Custom(miter_limit) = &miter_limit {
                r.push_str(sep);
                r.push_str("miter-limit: ");
                r.push_str(&miter_limit.get().repr());
            }
            r.push(')');
        }
        r
    }
}

impl<T: Numeric + Fold> Fold for Stroke<T> {
    fn fold(self, outer: Self) -> Self {
        Self {
            paint: self.paint.or(outer.paint),
            thickness: self.thickness.or(outer.thickness),
            cap: self.cap.or(outer.cap),
            join: self.join.or(outer.join),
            dash: self.dash.or(outer.dash),
            miter_limit: self.miter_limit.or(outer.miter_limit),
        }
    }
}

impl Resolve for Stroke {
    type Output = Stroke<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        Stroke {
            paint: self.paint,
            thickness: self.thickness.resolve(styles),
            cap: self.cap,
            join: self.join,
            dash: self.dash.resolve(styles),
            miter_limit: self.miter_limit,
        }
    }
}

cast! {
    type Stroke,
    thickness: Length => Self {
        thickness: Smart::Custom(thickness),
        ..Default::default()
    },
    color: Color => Self {
        paint: Smart::Custom(color.into()),
        ..Default::default()
    },
    gradient: Gradient => Self {
        paint: Smart::Custom(gradient.into()),
        ..Default::default()
    },
    tiling: Tiling => Self {
        paint: Smart::Custom(tiling.into()),
        ..Default::default()
    },
    mut dict: Dict => {
        // Get a value by key, accepting either Auto or something convertible to type T.
        fn take<T: FromValue>(dict: &mut Dict, key: &str) -> HintedStrResult<Smart<T>> {
            Ok(dict.take(key).ok().map(Smart::<T>::from_value)
                .transpose()?.unwrap_or(Smart::Auto))
        }

        let paint = take::<Paint>(&mut dict, "paint")?;
        let thickness = take::<Length>(&mut dict, "thickness")?;
        let cap = take::<LineCap>(&mut dict, "cap")?;
        let join = take::<LineJoin>(&mut dict, "join")?;
        let dash = take::<Option<DashPattern>>(&mut dict, "dash")?;
        let miter_limit = take::<f64>(&mut dict, "miter-limit")?;
        dict.finish(&["paint", "thickness", "cap", "join", "dash", "miter-limit"])?;

        Self {
            paint,
            thickness,
            cap,
            join,
            dash,
            miter_limit: miter_limit.map(Scalar::new),
        }
    },
}

cast! {
    Stroke<Abs>,
    self => self.map(Length::from).into_value(),
}

/// ストロークのラインキャップ。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum LineCap {
    /// ストロークの終点の端による矩形のキャップ。
    Butt,
    /// ストロークの終点の中心に基づいた円形のキャップ。
    Round,
    /// ストロークの終点の中心に基づいた矩形のキャップ。
    Square,
}

impl Repr for LineCap {
    fn repr(&self) -> EcoString {
        match self {
            Self::Butt => "butt".repr(),
            Self::Round => "round".repr(),
            Self::Square => "square".repr(),
        }
    }
}

/// ストロークの線の接続。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum LineJoin {
    /// 切片は鋭いエッジで接続されます。マイターリミットを超える鋭い折れ曲がりは代わりにベベルによって接続します。
    Miter,
    /// 切片は円形のコーナーで接続されます。
    Round,
    /// 切片はベベル（接続される切片の末端を繋ぐ直線）で接続されます。
    Bevel,
}

impl Repr for LineJoin {
    fn repr(&self) -> EcoString {
        match self {
            Self::Miter => "miter".repr(),
            Self::Round => "round".repr(),
            Self::Bevel => "bevel".repr(),
        }
    }
}

/// 破線パターン。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DashPattern<T: Numeric = Length, DT = DashLength<T>> {
    /// 破線の配列。
    pub array: Vec<DT>,
    /// 破線の開始位置。
    pub phase: T,
}

impl<T: Numeric + Repr, DT: Repr> Repr for DashPattern<T, DT> {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::from("(array: (");
        for (i, elem) in self.array.iter().enumerate() {
            if i != 0 {
                r.push_str(", ")
            }
            r.push_str(&elem.repr())
        }
        r.push_str("), phase: ");
        r.push_str(&self.phase.repr());
        r.push(')');
        r
    }
}

impl<T: Numeric + Default> From<Vec<DashLength<T>>> for DashPattern<T> {
    fn from(array: Vec<DashLength<T>>) -> Self {
        Self { array, phase: T::default() }
    }
}

impl Resolve for DashPattern {
    type Output = DashPattern<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        DashPattern {
            array: self.array.into_iter().map(|l| l.resolve(styles)).collect(),
            phase: self.phase.resolve(styles),
        }
    }
}

// Same names as tikz:
// https://tex.stackexchange.com/questions/45275/tikz-get-values-for-predefined-dash-patterns
cast! {
    DashPattern,
    self => dict! { "array" => self.array, "phase" => self.phase }.into_value(),

    "solid" => Vec::new().into(),
    "dotted" => vec![DashLength::LineWidth, Abs::pt(2.0).into()].into(),
    "densely-dotted" => vec![DashLength::LineWidth, Abs::pt(1.0).into()].into(),
    "loosely-dotted" => vec![DashLength::LineWidth, Abs::pt(4.0).into()].into(),
    "dashed" => vec![Abs::pt(3.0).into(), Abs::pt(3.0).into()].into(),
    "densely-dashed" => vec![Abs::pt(3.0).into(), Abs::pt(2.0).into()].into(),
    "loosely-dashed" => vec![Abs::pt(3.0).into(), Abs::pt(6.0).into()].into(),
    "dash-dotted" => vec![Abs::pt(3.0).into(), Abs::pt(2.0).into(), DashLength::LineWidth, Abs::pt(2.0).into()].into(),
    "densely-dash-dotted" => vec![Abs::pt(3.0).into(), Abs::pt(1.0).into(), DashLength::LineWidth, Abs::pt(1.0).into()].into(),
    "loosely-dash-dotted" => vec![Abs::pt(3.0).into(), Abs::pt(4.0).into(), DashLength::LineWidth, Abs::pt(4.0).into()].into(),

    array: Vec<DashLength> => Self { array, phase: Length::zero() },
    mut dict: Dict => {
        let array: Vec<DashLength> = dict.take("array")?.cast()?;
        let phase = dict.take("phase").ok().map(Value::cast)
            .transpose()?.unwrap_or(Length::zero());
        dict.finish(&["array", "phase"])?;
        Self {
            array,
            phase,
        }
    },
}

/// 破線パターンにおける破線の長さ。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DashLength<T: Numeric = Length> {
    LineWidth,
    Length(T),
}

impl<T: Numeric> DashLength<T> {
    fn finish(self, line_width: T) -> T {
        match self {
            Self::LineWidth => line_width,
            Self::Length(l) => l,
        }
    }
}

impl<T: Numeric + Repr> Repr for DashLength<T> {
    fn repr(&self) -> EcoString {
        match self {
            Self::LineWidth => "dot".repr(),
            Self::Length(v) => v.repr(),
        }
    }
}

impl Resolve for DashLength {
    type Output = DashLength<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        match self {
            Self::LineWidth => DashLength::LineWidth,
            Self::Length(v) => DashLength::Length(v.resolve(styles)),
        }
    }
}

impl From<Abs> for DashLength {
    fn from(l: Abs) -> Self {
        DashLength::Length(l.into())
    }
}

cast! {
    DashLength,
    self => match self {
        Self::LineWidth => "dot".into_value(),
        Self::Length(v) => v.into_value(),
    },
    "dot" => Self::LineWidth,
    v: Length => Self::Length(v),
}

/// 幾何的な形状が完全に指定されたストローク。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FixedStroke {
    /// ストロークの色。
    pub paint: Paint,
    /// ストロークの太さ。
    pub thickness: Abs,
    /// ストロークのラインキャップ。
    pub cap: LineCap,
    /// ストロークの線の接続。
    pub join: LineJoin,
    /// ストロークの破線パターン。
    pub dash: Option<DashPattern<Abs, Abs>>,
    /// マイターリミット。`tiny-skia`と同様にデフォルトは4.0です。
    pub miter_limit: Scalar,
}

impl FixedStroke {
    /// paintとthicknessからストロークを作成します。
    pub fn from_pair(paint: impl Into<Paint>, thickness: Abs) -> Self {
        Self {
            paint: paint.into(),
            thickness,
            ..Default::default()
        }
    }
}

impl Default for FixedStroke {
    fn default() -> Self {
        Self {
            paint: Paint::Solid(Color::BLACK),
            thickness: Abs::pt(1.0),
            cap: LineCap::Butt,
            join: LineJoin::Miter,
            dash: None,
            miter_limit: Scalar::new(4.0),
        }
    }
}
