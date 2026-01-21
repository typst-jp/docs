<<<<<<< HEAD
#![allow(unused)]

use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;

use comemo::{Tracked, TrackedMut};
use typst_syntax::Span;
use typst_utils::LazyHash;

use crate::diag::SourceResult;
use crate::engine::{Engine, Route, Sink, Traced};
use crate::foundations::{
    Args, Cast, Closure, Content, Context, Func, Packed, Scope, StyleChain, Styles, Value,
};
use crate::introspection::{Introspector, Locator, SplitLocator};
use crate::layout::{
    Abs, BoxElem, ColumnsElem, Fragment, Frame, GridElem, InlineItem, MoveElem, PadElem,
    PagedDocument, Region, Regions, Rel, RepeatElem, RotateElem, ScaleElem, Size,
    SkewElem, StackElem,
};
use crate::math::EquationElem;
use crate::model::{DocumentInfo, EnumElem, ListElem, TableElem};
use crate::visualize::{
    CircleElem, CurveElem, EllipseElem, ImageElem, LineElem, PathElem, PolygonElem,
    RectElem, SquareElem,
};
use crate::World;
=======
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

use comemo::{Tracked, TrackedMut};
use typst_syntax::{Span, SyntaxMode};
use typst_utils::LazyHash;

use crate::World;
use crate::diag::SourceResult;
use crate::engine::{Engine, Route, Sink, Traced};
use crate::foundations::{
    Args, Closure, Content, Context, Func, Module, NativeRuleMap, Scope, StyleChain,
    Styles, Value,
};
use crate::introspection::{Introspector, Locator, SplitLocator};
use crate::layout::{Frame, Region};
use crate::model::DocumentInfo;
use crate::visualize::Color;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

/// Defines the `Routines` struct.
macro_rules! routines {
    ($(
        $(#[$attr:meta])*
        fn $name:ident $(<$($time:lifetime),*>)? ($($args:tt)*) -> $ret:ty
    )*) => {
        /// Defines implementation of various Typst compiler routines as a table
        /// of function pointers.
        ///
        /// This is essentially dynamic linking and done to allow for crate
        /// splitting.
        pub struct Routines {
<<<<<<< HEAD
=======
            /// Native show rules.
            pub rules: NativeRuleMap,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            $(
                $(#[$attr])*
                pub $name: $(for<$($time),*>)? fn ($($args)*) -> $ret
            ),*
        }

        impl Hash for Routines {
            fn hash<H: Hasher>(&self, _: &mut H) {}
        }
<<<<<<< HEAD
=======

        impl Debug for Routines {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.pad("Routines(..)")
            }
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    };
}

routines! {
    /// Evaluates a string as code and return the resulting value.
    fn eval_string(
        routines: &Routines,
        world: Tracked<dyn World + '_>,
<<<<<<< HEAD
        string: &str,
        span: Span,
        mode: EvalMode,
=======
        sink: TrackedMut<Sink>,
        string: &str,
        span: Span,
        mode: SyntaxMode,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        scope: Scope,
    ) -> SourceResult<Value>

    /// Call the closure in the context with the arguments.
    fn eval_closure(
        func: &Func,
        closure: &LazyHash<Closure>,
        routines: &Routines,
        world: Tracked<dyn World + '_>,
        introspector: Tracked<Introspector>,
        traced: Tracked<Traced>,
        sink: TrackedMut<Sink>,
        route: Tracked<Route>,
        context: Tracked<Context>,
        args: Args,
    ) -> SourceResult<Value>

    /// Realizes content into a flat list of well-known, styled items.
    fn realize<'a>(
        kind: RealizationKind,
        engine: &mut Engine,
        locator: &mut SplitLocator,
        arenas: &'a Arenas,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<Vec<Pair<'a>>>

<<<<<<< HEAD
    /// Lays out content into multiple regions.
    fn layout_fragment(
        engine: &mut Engine,
        content: &Content,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// Lays out content into a single region, producing a single frame.
    fn layout_frame(
        engine: &mut Engine,
        content: &Content,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

<<<<<<< HEAD
    /// Lays out a [`ListElem`].
    fn layout_list(
        elem: &Packed<ListElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out an [`EnumElem`].
    fn layout_enum(
        elem: &Packed<EnumElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out a [`GridElem`].
    fn layout_grid(
        elem: &Packed<GridElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out a [`TableElem`].
    fn layout_table(
        elem: &Packed<TableElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out a [`StackElem`].
    fn layout_stack(
        elem: &Packed<StackElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out a [`ColumnsElem`].
    fn layout_columns(
        elem: &Packed<ColumnsElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out a [`MoveElem`].
    fn layout_move(
        elem: &Packed<MoveElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`RotateElem`].
    fn layout_rotate(
        elem: &Packed<RotateElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`ScaleElem`].
    fn layout_scale(
        elem: &Packed<ScaleElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`SkewElem`].
    fn layout_skew(
        elem: &Packed<SkewElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`RepeatElem`].
    fn layout_repeat(
        elem: &Packed<RepeatElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`PadElem`].
    fn layout_pad(
        elem: &Packed<PadElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>

    /// Lays out a [`LineElem`].
    fn layout_line(
        elem: &Packed<LineElem>,
        _: &mut Engine,
        _: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`CurveElem`].
    fn layout_curve(
        elem: &Packed<CurveElem>,
        _: &mut Engine,
        _: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`PathElem`].
    fn layout_path(
        elem: &Packed<PathElem>,
        _: &mut Engine,
        _: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`PolygonElem`].
    fn layout_polygon(
        elem: &Packed<PolygonElem>,
        _: &mut Engine,
        _: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`RectElem`].
    fn layout_rect(
        elem: &Packed<RectElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`SquareElem`].
    fn layout_square(
        elem: &Packed<SquareElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`EllipseElem`].
    fn layout_ellipse(
        elem: &Packed<EllipseElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out a [`CircleElem`].
    fn layout_circle(
        elem: &Packed<CircleElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out an [`ImageElem`].
    fn layout_image(
        elem: &Packed<ImageElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Region,
    ) -> SourceResult<Frame>

    /// Lays out an [`EquationElem`] in a paragraph.
    fn layout_equation_inline(
        elem: &Packed<EquationElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        region: Size,
    ) -> SourceResult<Vec<InlineItem>>

    /// Lays out an [`EquationElem`] in a flow.
    fn layout_equation_block(
        elem: &Packed<EquationElem>,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>
}

/// In which mode to evaluate a string.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum EvalMode {
    /// Evaluate as code, as after a hash.
    Code,
    /// Evaluate as markup, like in a Typst file.
    Markup,
    /// Evaluate as math, as in an equation.
    Math,
=======
    /// Constructs the `html` module.
    fn html_module() -> Module

    /// Wraps content in a span with a color.
    ///
    /// This is a temporary workaround until `TextElem::fill` is supported in
    /// HTML export.
    fn html_span_filled(content: Content, color: Color) -> Content
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Defines what kind of realization we are performing.
pub enum RealizationKind<'a> {
    /// This the root realization for layout. Requires a mutable reference
    /// to document metadata that will be filled from `set document` rules.
<<<<<<< HEAD
    LayoutDocument(&'a mut DocumentInfo),
    /// A nested realization in a container (e.g. a `block`). Requires a mutable
    /// reference to an enum that will be set to `FragmentKind::Inline` if the
    /// fragment's content was fully inline.
    LayoutFragment(&'a mut FragmentKind),
    /// A nested realization in a paragraph (i.e. a `par`)
    LayoutPar,
    /// This the root realization for HTML. Requires a mutable reference
    /// to document metadata that will be filled from `set document` rules.
    HtmlDocument(&'a mut DocumentInfo),
    /// A nested realization in a container (e.g. a `block`). Requires a mutable
    /// reference to an enum that will be set to `FragmentKind::Inline` if the
    /// fragment's content was fully inline.
    HtmlFragment(&'a mut FragmentKind),
=======
    LayoutDocument { info: &'a mut DocumentInfo },
    /// A nested realization in a container (e.g. a `block`). Requires a mutable
    /// reference to an enum that will be set to `FragmentKind::Inline` if the
    /// fragment's content was fully inline.
    LayoutFragment { kind: &'a mut FragmentKind },
    /// A nested realization in a paragraph (i.e. a `par`)
    LayoutPar,
    /// This the root realization for HTML. Requires a mutable reference to
    /// document metadata that will be filled from `set document` rules.
    ///
    /// The `is_inline` function checks whether content consists of an inline
    /// HTML element. It's used by the `PAR` grouping rules. This is slightly
    /// hacky and might be replaced by a mechanism to supply the grouping rules
    /// as a realization user.
    HtmlDocument { info: &'a mut DocumentInfo, is_inline: fn(&Content) -> bool },
    /// A nested realization in a container (e.g. a `block`). Requires a mutable
    /// reference to an enum that will be set to `FragmentKind::Inline` if the
    /// fragment's content was fully inline.
    HtmlFragment { kind: &'a mut FragmentKind, is_inline: fn(&Content) -> bool },
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// A realization within math.
    Math,
}

impl RealizationKind<'_> {
    /// It this a realization for HTML export?
    pub fn is_html(&self) -> bool {
<<<<<<< HEAD
        matches!(self, Self::HtmlDocument(_) | Self::HtmlFragment(_))
=======
        matches!(self, Self::HtmlDocument { .. } | Self::HtmlFragment { .. })
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    /// It this a realization for a container?
    pub fn is_fragment(&self) -> bool {
<<<<<<< HEAD
        matches!(self, Self::LayoutFragment(_) | Self::HtmlFragment(_))
=======
        matches!(self, Self::LayoutFragment { .. } | Self::HtmlFragment { .. })
    }

    /// It this a realization for the whole document?
    pub fn is_document(&self) -> bool {
        matches!(self, Self::LayoutDocument { .. } | Self::HtmlDocument { .. })
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    /// If this is a document-level realization, accesses the document info.
    pub fn as_document_mut(&mut self) -> Option<&mut DocumentInfo> {
        match self {
<<<<<<< HEAD
            Self::LayoutDocument(info) | Self::HtmlDocument(info) => Some(*info),
=======
            Self::LayoutDocument { info } | Self::HtmlDocument { info, .. } => {
                Some(*info)
            }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            _ => None,
        }
    }

    /// If this is a container-level realization, accesses the fragment kind.
    pub fn as_fragment_mut(&mut self) -> Option<&mut FragmentKind> {
        match self {
<<<<<<< HEAD
            Self::LayoutFragment(kind) | Self::HtmlFragment(kind) => Some(*kind),
=======
            Self::LayoutFragment { kind } | Self::HtmlFragment { kind, .. } => {
                Some(*kind)
            }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            _ => None,
        }
    }
}

/// The kind of fragment output that realization produced.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FragmentKind {
    /// The fragment's contents were fully inline, and as a result, the output
    /// elements are too.
    Inline,
    /// The fragment contained non-inline content, so inline content was forced
    /// into paragraphs, and as a result, the output elements are not inline.
    Block,
}

/// Temporary storage arenas for lifetime extension during realization.
///
/// Must be kept live while the content returned from realization is processed.
#[derive(Default)]
pub struct Arenas {
    /// A typed arena for owned content.
    pub content: typed_arena::Arena<Content>,
    /// A typed arena for owned styles.
    pub styles: typed_arena::Arena<Styles>,
    /// An untyped arena for everything that is `Copy`.
    pub bump: bumpalo::Bump,
}

/// A pair of content and a style chain that applies to it.
pub type Pair<'a> = (&'a Content, StyleChain<'a>);
