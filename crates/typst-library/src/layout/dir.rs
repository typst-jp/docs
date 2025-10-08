use ecow::EcoString;

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

    /// このdirectionが属する軸。`{"horizontal"}`か`{"vertical"}`のいずれかになります。
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

    /// このdirectionの始点をalignmentとして返します。
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

    /// このdirectionの終点をalignmentとして返します。
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

    /// 逆の向き。
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
