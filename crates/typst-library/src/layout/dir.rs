use ecow::EcoString;

<<<<<<< HEAD
use crate::foundations::{func, scope, ty, Repr};
use crate::layout::{Axis, Side};

/// コンテンツをレイアウトできる4つの向き。
///
///  取りうる値は以下の通りです。
/// - `{ltr}`: 左から右。
/// - `{rtl}`: 右から左。
/// - `{ttb}`: 上から下。
/// - `{btt}`: 下から上。
///
/// これらの値はグローバルスコープでも、direction型のスコープでも用いることができます。
/// したがって、以下の2つのどちらでも書くことができます。
=======
use crate::foundations::{Repr, func, scope, ty};
use crate::layout::{Axis, Side};

/// The four directions into which content can be laid out.
///
///  Possible values are:
/// - `{ltr}`: Left to right.
/// - `{rtl}`: Right to left.
/// - `{ttb}`: Top to bottom.
/// - `{btt}`: Bottom to top.
///
/// These values are available globally and
/// also in the direction type's scope, so you can write either of the following
/// two:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #stack(dir: rtl)[A][B][C]
/// #stack(dir: direction.rtl)[A][B][C]
/// ```
#[ty(scope, name = "direction")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Dir {
    /// Left to right.
    LTR,
    /// Right to left.
    RTL,
    /// Top to bottom.
    TTB,
    /// Bottom to top.
    BTT,
}

impl Dir {
    /// Whether this direction points into the positive coordinate direction.
    ///
    /// The positive directions are left-to-right and top-to-bottom.
    pub const fn is_positive(self) -> bool {
        match self {
            Self::LTR | Self::TTB => true,
            Self::RTL | Self::BTT => false,
        }
    }
}

#[scope]
impl Dir {
    pub const LTR: Self = Self::LTR;
    pub const RTL: Self = Self::RTL;
    pub const TTB: Self = Self::TTB;
    pub const BTT: Self = Self::BTT;

<<<<<<< HEAD
    /// このdirectionが属する軸。`{"horizontal"}`か`{"vertical"}`のいずれかになります。
=======
    /// Returns a direction from a starting point.
    ///
    /// ```example
    /// #direction.from(left) \
    /// #direction.from(right) \
    /// #direction.from(top) \
    /// #direction.from(bottom)
    /// ```
    #[func]
    pub const fn from(side: Side) -> Dir {
        match side {
            Side::Left => Self::LTR,
            Side::Right => Self::RTL,
            Side::Top => Self::TTB,
            Side::Bottom => Self::BTT,
        }
    }

    /// Returns a direction from an end point.
    ///
    /// ```example
    /// #direction.to(left) \
    /// #direction.to(right) \
    /// #direction.to(top) \
    /// #direction.to(bottom)
    /// ```
    #[func]
    pub const fn to(side: Side) -> Dir {
        match side {
            Side::Right => Self::LTR,
            Side::Left => Self::RTL,
            Side::Bottom => Self::TTB,
            Side::Top => Self::BTT,
        }
    }

    /// The axis this direction belongs to, either `{"horizontal"}` or
    /// `{"vertical"}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #ltr.axis() \
    /// #ttb.axis()
    /// ```
    #[func]
    pub const fn axis(self) -> Axis {
        match self {
            Self::LTR | Self::RTL => Axis::X,
            Self::TTB | Self::BTT => Axis::Y,
        }
    }

<<<<<<< HEAD
    /// このdirectionの始点をalignmentとして返します。
=======
    /// The corresponding sign, for use in calculations.
    ///
    /// ```example
    /// #ltr.sign() \
    /// #rtl.sign() \
    /// #ttb.sign() \
    /// #btt.sign()
    /// ```
    #[func]
    pub const fn sign(self) -> i64 {
        match self {
            Self::LTR | Self::TTB => 1,
            Self::RTL | Self::BTT => -1,
        }
    }

    /// The start point of this direction, as an alignment.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #ltr.start() \
    /// #rtl.start() \
    /// #ttb.start() \
    /// #btt.start()
    /// ```
    #[func]
    pub const fn start(self) -> Side {
        match self {
            Self::LTR => Side::Left,
            Self::RTL => Side::Right,
            Self::TTB => Side::Top,
            Self::BTT => Side::Bottom,
        }
    }

<<<<<<< HEAD
    /// このdirectionの終点をalignmentとして返します。
=======
    /// The end point of this direction, as an alignment.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #ltr.end() \
    /// #rtl.end() \
    /// #ttb.end() \
    /// #btt.end()
    /// ```
    #[func]
    pub const fn end(self) -> Side {
        match self {
            Self::LTR => Side::Right,
            Self::RTL => Side::Left,
            Self::TTB => Side::Bottom,
            Self::BTT => Side::Top,
        }
    }

<<<<<<< HEAD
    /// 逆の向き。
=======
    /// The inverse direction.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #ltr.inv() \
    /// #rtl.inv() \
    /// #ttb.inv() \
    /// #btt.inv()
    /// ```
    #[func(title = "Inverse")]
    pub const fn inv(self) -> Dir {
        match self {
            Self::LTR => Self::RTL,
            Self::RTL => Self::LTR,
            Self::TTB => Self::BTT,
            Self::BTT => Self::TTB,
        }
    }
}

impl Repr for Dir {
    fn repr(&self) -> EcoString {
        match self {
            Self::LTR => "ltr".into(),
            Self::RTL => "rtl".into(),
            Self::TTB => "ttb".into(),
            Self::BTT => "btt".into(),
        }
    }
}
