<<<<<<< HEAD
use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, Content, NativeElement, Packed, Show, Smart, StyleChain,
};
use crate::layout::{
    Abs, Alignment, Angle, BlockElem, HAlignment, Length, Ratio, Rel, VAlignment,
};

/// レイアウトに影響を与えないコンテンツの移動。
///
/// `move`関数を用いると、コンテンツの元々の位置をレイアウトに「認識」させながら、そのコンテンツを移動させることができます。
/// コンテナの大きさは、コンテンツが移動されていないかのように決定されます。
///
/// # 例
=======
use crate::foundations::{Content, Smart, cast, elem};
use crate::layout::{Abs, Alignment, Angle, HAlignment, Length, Ratio, Rel, VAlignment};

/// Moves content without affecting layout.
///
/// The `move` function allows you to move content while the layout still 'sees'
/// it at the original positions. Containers will still be sized as if the
/// content was not moved.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #rect(inset: 0pt, move(
///   dx: 6pt, dy: 6pt,
///   rect(
///     inset: 8pt,
///     fill: white,
///     stroke: black,
///     [Abra cadabra]
///   )
/// ))
/// ```
<<<<<<< HEAD
#[elem(Show)]
pub struct MoveElem {
    /// コンテンツの水平方向の変位。
    pub dx: Rel<Length>,

    /// コンテンツの垂直方向の変位。
    pub dy: Rel<Length>,

    /// 移動させたいコンテンツ。
=======
///
/// # Accessibility
/// Moving is transparent to Assistive Technology (AT). Your content will be
/// read in the order it appears in the source, regardless of any visual
/// movement. If you need to hide content from AT altogether in PDF export,
/// consider using [`pdf.artifact`].
#[elem]
pub struct MoveElem {
    /// The horizontal displacement of the content.
    pub dx: Rel<Length>,

    /// The vertical displacement of the content.
    pub dy: Rel<Length>,

    /// The content to move.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
impl Show for Packed<MoveElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_move)
            .pack()
            .spanned(self.span()))
    }
}

/// レイアウトに影響を与えないコンテンツの回転。
///
/// 要素を指定された角度だけ回転させます。
/// `{reflow: true}`を指定しない限り、レイアウトは、その要素が回転していないかのように振る舞います。
///
/// # 例
=======
/// Rotates content without affecting layout.
///
/// Rotates an element by a given angle. The layout will act as if the element
/// was not rotated unless you specify `{reflow: true}`.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #stack(
///   dir: ltr,
///   spacing: 1fr,
///   ..range(16)
///     .map(i => rotate(24deg * i)[X]),
/// )
/// ```
<<<<<<< HEAD
#[elem(Show)]
pub struct RotateElem {
    /// 回転させる量。
=======
#[elem]
pub struct RotateElem {
    /// The amount of rotation.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #rotate(-1.571rad)[Space!]
    /// ```
<<<<<<< HEAD
    ///
    #[positional]
    pub angle: Angle,

    /// 回転の中心点。
    ///
    /// 例えば、回転した要素の左下隅をベースラインに揃えたままにしたい場合、代わりに`bottom + left`を指定します。
=======
    #[positional]
    pub angle: Angle,

    /// The origin of the rotation.
    ///
    /// If, for instance, you wanted the bottom left corner of the rotated
    /// element to stay aligned with the baseline, you would set it to `bottom +
    /// left` instead.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set text(spacing: 8pt)
    /// #let square = square.with(width: 8pt)
    ///
    /// #box(square())
    /// #box(rotate(30deg, origin: center, square()))
    /// #box(rotate(30deg, origin: top + left, square()))
    /// #box(rotate(30deg, origin: bottom + right, square()))
    /// ```
    #[fold]
    #[default(HAlignment::Center + VAlignment::Horizon)]
    pub origin: Alignment,

<<<<<<< HEAD
    /// 回転がレイアウトに影響を与えるかどうか。
    ///
    /// `{false}`に設定された場合、回転したコンテンツは元々のコンテンツのバウンディングボックスに留まります。
    /// `{true}`に設定された場合、バウンディングボックスはコンテンツの回転を考慮してレイアウトを調整します。
=======
    /// Whether the rotation impacts the layout.
    ///
    /// If set to `{false}`, the rotated content will retain the bounding box of
    /// the original content. If set to `{true}`, the bounding box will take the
    /// rotation of the content into account and adjust the layout accordingly.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// Hello #rotate(90deg, reflow: true)[World]!
    /// ```
    #[default(false)]
    pub reflow: bool,

<<<<<<< HEAD
    /// 回転させるコンテンツ。
=======
    /// The content to rotate.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
impl Show for Packed<RotateElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_rotate)
            .pack()
            .spanned(self.span()))
    }
}

/// レイアウトに影響を与えないコンテンツの拡大縮小。
///
/// 単一の軸で負のスケールを指定することで、コンテンツを反転表示できます。
///
/// # 例
=======
/// Scales content without affecting layout.
///
/// Lets you mirror content by specifying a negative scale on a single axis.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #set align(center)
/// #scale(x: -100%)[This is mirrored.]
/// #scale(x: -100%, reflow: true)[This is mirrored.]
/// ```
<<<<<<< HEAD
#[elem(Show)]
pub struct ScaleElem {
    /// 位置引数として両方の軸の拡大縮小率を設定します。
    /// これは`x`と`y`を同じ値で設定する省略記法です。
=======
#[elem]
pub struct ScaleElem {
    /// The scaling factor for both axes, as a positional argument. This is just
    /// an optional shorthand notation for setting `x` and `y` to the same
    /// value.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[external]
    #[positional]
    #[default(Smart::Custom(ScaleAmount::Ratio(Ratio::one())))]
    pub factor: Smart<ScaleAmount>,

<<<<<<< HEAD
    /// 水平方向の拡大縮小率。
    ///
    /// 負の値が指定された場合は本文が水平方向に反転します。
=======
    /// The horizontal scaling factor.
    ///
    /// The body will be mirrored horizontally if the parameter is negative.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[parse(
        let all = args.find()?;
        args.named("x")?.or(all)
    )]
    #[default(Smart::Custom(ScaleAmount::Ratio(Ratio::one())))]
    pub x: Smart<ScaleAmount>,

<<<<<<< HEAD
    /// 垂直方向の拡大縮小率。
    ///
    /// 負の値が指定された場合は本文が垂直方向に反転します。
=======
    /// The vertical scaling factor.
    ///
    /// The body will be mirrored vertically if the parameter is negative.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[parse(args.named("y")?.or(all))]
    #[default(Smart::Custom(ScaleAmount::Ratio(Ratio::one())))]
    pub y: Smart<ScaleAmount>,

<<<<<<< HEAD
    /// 変換の原点。
=======
    /// The origin of the transformation.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// A#box(scale(75%)[A])A \
    /// B#box(scale(75%, origin: bottom + left)[B])B
    /// ```
    #[fold]
    #[default(HAlignment::Center + VAlignment::Horizon)]
    pub origin: Alignment,

<<<<<<< HEAD
    /// 拡大縮小がレイアウトに影響を与えるかどうか。
    ///
    /// `{false}`の場合、拡大縮小したコンテンツが他のコンテンツと重なることを許可します。
    /// `{true}`の場合、拡大縮小したコンテンツの新しい大きさを計算し、それに応じてレイアウトを調整します。
=======
    /// Whether the scaling impacts the layout.
    ///
    /// If set to `{false}`, the scaled content will be allowed to overlap
    /// other content. If set to `{true}`, it will compute the new size of
    /// the scaled content and adjust the layout accordingly.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// Hello #scale(x: 20%, y: 40%, reflow: true)[World]!
    /// ```
    #[default(false)]
    pub reflow: bool,

<<<<<<< HEAD
    /// 拡大縮小するコンテンツ。
=======
    /// The content to scale.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
impl Show for Packed<ScaleElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_scale)
            .pack()
            .spanned(self.span()))
    }
}

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// To what size something shall be scaled.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScaleAmount {
    Ratio(Ratio),
    Length(Length),
}

cast! {
    ScaleAmount,
    self => match self {
        ScaleAmount::Ratio(ratio) => ratio.into_value(),
        ScaleAmount::Length(length) => length.into_value(),
    },
    ratio: Ratio => ScaleAmount::Ratio(ratio),
    length: Length => ScaleAmount::Length(length),
}

<<<<<<< HEAD
/// コンテンツのスキュー変形。
///
/// 水平方向または垂直方向、あるいは両方向に要素をスキュー（シアー）変形します。
/// `{reflow: true}`を指定しない限り、レイアウトは要素がスキュー変形を受けていないかのように振る舞います。
///
/// # 例
=======
/// Skews content.
///
/// Skews an element in horizontal and/or vertical direction. The layout will
/// act as if the element was not skewed unless you specify `{reflow: true}`.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #skew(ax: -12deg)[
///   This is some fake italic text.
/// ]
/// ```
<<<<<<< HEAD
#[elem(Show)]
pub struct SkewElem {
    /// 水平方向のスキュー角。
=======
#[elem]
pub struct SkewElem {
    /// The horizontal skewing angle.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #skew(ax: 30deg)[Skewed]
    /// ```
<<<<<<< HEAD
    ///
    #[default(Angle::zero())]
    pub ax: Angle,

    /// 垂直方向のスキュー角。
=======
    #[default(Angle::zero())]
    pub ax: Angle,

    /// The vertical skewing angle.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #skew(ay: 30deg)[Skewed]
    /// ```
<<<<<<< HEAD
    ///
    #[default(Angle::zero())]
    pub ay: Angle,

    /// スキュー変形の原点。
    ///
    /// 操作中は原点が固定されます。
=======
    #[default(Angle::zero())]
    pub ay: Angle,

    /// The origin of the skew transformation.
    ///
    /// The origin will stay fixed during the operation.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// X #box(skew(ax: -30deg, origin: center + horizon)[X]) X \
    /// X #box(skew(ax: -30deg, origin: bottom + left)[X]) X \
    /// X #box(skew(ax: -30deg, origin: top + right)[X]) X
    /// ```
    #[fold]
    #[default(HAlignment::Center + VAlignment::Horizon)]
    pub origin: Alignment,

<<<<<<< HEAD
    /// スキュー変形がレイアウトに影響を与えるかどうか。
    ///
    /// `{false}`の場合、スキュー変形されたコンテンツは元々のコンテンツのバウンディングボックスに留まります。
    /// `{true}`の場合、バウンディングボックスはコンテンツの変形を考慮してレイアウトを調整します。
=======
    /// Whether the skew transformation impacts the layout.
    ///
    /// If set to `{false}`, the skewed content will retain the bounding box of
    /// the original content. If set to `{true}`, the bounding box will take the
    /// transformation of the content into account and adjust the layout accordingly.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// Hello #skew(ay: 30deg, reflow: true, "World")!
    /// ```
    #[default(false)]
    pub reflow: bool,

<<<<<<< HEAD
    /// スキュー変形するコンテンツ。
=======
    /// The content to skew.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
impl Show for Packed<SkewElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_skew)
            .pack()
            .spanned(self.span()))
    }
}

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// A scale-skew-translate transformation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Transform {
    pub sx: Ratio,
    pub ky: Ratio,
    pub kx: Ratio,
    pub sy: Ratio,
    pub tx: Abs,
    pub ty: Abs,
}

impl Transform {
    /// The identity transformation.
    pub const fn identity() -> Self {
        Self {
            sx: Ratio::one(),
            ky: Ratio::zero(),
            kx: Ratio::zero(),
            sy: Ratio::one(),
            tx: Abs::zero(),
            ty: Abs::zero(),
        }
    }

    /// A translate transform.
    pub const fn translate(tx: Abs, ty: Abs) -> Self {
        Self { tx, ty, ..Self::identity() }
    }

    /// A scale transform.
    pub const fn scale(sx: Ratio, sy: Ratio) -> Self {
        Self { sx, sy, ..Self::identity() }
    }

<<<<<<< HEAD
=======
    /// A scale transform at a specific position.
    pub fn scale_at(sx: Ratio, sy: Ratio, px: Abs, py: Abs) -> Self {
        Self::translate(px, py)
            .pre_concat(Self::scale(sx, sy))
            .pre_concat(Self::translate(-px, -py))
    }

    /// A rotate transform at a specific position.
    pub fn rotate_at(angle: Angle, px: Abs, py: Abs) -> Self {
        Self::translate(px, py)
            .pre_concat(Self::rotate(angle))
            .pre_concat(Self::translate(-px, -py))
    }

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// A rotate transform.
    pub fn rotate(angle: Angle) -> Self {
        let cos = Ratio::new(angle.cos());
        let sin = Ratio::new(angle.sin());
        Self {
            sx: cos,
            ky: sin,
            kx: -sin,
            sy: cos,
            ..Self::default()
        }
    }

    /// A skew transform.
    pub fn skew(ax: Angle, ay: Angle) -> Self {
        Self {
            kx: Ratio::new(ax.tan()),
            ky: Ratio::new(ay.tan()),
            ..Self::identity()
        }
    }

    /// Whether this is the identity transformation.
    pub fn is_identity(self) -> bool {
        self == Self::identity()
    }

    /// Pre-concatenate another transformation.
    pub fn pre_concat(self, prev: Self) -> Self {
        Transform {
            sx: self.sx * prev.sx + self.kx * prev.ky,
            ky: self.ky * prev.sx + self.sy * prev.ky,
            kx: self.sx * prev.kx + self.kx * prev.sy,
            sy: self.ky * prev.kx + self.sy * prev.sy,
            tx: self.sx.of(prev.tx) + self.kx.of(prev.ty) + self.tx,
            ty: self.ky.of(prev.tx) + self.sy.of(prev.ty) + self.ty,
        }
    }

    /// Post-concatenate another transformation.
    pub fn post_concat(self, next: Self) -> Self {
        next.pre_concat(self)
    }

    /// Inverts the transformation.
    ///
    /// Returns `None` if the determinant of the matrix is zero.
    pub fn invert(self) -> Option<Self> {
        // Allow the trivial case to be inlined.
        if self.is_identity() {
            return Some(self);
        }

        // Fast path for scale-translate-only transforms.
        if self.kx.is_zero() && self.ky.is_zero() {
            if self.sx.is_zero() || self.sy.is_zero() {
                return Some(Self::translate(-self.tx, -self.ty));
            }

            let inv_x = 1.0 / self.sx;
            let inv_y = 1.0 / self.sy;
            return Some(Self {
                sx: Ratio::new(inv_x),
                ky: Ratio::zero(),
                kx: Ratio::zero(),
                sy: Ratio::new(inv_y),
                tx: -self.tx * inv_x,
                ty: -self.ty * inv_y,
            });
        }

        let det = self.sx * self.sy - self.kx * self.ky;
        if det.get().abs() < 1e-12 {
            return None;
        }

        let inv_det = 1.0 / det;
        Some(Self {
            sx: (self.sy * inv_det),
            ky: (-self.ky * inv_det),
            kx: (-self.kx * inv_det),
            sy: (self.sx * inv_det),
            tx: Abs::pt(
                (self.kx.get() * self.ty.to_pt() - self.sy.get() * self.tx.to_pt())
                    * inv_det,
            ),
            ty: Abs::pt(
                (self.ky.get() * self.tx.to_pt() - self.sx.get() * self.ty.to_pt())
                    * inv_det,
            ),
        })
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
