//! Drawing and visualization.

mod color;
mod curve;
mod gradient;
mod image;
mod line;
mod paint;
mod path;
mod polygon;
mod shape;
mod stroke;
mod tiling;

pub use self::color::*;
pub use self::curve::*;
pub use self::gradient::*;
pub use self::image::*;
pub use self::line::*;
pub use self::paint::*;
pub use self::path::*;
pub use self::polygon::*;
pub use self::shape::*;
pub use self::stroke::*;
pub use self::tiling::*;

<<<<<<< HEAD
=======
use crate::foundations::Deprecation;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use crate::foundations::{Element, Scope, Type};

/// Hook up all visualize definitions.
pub(super) fn define(global: &mut Scope) {
    global.start_category(crate::Category::Visualize);
    global.define_type::<Color>();
    global.define_type::<Gradient>();
    global.define_type::<Tiling>();
    global.define_type::<Stroke>();
    global.define_elem::<ImageElem>();
    global.define_elem::<LineElem>();
    global.define_elem::<RectElem>();
    global.define_elem::<SquareElem>();
    global.define_elem::<EllipseElem>();
    global.define_elem::<CircleElem>();
    global.define_elem::<PolygonElem>();
    global.define_elem::<CurveElem>();
<<<<<<< HEAD
    global
        .define("path", Element::of::<PathElem>())
        .deprecated("the `path` function is deprecated, use `curve` instead");
    global
        .define("pattern", Type::of::<Tiling>())
        .deprecated("the name `pattern` is deprecated, use `tiling` instead");
=======
    global.define("path", Element::of::<PathElem>()).deprecated(
        Deprecation::new()
            .with_message("the `path` function is deprecated, use `curve` instead"),
    );
    global.define("pattern", Type::of::<Tiling>()).deprecated(
        Deprecation::new()
            .with_message("the name `pattern` is deprecated, use `tiling` instead")
            .with_until("0.15.0"),
    );
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    global.reset_category();
}
