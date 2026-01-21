<<<<<<< HEAD
use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, NativeElement, Packed, Show, StyleChain};
use crate::layout::{Abs, Angle, Axes, BlockElem, Length, Rel};
=======
use crate::foundations::elem;
use crate::layout::{Abs, Angle, Axes, Length, Rel};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use crate::visualize::Stroke;

/// A line from one point to another.
///
/// # Example
/// ```example
/// #set page(height: 100pt)
///
/// #line(length: 100%)
/// #line(end: (50%, 50%))
/// #line(
///   length: 4cm,
///   stroke: 2pt + maroon,
/// )
/// ```
<<<<<<< HEAD
#[elem(Show)]
=======
#[elem]
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub struct LineElem {
    /// The start point of the line.
    ///
    /// Must be an array of exactly two relative lengths.
<<<<<<< HEAD
    #[resolve]
    pub start: Axes<Rel<Length>>,

    /// The point where the line ends.
    #[resolve]
    pub end: Option<Axes<Rel<Length>>>,

    /// The line's length. This is only respected if `end` is `{none}`.
    #[resolve]
=======
    pub start: Axes<Rel<Length>>,

    /// The point where the line ends.
    pub end: Option<Axes<Rel<Length>>>,

    /// The line's length. This is only respected if `end` is `{none}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[default(Abs::pt(30.0).into())]
    pub length: Rel<Length>,

    /// The angle at which the line points away from the origin. This is only
    /// respected if `end` is `{none}`.
    pub angle: Angle,

    /// How to [stroke] the line.
    ///
    /// ```example
    /// #set line(length: 100%)
    /// #stack(
    ///   spacing: 1em,
    ///   line(stroke: 2pt + red),
    ///   line(stroke: (paint: blue, thickness: 4pt, cap: "round")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: "dashed")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: ("dot", 2pt, 4pt, 2pt))),
    /// )
    /// ```
<<<<<<< HEAD
    #[resolve]
    #[fold]
    pub stroke: Stroke,
}

impl Show for Packed<LineElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_line)
            .pack()
            .spanned(self.span()))
    }
}
=======
    #[fold]
    pub stroke: Stroke,
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
