use comemo::Track;
use smallvec::smallvec;
use typst_library::diag::SourceResult;
use typst_library::engine::Engine;
use typst_library::foundations::{Content, Context, Depth, Packed, StyleChain};
use typst_library::introspection::Locator;
use typst_library::layout::grid::resolve::{Cell, CellGrid};
use typst_library::layout::{Axes, Fragment, HAlignment, Regions, Sizing, VAlignment};
use typst_library::model::{EnumElem, ListElem, Numbering, ParElem, ParbreakElem};
<<<<<<< HEAD
=======
use typst_library::pdf::PdfMarkerTag;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use typst_library::text::TextElem;

use crate::grid::GridLayouter;

/// Layout the list.
#[typst_macros::time(span = elem.span())]
pub fn layout_list(
    elem: &Packed<ListElem>,
    engine: &mut Engine,
    locator: Locator,
    styles: StyleChain,
    regions: Regions,
) -> SourceResult<Fragment> {
<<<<<<< HEAD
    let indent = elem.indent(styles);
    let body_indent = elem.body_indent(styles);
    let tight = elem.tight(styles);
    let gutter = elem.spacing(styles).unwrap_or_else(|| {
        if tight {
            ParElem::leading_in(styles).into()
        } else {
            ParElem::spacing_in(styles).into()
        }
    });

    let Depth(depth) = ListElem::depth_in(styles);
    let marker = elem
        .marker(styles)
=======
    let indent = elem.indent.get(styles);
    let body_indent = elem.body_indent.get(styles);
    let tight = elem.tight.get(styles);
    let gutter = elem.spacing.get(styles).unwrap_or_else(|| {
        if tight { styles.get(ParElem::leading) } else { styles.get(ParElem::spacing) }
    });

    let Depth(depth) = styles.get(ListElem::depth);
    let marker = elem
        .marker
        .get_ref(styles)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        .resolve(engine, styles, depth)?
        // avoid '#set align' interference with the list
        .aligned(HAlignment::Start + VAlignment::Top);

    let mut cells = vec![];
<<<<<<< HEAD
    let mut locator = locator.split();

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    for item in &elem.children {
        // Text in wide lists shall always turn into paragraphs.
        let mut body = item.body.clone();
        if !tight {
            body += ParbreakElem::shared();
        }
<<<<<<< HEAD

        cells.push(Cell::new(Content::empty(), locator.next(&())));
        cells.push(Cell::new(marker.clone(), locator.next(&marker.span())));
        cells.push(Cell::new(Content::empty(), locator.next(&())));
        cells.push(Cell::new(
            body.styled(ListElem::set_depth(Depth(1))),
            locator.next(&item.body.span()),
        ));
=======
        let body = body.set(ListElem::depth, Depth(1));

        cells.push(Cell::new(Content::empty()));
        cells.push(Cell::new(PdfMarkerTag::ListItemLabel(marker.clone())));
        cells.push(Cell::new(Content::empty()));
        cells.push(Cell::new(PdfMarkerTag::ListItemBody(body)));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    let grid = CellGrid::new(
        Axes::with_x(&[
            Sizing::Rel(indent.into()),
            Sizing::Auto,
            Sizing::Rel(body_indent.into()),
            Sizing::Auto,
        ]),
        Axes::with_y(&[gutter.into()]),
        cells,
    );
<<<<<<< HEAD
    let layouter = GridLayouter::new(&grid, regions, styles, elem.span());
=======
    let layouter = GridLayouter::new(&grid, regions, locator, styles, elem.span());
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    layouter.layout(engine)
}

/// Layout the enumeration.
#[typst_macros::time(span = elem.span())]
pub fn layout_enum(
    elem: &Packed<EnumElem>,
    engine: &mut Engine,
    locator: Locator,
    styles: StyleChain,
    regions: Regions,
) -> SourceResult<Fragment> {
<<<<<<< HEAD
    let numbering = elem.numbering(styles);
    let reversed = elem.reversed(styles);
    let indent = elem.indent(styles);
    let body_indent = elem.body_indent(styles);
    let tight = elem.tight(styles);
    let gutter = elem.spacing(styles).unwrap_or_else(|| {
        if tight {
            ParElem::leading_in(styles).into()
        } else {
            ParElem::spacing_in(styles).into()
        }
    });

    let mut cells = vec![];
    let mut locator = locator.split();
    let mut number =
        elem.start(styles)
            .unwrap_or_else(|| if reversed { elem.children.len() } else { 1 });
    let mut parents = EnumElem::parents_in(styles);

    let full = elem.full(styles);
=======
    let numbering = elem.numbering.get_ref(styles);
    let reversed = elem.reversed.get(styles);
    let indent = elem.indent.get(styles);
    let body_indent = elem.body_indent.get(styles);
    let tight = elem.tight.get(styles);
    let gutter = elem.spacing.get(styles).unwrap_or_else(|| {
        if tight { styles.get(ParElem::leading) } else { styles.get(ParElem::spacing) }
    });

    let mut cells = vec![];
    let mut number = elem
        .start
        .get(styles)
        .unwrap_or_else(|| if reversed { elem.children.len() as u64 } else { 1 });
    let mut parents = styles.get_cloned(EnumElem::parents);

    let full = elem.full.get(styles);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    // Horizontally align based on the given respective parameter.
    // Vertically align to the top to avoid inheriting `horizon` or `bottom`
    // alignment from the context and having the number be displaced in
    // relation to the item it refers to.
<<<<<<< HEAD
    let number_align = elem.number_align(styles);

    for item in &elem.children {
        number = item.number(styles).unwrap_or(number);
=======
    let number_align = elem.number_align.get(styles);

    for item in &elem.children {
        number = item.number.get(styles).unwrap_or(number);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        let context = Context::new(None, Some(styles));
        let resolved = if full {
            parents.push(number);
            let content = numbering.apply(engine, context.track(), &parents)?.display();
            parents.pop();
            content
        } else {
            match numbering {
                Numbering::Pattern(pattern) => {
                    TextElem::packed(pattern.apply_kth(parents.len(), number))
                }
                other => other.apply(engine, context.track(), &[number])?.display(),
            }
        };

        // Disable overhang as a workaround to end-aligned dots glitching
        // and decreasing spacing between numbers and items.
<<<<<<< HEAD
        let resolved =
            resolved.aligned(number_align).styled(TextElem::set_overhang(false));
=======
        let resolved = resolved.aligned(number_align).set(TextElem::overhang, false);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        // Text in wide enums shall always turn into paragraphs.
        let mut body = item.body.clone();
        if !tight {
            body += ParbreakElem::shared();
        }

<<<<<<< HEAD
        cells.push(Cell::new(Content::empty(), locator.next(&())));
        cells.push(Cell::new(resolved, locator.next(&())));
        cells.push(Cell::new(Content::empty(), locator.next(&())));
        cells.push(Cell::new(
            body.styled(EnumElem::set_parents(smallvec![number])),
            locator.next(&item.body.span()),
        ));
=======
        let body = body.set(EnumElem::parents, smallvec![number]);

        cells.push(Cell::new(Content::empty()));
        cells.push(Cell::new(PdfMarkerTag::ListItemLabel(resolved)));
        cells.push(Cell::new(Content::empty()));
        cells.push(Cell::new(PdfMarkerTag::ListItemBody(body)));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        number =
            if reversed { number.saturating_sub(1) } else { number.saturating_add(1) };
    }

    let grid = CellGrid::new(
        Axes::with_x(&[
            Sizing::Rel(indent.into()),
            Sizing::Auto,
            Sizing::Rel(body_indent.into()),
            Sizing::Auto,
        ]),
        Axes::with_y(&[gutter.into()]),
        cells,
    );
<<<<<<< HEAD
    let layouter = GridLayouter::new(&grid, regions, styles, elem.span());
=======
    let layouter = GridLayouter::new(&grid, regions, locator, styles, elem.span());
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    layouter.layout(engine)
}
