use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};

use typst_library::engine::Engine;
<<<<<<< HEAD
use typst_library::introspection::{SplitLocator, Tag};
use typst_library::layout::{Abs, Dir, Em, Fr, Frame, FrameItem, Point};
use typst_library::model::ParLineMarker;
use typst_library::text::{Lang, TextElem};
use typst_utils::Numeric;

use super::*;
=======
use typst_library::foundations::Resolve;
use typst_library::introspection::{SplitLocator, Tag, TagFlags};
use typst_library::layout::{Abs, Dir, Em, Fr, Frame, FrameItem, Point};
use typst_library::model::ParLineMarker;
use typst_library::text::{Lang, TextElem, variant};
use typst_utils::Numeric;

use super::*;
use crate::inline::linebreak::Trim;
use crate::inline::shaping::Adjustability;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use crate::modifiers::layout_and_modify;

const SHY: char = '\u{ad}';
const HYPHEN: char = '-';
const EN_DASH: char = '–';
const EM_DASH: char = '—';
const LINE_SEPARATOR: char = '\u{2028}'; // We use LS to distinguish justified breaks.

/// A layouted line, consisting of a sequence of layouted inline items that are
/// mostly borrowed from the preparation phase. This type enables you to measure
/// the size of a line in a range before committing to building the line's
/// frame.
///
/// At most two inline items must be created individually for this line: The
/// first and last one since they may be broken apart by the start or end of the
/// line, respectively. But even those can partially reuse previous results when
/// the break index is safe-to-break per rustybuzz.
pub struct Line<'a> {
    /// The items the line is made of.
    pub items: Items<'a>,
    /// The exact natural width of the line.
    pub width: Abs,
    /// Whether the line should be justified.
    pub justify: bool,
    /// Whether the line ends with a hyphen or dash, either naturally or through
    /// hyphenation.
    pub dash: Option<Dash>,
}

impl Line<'_> {
    /// Create an empty line.
    pub fn empty() -> Self {
        Self {
            items: Items::new(),
            width: Abs::zero(),
            justify: false,
            dash: None,
        }
    }

    /// How many glyphs are in the text where we can insert additional
    /// space when encountering underfull lines.
    pub fn justifiables(&self) -> usize {
        let mut count = 0;
        for shaped in self.items.iter().filter_map(Item::text) {
            count += shaped.justifiables();
        }

        // CJK character at line end should not be adjusted.
        if self
            .items
<<<<<<< HEAD
            .last()
            .and_then(Item::text)
=======
            .trailing_text()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            .map(|s| s.cjk_justifiable_at_last())
            .unwrap_or(false)
        {
            count -= 1;
        }

        count
    }

    /// How much the line can stretch.
    pub fn stretchability(&self) -> Abs {
        self.items
            .iter()
            .filter_map(Item::text)
            .map(|s| s.stretchability())
            .sum()
    }

    /// How much the line can shrink.
    pub fn shrinkability(&self) -> Abs {
        self.items
            .iter()
            .filter_map(Item::text)
            .map(|s| s.shrinkability())
            .sum()
    }

    /// Whether the line has items with negative width.
    pub fn has_negative_width_items(&self) -> bool {
        self.items.iter().any(|item| match item {
            Item::Absolute(amount, _) => *amount < Abs::zero(),
            Item::Frame(frame) => frame.width() < Abs::zero(),
            _ => false,
        })
    }

    /// The sum of fractions in the line.
    pub fn fr(&self) -> Fr {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Fractional(fr, _) => Some(*fr),
                _ => None,
            })
            .sum()
    }
}

/// A dash at the end of a line.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dash {
    /// A soft hyphen added to break a word.
    Soft,
    /// A regular hyphen, present in a compound word, e.g. beija-flor.
    Hard,
    /// Another kind of dash. Only relevant for cost computation.
    Other,
}

/// Create a line which spans the given range.
pub fn line<'a>(
    engine: &Engine,
    p: &'a Preparation,
    range: Range,
    breakpoint: Breakpoint,
<<<<<<< HEAD
    pred: Option<&Line>,
=======
    pred: Option<&Line<'a>>,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
) -> Line<'a> {
    // The line's full text.
    let full = &p.text[range.clone()];

    // Whether the line is justified.
    let justify = full.ends_with(LINE_SEPARATOR)
        || (p.config.justify && breakpoint != Breakpoint::Mandatory);

    // Process dashes.
    let dash = if breakpoint.is_hyphen() || full.ends_with(SHY) {
        Some(Dash::Soft)
    } else if full.ends_with(HYPHEN) {
        Some(Dash::Hard)
    } else if full.ends_with([EN_DASH, EM_DASH]) {
        Some(Dash::Other)
    } else {
        None
    };

    // Trim the line at the end, if necessary for this breakpoint.
<<<<<<< HEAD
    let trim = range.start + breakpoint.trim(full).len();

    // Collect the items for the line.
    let mut items = collect_items(engine, p, range, trim);

    // Add a hyphen at the line start, if a previous dash should be repeated.
    if pred.map_or(false, |pred| should_repeat_hyphen(pred, full)) {
        if let Some(shaped) = items.first_text_mut() {
            shaped.prepend_hyphen(engine, p.config.fallback);
        }
    }

    // Add a hyphen at the line end, if we ended on a soft hyphen.
    if dash == Some(Dash::Soft) {
        if let Some(shaped) = items.last_text_mut() {
            shaped.push_hyphen(engine, p.config.fallback);
        }
    }

    // Deal with CJ characters at line boundaries.
    adjust_cj_at_line_boundaries(p, full, &mut items);
=======
    let trim = breakpoint.trim(range.start, full);
    let trimmed_range = range.start..trim.layout;

    // Collect the items for the line.
    let mut items = Items::new();

    // Add a hyphen at the line start, if a previous dash should be repeated.
    if let Some(pred) = pred
        && pred.dash == Some(Dash::Hard)
        && let Some(base) = pred.items.trailing_text()
        && should_repeat_hyphen(base.lang, full)
        && let Some(hyphen) =
            ShapedText::hyphen(engine, p.config.fallback, base, trim.shaping, false)
    {
        items.push(Item::Text(hyphen), LogicalIndex::START_HYPHEN);
    }

    collect_items(&mut items, engine, p, range, &trim);

    // Add a hyphen at the line end, if we ended on a soft hyphen.
    if dash == Some(Dash::Soft)
        && let Some(base) = items.trailing_text()
        && let Some(hyphen) =
            ShapedText::hyphen(engine, p.config.fallback, base, trim.shaping, true)
    {
        items.push(Item::Text(hyphen), LogicalIndex::END_HYPHEN);
    }

    // Ensure that there is no weak spacing at the start and end of the line.
    trim_weak_spacing(&mut items);

    // Deal with CJ characters at line boundaries.
    // Use the trimmed range for robust boundary checks.
    adjust_cj_at_line_boundaries(p, trimmed_range, &mut items);

    // Deal with stretchability of glyphs at the end of the line.
    adjust_glyph_stretch_at_line_end(p, &mut items);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    // Compute the line's width.
    let width = items.iter().map(Item::natural_width).sum();

    Line { items, width, justify, dash }
}

/// Collects / reshapes all items for the line with the given `range`.
///
/// The `trim` defines an end position to which text items are trimmed. For
/// example, the `range` may span "hello\n", but the `trim` specifies that the
/// linebreak is trimmed.
///
/// We do not factor the `trim` directly into the `range` because we still want
/// to keep non-text items after the trim (e.g. tags).
fn collect_items<'a>(
<<<<<<< HEAD
    engine: &Engine,
    p: &'a Preparation,
    range: Range,
    trim: usize,
) -> Items<'a> {
    let mut items = Items::new();
=======
    items: &mut Items<'a>,
    engine: &Engine,
    p: &'a Preparation,
    range: Range,
    trim: &Trim,
) {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let mut fallback = None;

    // Collect the items for each consecutively ordered run.
    reorder(p, range.clone(), |subrange, rtl| {
        let from = items.len();
<<<<<<< HEAD
        collect_range(engine, p, subrange, trim, &mut items, &mut fallback);
=======
        collect_range(engine, p, subrange, trim, items, &mut fallback);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        if rtl {
            items.reorder(from);
        }
    });

<<<<<<< HEAD
=======
    // Add fallback text to expand the line height, if necessary.
    if !items.iter().any(|item| matches!(item, Item::Text(_)))
        && let Some(fallback) = fallback
    {
        items.push(fallback, LogicalIndex::FALLBACK_TEXT);
    }
}

/// Trims weak spacing from the start and end of the line.
fn trim_weak_spacing(items: &mut Items) {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    // Trim weak spacing at the start of the line.
    let prefix = items
        .iter()
        .take_while(|item| matches!(item, Item::Absolute(_, true)))
        .count();
    if prefix > 0 {
        items.drain(..prefix);
    }

    // Trim weak spacing at the end of the line.
<<<<<<< HEAD
    while matches!(items.last(), Some(Item::Absolute(_, true))) {
        items.pop();
    }

    // Add fallback text to expand the line height, if necessary.
    if !items.iter().any(|item| matches!(item, Item::Text(_))) {
        if let Some(fallback) = fallback {
            items.push(fallback);
        }
    }

    items
=======
    while matches!(items.iter().next_back(), Some(Item::Absolute(_, true))) {
        items.pop();
    }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Calls `f` for the BiDi-reordered ranges of a line.
fn reorder<F>(p: &Preparation, range: Range, mut f: F)
where
    F: FnMut(Range, bool),
{
    // If there is nothing bidirectional going on, skip reordering.
    let Some(bidi) = &p.bidi else {
        f(range, p.config.dir == Dir::RTL);
        return;
    };

    // The bidi crate panics for empty lines.
    if range.is_empty() {
        f(range, p.config.dir == Dir::RTL);
        return;
    }

    // Find the paragraph that contains the line.
    let para = bidi
        .paragraphs
        .iter()
        .find(|para| para.range.contains(&range.start))
        .unwrap();

    // Compute the reordered ranges in visual order (left to right).
    let (levels, runs) = bidi.visual_runs(para, range.clone());

    // Call `f` for each run.
    for run in runs {
        let rtl = levels[run.start].is_rtl();
        f(run, rtl)
    }
}

/// Collects / reshapes all items for the given `subrange` with continuous
/// direction.
fn collect_range<'a>(
    engine: &Engine,
    p: &'a Preparation,
    range: Range,
<<<<<<< HEAD
    trim: usize,
    items: &mut Items<'a>,
    fallback: &mut Option<ItemEntry<'a>>,
) {
    for (subrange, item) in p.slice(range.clone()) {
        // All non-text items are just kept, they can't be split.
        let Item::Text(shaped) = item else {
            items.push(item);
=======
    trim: &Trim,
    items: &mut Items<'a>,
    fallback: &mut Option<ItemEntry<'a>>,
) {
    for (i, (subrange, item)) in p.slice(range.clone()) {
        let idx = LogicalIndex::from_item_index(i);

        // All non-text items are just kept, they can't be split.
        let Item::Text(shaped) = item else {
            items.push(item, idx);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            continue;
        };

        // The intersection range of the item, the subrange, and the line's
        // trimming.
<<<<<<< HEAD
        let sliced =
            range.start.max(subrange.start)..range.end.min(subrange.end).min(trim);
=======
        let sliced = range.start.max(subrange.start)
            ..range.end.min(subrange.end).min(trim.shaping);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        // Whether the item is split by the line.
        let split = subrange.start < sliced.start || sliced.end < subrange.end;

        if sliced.is_empty() {
            // When there is no text, still keep this as a fallback item, which
            // we can use to force a non-zero line-height when the line doesn't
            // contain any other text.
            *fallback = Some(ItemEntry::from(Item::Text(shaped.empty())));
<<<<<<< HEAD
        } else if split {
            // When the item is split in half, reshape it.
            let reshaped = shaped.reshape(engine, sliced);
            items.push(Item::Text(reshaped));
        } else {
            // When the item is fully contained, just keep it.
            items.push(item);
        }
=======
            continue;
        }

        let mut item: ItemEntry = if split {
            // When the item is split in half, reshape it.
            let reshaped = shaped.reshape(engine, sliced);
            Item::Text(reshaped).into()
        } else {
            // When the item is fully contained, just keep it.
            item.into()
        };

        // Trim end-of-line whitespace glyphs.
        if trim.layout < range.end {
            let shaped = item.text_mut().unwrap();
            shaped.glyphs.trim(|glyph| trim.layout < glyph.range.end);
        }

        items.push(item, idx);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

/// Add spacing around punctuation marks for CJ glyphs at line boundaries.
///
/// See Requirements for Chinese Text Layout, Section 3.1.6.3 Compression of
/// punctuation marks at line start or line end.
<<<<<<< HEAD
fn adjust_cj_at_line_boundaries(p: &Preparation, text: &str, items: &mut Items) {
=======
///
/// The `range` should only contain regular texts, with linebreaks trimmed.
fn adjust_cj_at_line_boundaries(p: &Preparation, range: Range, items: &mut Items) {
    let text = &p.text[range];

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    if text.starts_with(BEGIN_PUNCT_PAT)
        || (p.config.cjk_latin_spacing && text.starts_with(is_of_cj_script))
    {
        adjust_cj_at_line_start(p, items);
    }

    if text.ends_with(END_PUNCT_PAT)
        || (p.config.cjk_latin_spacing && text.ends_with(is_of_cj_script))
    {
        adjust_cj_at_line_end(p, items);
    }
}

<<<<<<< HEAD
/// Add spacing around punctuation marks for CJ glyphs at the line start.
fn adjust_cj_at_line_start(p: &Preparation, items: &mut Items) {
    let Some(shaped) = items.first_text_mut() else { return };
=======
/// Remove stretchability from the last glyph in the line to avoid trailing
/// space after a glyph due to glyph-level justification.
fn adjust_glyph_stretch_at_line_end(p: &Preparation, items: &mut Items) {
    let glyph_limits = &p.config.justification_limits.tracking();
    if glyph_limits.min.is_zero() && glyph_limits.max.is_zero() {
        return;
    }

    // This is not perfect (ignores clusters and is not particularly fast), but
    // it is good enough for now. The adjustability handling in general needs a
    // cleanup.
    let Some(shaped) = items.trailing_text_mut() else { return };
    let Some(glyph) = shaped.glyphs.to_mut().last_mut() else { return };
    glyph.adjustability = Adjustability::default();
}

/// Add spacing around punctuation marks for CJ glyphs at the line start.
fn adjust_cj_at_line_start(p: &Preparation, items: &mut Items) {
    let Some(shaped) = items.leading_text_mut() else { return };
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let Some(glyph) = shaped.glyphs.first() else { return };

    if glyph.is_cjk_right_aligned_punctuation() {
        // If the first glyph is a CJK punctuation, we want to
        // shrink it.
        let glyph = shaped.glyphs.to_mut().first_mut().unwrap();
        let shrink = glyph.shrinkability().0;
        glyph.shrink_left(shrink);
<<<<<<< HEAD
        shaped.width -= shrink.at(shaped.size);
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    } else if p.config.cjk_latin_spacing
        && glyph.is_cj_script()
        && glyph.x_offset > Em::zero()
    {
        // If the first glyph is a CJK character adjusted by
        // [`add_cjk_latin_spacing`], restore the original width.
        let glyph = shaped.glyphs.to_mut().first_mut().unwrap();
        let shrink = glyph.x_offset;
        glyph.x_advance -= shrink;
        glyph.x_offset = Em::zero();
        glyph.adjustability.shrinkability.0 = Em::zero();
<<<<<<< HEAD
        shaped.width -= shrink.at(shaped.size);
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

/// Add spacing around punctuation marks for CJ glyphs at the line end.
fn adjust_cj_at_line_end(p: &Preparation, items: &mut Items) {
<<<<<<< HEAD
    let Some(shaped) = items.last_text_mut() else { return };
=======
    let Some(shaped) = items.trailing_text_mut() else { return };
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let Some(glyph) = shaped.glyphs.last() else { return };

    // Deal with CJK punctuation at line ends.
    let style = cjk_punct_style(shaped.lang, shaped.region);

    if glyph.is_cjk_left_aligned_punctuation(style) {
        // If the last glyph is a CJK punctuation, we want to
        // shrink it.
        let shrink = glyph.shrinkability().1;
        let punct = shaped.glyphs.to_mut().last_mut().unwrap();
        punct.shrink_right(shrink);
<<<<<<< HEAD
        shaped.width -= shrink.at(shaped.size);
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    } else if p.config.cjk_latin_spacing
        && glyph.is_cj_script()
        && (glyph.x_advance - glyph.x_offset) > Em::one()
    {
        // If the last glyph is a CJK character adjusted by
        // [`add_cjk_latin_spacing`], restore the original width.
        let shrink = glyph.x_advance - glyph.x_offset - Em::one();
        let glyph = shaped.glyphs.to_mut().last_mut().unwrap();
        glyph.x_advance -= shrink;
        glyph.adjustability.shrinkability.1 = Em::zero();
<<<<<<< HEAD
        shaped.width -= shrink.at(shaped.size);
    }
}

/// Whether a hyphen should be inserted at the start of the next line.
fn should_repeat_hyphen(pred_line: &Line, text: &str) -> bool {
    // If the predecessor line does not end with a `Dash::Hard`, we shall
    // not place a hyphen at the start of the next line.
    if pred_line.dash != Some(Dash::Hard) {
        return false;
    }

    // The hyphen should repeat only in the languages that require that feature.
    // For more information see the discussion at https://github.com/typst/typst/issues/3235
    let Some(Item::Text(shaped)) = pred_line.items.last() else { return false };

    match shaped.lang {
=======
    }
}

/// Whether a hyphen should be repeated at the start of the line in the given
/// language, when the following text is the given one.
fn should_repeat_hyphen(lang: Lang, following_text: &str) -> bool {
    match lang {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        // - Lower Sorbian: see https://dolnoserbski.de/ortografija/psawidla/K3
        // - Czech: see https://prirucka.ujc.cas.cz/?id=164
        // - Croatian: see http://pravopis.hr/pravilo/spojnica/68/
        // - Polish: see https://www.ortograf.pl/zasady-pisowni/lacznik-zasady-pisowni
        // - Portuguese: see https://www2.senado.leg.br/bdsf/bitstream/handle/id/508145/000997415.pdf (Base XX)
        // - Slovak: see https://www.zones.sk/studentske-prace/gramatika/10620-pravopis-rozdelovanie-slov/
        Lang::LOWER_SORBIAN
        | Lang::CZECH
        | Lang::CROATIAN
        | Lang::POLISH
        | Lang::PORTUGUESE
        | Lang::SLOVAK => true,

        // In Spanish the hyphen is required only if the word next to hyphen is
        // not capitalized. Otherwise, the hyphen must not be repeated.
        //
        // See § 4.1.1.1.2.e on the "Ortografía de la lengua española"
        // https://www.rae.es/ortografía/como-signo-de-división-de-palabras-a-final-de-línea
<<<<<<< HEAD
        Lang::SPANISH => text.chars().next().map_or(false, |c| !c.is_uppercase()),
=======
        Lang::SPANISH => following_text.chars().next().is_some_and(|c| !c.is_uppercase()),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        _ => false,
    }
}

<<<<<<< HEAD
/// Apply the current baseline shift to a frame.
pub fn apply_baseline_shift(frame: &mut Frame, styles: StyleChain) {
    frame.translate(Point::with_y(TextElem::baseline_in(styles)));
=======
/// Apply the current baseline shift and italic compensation to a frame.
pub fn apply_shift<'a>(
    world: &Tracked<'a, dyn World + 'a>,
    frame: &mut Frame,
    styles: StyleChain,
) {
    let mut baseline = styles.resolve(TextElem::baseline);
    let mut compensation = Abs::zero();
    if let Some(scripts) = styles.get_ref(TextElem::shift_settings) {
        let font_metrics = styles
            .get_ref(TextElem::font)
            .into_iter()
            .find_map(|family| {
                world
                    .book()
                    .select(family.as_str(), variant(styles))
                    .and_then(|id| world.font(id))
            })
            .map_or(*scripts.kind.default_metrics(), |f| {
                *scripts.kind.read_metrics(f.metrics())
            });
        baseline -= scripts.shift.unwrap_or(font_metrics.vertical_offset).resolve(styles);
        compensation += font_metrics.horizontal_offset.resolve(styles);
    }
    frame.translate(Point::new(compensation, baseline));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Commit to a line and build its frame.
#[allow(clippy::too_many_arguments)]
pub fn commit(
    engine: &mut Engine,
    p: &Preparation,
    line: &Line,
    width: Abs,
    full: Abs,
    locator: &mut SplitLocator<'_>,
) -> SourceResult<Frame> {
    let mut remaining = width - line.width - p.config.hanging_indent;
    let mut offset = Abs::zero();

    // We always build the line from left to right. In an LTR paragraph, we must
    // thus add the hanging indent to the offset. In an RTL paragraph, the
    // hanging indent arises naturally due to the line width.
    if p.config.dir == Dir::LTR {
        offset += p.config.hanging_indent;
    }

    // Handle hanging punctuation to the left.
<<<<<<< HEAD
    if let Some(Item::Text(text)) = line.items.first() {
        if let Some(glyph) = text.glyphs.first() {
            if !text.dir.is_positive()
                && TextElem::overhang_in(text.styles)
                && (line.items.len() > 1 || text.glyphs.len() > 1)
            {
                let amount = overhang(glyph.c) * glyph.x_advance.at(text.size);
                offset -= amount;
                remaining += amount;
            }
        }
    }

    // Handle hanging punctuation to the right.
    if let Some(Item::Text(text)) = line.items.last() {
        if let Some(glyph) = text.glyphs.last() {
            if text.dir.is_positive()
                && TextElem::overhang_in(text.styles)
                && (line.items.len() > 1 || text.glyphs.len() > 1)
            {
                let amount = overhang(glyph.c) * glyph.x_advance.at(text.size);
                remaining += amount;
            }
        }
=======
    if let Some(text) = line.items.leading_text()
        && let Some(glyph) = text.glyphs.first()
        && !text.dir.is_positive()
        && text.styles.get(TextElem::overhang)
        && (line.items.len() > 1 || text.glyphs.len() > 1)
    {
        let amount = overhang(glyph.c) * glyph.x_advance.at(glyph.size);
        offset -= amount;
        remaining += amount;
    }

    // Handle hanging punctuation to the right.
    if let Some(text) = line.items.trailing_text()
        && let Some(glyph) = text.glyphs.last()
        && text.dir.is_positive()
        && text.styles.get(TextElem::overhang)
        && (line.items.len() > 1 || text.glyphs.len() > 1)
    {
        let amount = overhang(glyph.c) * glyph.x_advance.at(glyph.size);
        remaining += amount;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    // Determine how much additional space is needed. The justification_ratio is
    // for the first step justification, extra_justification is for the last
    // step. For more info on multi-step justification, see Procedures for
    // Inter- Character Space Expansion in W3C document Chinese Layout
    // Requirements.
    let fr = line.fr();
    let mut justification_ratio = 0.0;
    let mut extra_justification = Abs::zero();

    let shrinkability = line.shrinkability();
    let stretchability = line.stretchability();
    if remaining < Abs::zero() && shrinkability > Abs::zero() {
        // Attempt to reduce the length of the line, using shrinkability.
        justification_ratio = (remaining / shrinkability).max(-1.0);
        remaining = (remaining + shrinkability).min(Abs::zero());
    } else if line.justify && fr.is_zero() {
        // Attempt to increase the length of the line, using stretchability.
        if stretchability > Abs::zero() {
            justification_ratio = (remaining / stretchability).min(1.0);
            remaining = (remaining - stretchability).max(Abs::zero());
        }

        let justifiables = line.justifiables();
        if justifiables > 0 && remaining > Abs::zero() {
            // Underfull line, distribute the extra space.
            extra_justification = remaining / justifiables as f64;
            remaining = Abs::zero();
        }
    }

    let mut top = Abs::zero();
    let mut bottom = Abs::zero();

    // Build the frames and determine the height and baseline.
    let mut frames = vec![];
<<<<<<< HEAD
    for item in line.items.iter() {
        let mut push = |offset: &mut Abs, frame: Frame| {
            let width = frame.width();
            top.set_max(frame.baseline());
            bottom.set_max(frame.size().y - frame.baseline());
            frames.push((*offset, frame));
            *offset += width;
        };

        match item {
=======
    for &(idx, ref item) in line.items.indexed_iter() {
        let mut push = |offset: &mut Abs, frame: Frame, idx: LogicalIndex| {
            let width = frame.width();
            top.set_max(frame.baseline());
            bottom.set_max(frame.size().y - frame.baseline());
            frames.push((*offset, frame, idx));
            *offset += width;
        };

        match &**item {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Item::Absolute(v, _) => {
                offset += *v;
            }
            Item::Fractional(v, elem) => {
                let amount = v.share(fr, remaining);
                if let Some((elem, loc, styles)) = elem {
                    let region = Size::new(amount, full);
                    let mut frame = layout_and_modify(*styles, |styles| {
                        layout_box(elem, engine, loc.relayout(), styles, region)
                    })?;
<<<<<<< HEAD
                    apply_baseline_shift(&mut frame, *styles);
                    push(&mut offset, frame);
=======
                    apply_shift(&engine.world, &mut frame, *styles);
                    push(&mut offset, frame, idx);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                } else {
                    offset += amount;
                }
            }
            Item::Text(shaped) => {
                let frame = shaped.build(
                    engine,
                    &p.spans,
                    justification_ratio,
                    extra_justification,
                );
<<<<<<< HEAD
                push(&mut offset, frame);
            }
            Item::Frame(frame) => {
                push(&mut offset, frame.clone());
=======
                push(&mut offset, frame, idx);
            }
            Item::Frame(frame) => {
                push(&mut offset, frame.clone(), idx);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            }
            Item::Tag(tag) => {
                let mut frame = Frame::soft(Size::zero());
                frame.push(Point::zero(), FrameItem::Tag((*tag).clone()));
<<<<<<< HEAD
                frames.push((offset, frame));
=======
                frames.push((offset, frame, idx));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            }
            Item::Skip(_) => {}
        }
    }

    // Remaining space is distributed now.
    if !fr.is_zero() {
        remaining = Abs::zero();
    }

    let size = Size::new(width, top + bottom);
    let mut output = Frame::soft(size);
    output.set_baseline(top);

    if let Some(marker) = &p.config.numbering_marker {
        add_par_line_marker(&mut output, marker, engine, locator, top);
    }

<<<<<<< HEAD
    // Construct the line's frame.
    for (offset, frame) in frames {
=======
    // Ensure that the final frame's items are in logical order rather than in
    // visual order. This is important because it affects the order of elements
    // during introspection and thus things like counters.
    frames.sort_unstable_by_key(|(_, _, idx)| *idx);

    // Construct the line's frame.
    for (offset, frame, _) in frames {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        let x = offset + p.config.align.position(remaining);
        let y = top - frame.baseline();
        output.push_frame(Point::new(x, y), frame);
    }

    Ok(output)
}

/// Adds a paragraph line marker to a paragraph line's output frame if
/// line numbering is not `None` at this point. Ensures other style properties,
/// namely number margin, number align and number clearance, are stored in the
/// marker as well.
///
/// The `top` parameter is used to ensure the marker, and thus the line's
/// number in the margin, is aligned to the line's baseline.
fn add_par_line_marker(
    output: &mut Frame,
    marker: &Packed<ParLineMarker>,
    engine: &mut Engine,
    locator: &mut SplitLocator,
    top: Abs,
) {
    // Elements in tags must have a location for introspection to work. We do
    // the work here instead of going through all of the realization process
    // just for this, given we don't need to actually place the marker as we
    // manually search for it in the frame later (when building a root flow,
    // where line numbers can be displayed), so we just need it to be in a tag
    // and to be valid (to have a location).
    let mut marker = marker.clone();
    let key = typst_utils::hash128(&marker);
    let loc = locator.next_location(engine.introspector, key);
    marker.set_location(loc);

    // Create start and end tags through which we can search for this line's
    // marker later. The 'x' coordinate is not important, just the 'y'
    // coordinate, as that's what is used for line numbers. We will place the
    // tags among other subframes in the line such that it is aligned with the
    // line's general baseline. However, the line number will still need to
    // manually adjust its own 'y' position based on its own baseline.
    let pos = Point::with_y(top);
<<<<<<< HEAD
    output.push(pos, FrameItem::Tag(Tag::Start(marker.pack())));
    output.push(pos, FrameItem::Tag(Tag::End(loc, key)));
=======
    let flags = TagFlags { introspectable: false, tagged: false };
    output.push(pos, FrameItem::Tag(Tag::Start(marker.pack(), flags)));
    output.push(pos, FrameItem::Tag(Tag::End(loc, key, flags)));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// How much a character should hang into the end margin.
///
/// For more discussion, see:
/// <https://recoveringphysicist.com/21/>
fn overhang(c: char) -> f64 {
    match c {
        // Dashes.
        '–' | '—' => 0.2,
<<<<<<< HEAD
        '-' => 0.55,
=======
        '-' | '\u{ad}' => 0.55,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        // Punctuation.
        '.' | ',' => 0.8,
        ':' | ';' => 0.3,

        // Arabic
        '\u{60C}' | '\u{6D4}' => 0.4,

        _ => 0.0,
    }
}

/// A collection of owned or borrowed inline items.
<<<<<<< HEAD
pub struct Items<'a>(Vec<ItemEntry<'a>>);
=======
pub struct Items<'a>(Vec<(LogicalIndex, ItemEntry<'a>)>);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

impl<'a> Items<'a> {
    /// Create empty items.
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Push a new item.
<<<<<<< HEAD
    pub fn push(&mut self, entry: impl Into<ItemEntry<'a>>) {
        self.0.push(entry.into());
    }

    /// Iterate over the items
    pub fn iter(&self) -> impl Iterator<Item = &Item<'a>> {
        self.0.iter().map(|item| &**item)
    }

    /// Access the first item.
    pub fn first(&self) -> Option<&Item<'a>> {
        self.0.first().map(|item| &**item)
    }

    /// Access the last item.
    pub fn last(&self) -> Option<&Item<'a>> {
        self.0.last().map(|item| &**item)
    }

    /// Access the first item mutably, if it is text.
    pub fn first_text_mut(&mut self) -> Option<&mut ShapedText<'a>> {
        self.0.first_mut()?.text_mut()
    }

    /// Access the last item mutably, if it is text.
    pub fn last_text_mut(&mut self) -> Option<&mut ShapedText<'a>> {
        self.0.last_mut()?.text_mut()
=======
    pub fn push(&mut self, entry: impl Into<ItemEntry<'a>>, idx: LogicalIndex) {
        self.0.push((idx, entry.into()));
    }

    /// Iterate over the items.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Item<'a>> {
        self.0.iter().map(|(_, item)| &**item)
    }

    /// Iterate over the items with the indices that define their logical order.
    /// See the docs above `logical_item_idx` for more details.
    ///
    /// Note that this is different from `.iter().enumerate()` which would
    /// provide the indices in visual order!
    pub fn indexed_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = &(LogicalIndex, ItemEntry<'a>)> {
        self.0.iter()
    }

    /// Access the first item (skipping tags), if it is text.
    pub fn leading_text(&self) -> Option<&ShapedText<'a>> {
        self.0.iter().find(|(_, item)| !item.is_tag())?.1.text()
    }

    /// Access the first item (skipping tags) mutably, if it is text.
    pub fn leading_text_mut(&mut self) -> Option<&mut ShapedText<'a>> {
        self.0.iter_mut().find(|(_, item)| !item.is_tag())?.1.text_mut()
    }

    /// Access the last item (skipping tags), if it is text.
    pub fn trailing_text(&self) -> Option<&ShapedText<'a>> {
        self.0.iter().rev().find(|(_, item)| !item.is_tag())?.1.text()
    }

    /// Access the last item (skipping tags) mutably, if it is text.
    pub fn trailing_text_mut(&mut self) -> Option<&mut ShapedText<'a>> {
        self.0.iter_mut().rev().find(|(_, item)| !item.is_tag())?.1.text_mut()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    /// Reorder the items starting at the given index to RTL.
    pub fn reorder(&mut self, from: usize) {
        self.0[from..].reverse()
    }
}

<<<<<<< HEAD
impl<'a> FromIterator<ItemEntry<'a>> for Items<'a> {
    fn from_iter<I: IntoIterator<Item = ItemEntry<'a>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> Deref for Items<'a> {
    type Target = Vec<ItemEntry<'a>>;
=======
impl<'a> Deref for Items<'a> {
    type Target = Vec<(LogicalIndex, ItemEntry<'a>)>;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Items<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for Items<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

<<<<<<< HEAD
/// A reference to or a boxed item.
=======
/// We use indices to remember the logical (as opposed to visual) order of
/// items. During line building, the items are stored in visual (BiDi-reordered)
/// order. When committing to a line and building its frame, we sort by logical
/// index.
///
/// - Special layout-generated items have custom indices that ensure correct
///   ordering w.r.t. to each other and normal elements, listed below.
/// - Normal items have their position in `p.items` plus the number of special
///   reserved prefix indices.
///
/// Logical indices must be unique within a line because we use an unstable
/// sort.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LogicalIndex(usize);

impl LogicalIndex {
    const START_HYPHEN: Self = Self(0);
    const FALLBACK_TEXT: Self = Self(usize::MAX - 1);
    const END_HYPHEN: Self = Self(usize::MAX);

    /// Create a logical index from the index of an item in the [`p.items`](Preparation::items).
    const fn from_item_index(i: usize) -> Self {
        // This won't overflow because the `idx` comes from a vector which is
        // limited to `isize::MAX` elements.
        Self(i + 1)
    }
}

/// A reference to or a boxed item.
///
/// This is conceptually similar to a [`Cow<'a, Item<'a>>`][std::borrow::Cow],
/// but we box owned items since an [`Item`] is much bigger than
/// a box.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub enum ItemEntry<'a> {
    Ref(&'a Item<'a>),
    Box(Box<Item<'a>>),
}

impl<'a> ItemEntry<'a> {
    fn text_mut(&mut self) -> Option<&mut ShapedText<'a>> {
        match self {
            Self::Ref(item) => {
                let text = item.text()?;
                *self = Self::Box(Box::new(Item::Text(text.clone())));
                match self {
                    Self::Box(item) => item.text_mut(),
                    _ => unreachable!(),
                }
            }
            Self::Box(item) => item.text_mut(),
        }
    }
}

impl<'a> Deref for ItemEntry<'a> {
    type Target = Item<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(item) => item,
            Self::Box(item) => item,
        }
    }
}

impl Debug for ItemEntry<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a> From<&'a Item<'a>> for ItemEntry<'a> {
    fn from(item: &'a Item<'a>) -> Self {
        Self::Ref(item)
    }
}

impl<'a> From<Item<'a>> for ItemEntry<'a> {
    fn from(item: Item<'a>) -> Self {
        Self::Box(Box::new(item))
    }
}
