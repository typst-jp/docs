use typst_library::diag::SourceResult;
<<<<<<< HEAD
use typst_library::foundations::{Packed, StyleChain};
use typst_library::layout::{Em, Frame, Point, Size};
use typst_library::math::{Accent, AccentElem};

use super::{style_cramped, FrameFragment, GlyphFragment, MathContext, MathFragment};
=======
use typst_library::foundations::{Packed, StyleChain, SymbolElem};
use typst_library::layout::{Em, Frame, Point, Size};
use typst_library::math::AccentElem;

use super::{
    FrameFragment, MathContext, MathFragment, style_cramped, style_dtls, style_flac,
};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

/// How much the accent can be shorter than the base.
const ACCENT_SHORT_FALL: Em = Em::new(0.5);

/// Lays out an [`AccentElem`].
#[typst_macros::time(name = "math.accent", span = elem.span())]
pub fn layout_accent(
    elem: &Packed<AccentElem>,
    ctx: &mut MathContext,
    styles: StyleChain,
) -> SourceResult<()> {
<<<<<<< HEAD
    let cramped = style_cramped();
    let mut base = ctx.layout_into_fragment(&elem.base, styles.chain(&cramped))?;

    // Try to replace a glyph with its dotless variant.
    if let MathFragment::Glyph(glyph) = &mut base {
        glyph.make_dotless_form(ctx);
    }
=======
    let accent = elem.accent;
    let top_accent = !accent.is_bottom();

    // Try to replace the base glyph with its dotless variant.
    let dtls = style_dtls();
    let base_styles =
        if top_accent && elem.dotless.get(styles) { styles.chain(&dtls) } else { styles };

    let cramped = style_cramped();
    let base_styles = base_styles.chain(&cramped);
    let base = ctx.layout_into_fragment(&elem.base, base_styles)?;

    let (font, size) = base.font(ctx, base_styles);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    // Preserve class to preserve automatic spacing.
    let base_class = base.class();
    let base_attach = base.accent_attach();

<<<<<<< HEAD
    let width = elem.size(styles).relative_to(base.width());

    let Accent(c) = elem.accent;
    let mut glyph = GlyphFragment::new(ctx, styles, c, elem.span());

    // Try to replace accent glyph with flattened variant.
    let flattened_base_height = scaled!(ctx, styles, flattened_accent_base_height);
    if base.height() > flattened_base_height {
        glyph.make_flattened_accent_form(ctx);
    }

    // Forcing the accent to be at least as large as the base makes it too
    // wide in many case.
    let short_fall = ACCENT_SHORT_FALL.at(glyph.font_size);
    let variant = glyph.stretch_horizontal(ctx, width, short_fall);
    let accent = variant.frame;
    let accent_attach = variant.accent_attach;

    // Descent is negative because the accent's ink bottom is above the
    // baseline. Therefore, the default gap is the accent's negated descent
    // minus the accent base height. Only if the base is very small, we need
    // a larger gap so that the accent doesn't move too low.
    let accent_base_height = scaled!(ctx, styles, accent_base_height);
    let gap = -accent.descent() - base.height().min(accent_base_height);
    let size = Size::new(base.width(), accent.height() + gap + base.height());
    let accent_pos = Point::with_x(base_attach - accent_attach);
    let base_pos = Point::with_y(accent.height() + gap);
    let baseline = base_pos.y + base.ascent();
    let base_italics_correction = base.italics_correction();
    let base_text_like = base.is_text_like();

=======
    // Try to replace the accent glyph with its flattened variant.
    let flattened_base_height = font.math().flattened_accent_base_height.at(size);
    let flac = style_flac();
    let accent_styles = if top_accent && base.ascent() > flattened_base_height {
        styles.chain(&flac)
    } else {
        styles
    };

    let mut accent = ctx.layout_into_fragment(
        &SymbolElem::packed(accent.0).spanned(elem.span()),
        accent_styles,
    )?;

    // Forcing the accent to be at least as large as the base makes it too wide
    // in many cases.
    let width = elem.size.resolve(styles).relative_to(base.width());
    let short_fall = ACCENT_SHORT_FALL.at(size);
    accent.stretch_horizontal(ctx, width, short_fall);
    let accent_attach = accent.accent_attach().0;
    let accent = accent.into_frame();

    let (gap, accent_pos, base_pos) = if top_accent {
        // Descent is negative because the accent's ink bottom is above the
        // baseline. Therefore, the default gap is the accent's negated descent
        // minus the accent base height. Only if the base is very small, we
        // need a larger gap so that the accent doesn't move too low.
        let accent_base_height = font.math().accent_base_height.at(size);
        let gap = -accent.descent() - base.ascent().min(accent_base_height);
        let accent_pos = Point::with_x(base_attach.0 - accent_attach);
        let base_pos = Point::with_y(accent.height() + gap);
        (gap, accent_pos, base_pos)
    } else {
        let gap = -accent.ascent();
        let accent_pos = Point::new(base_attach.1 - accent_attach, base.height() + gap);
        let base_pos = Point::zero();
        (gap, accent_pos, base_pos)
    };

    let size = Size::new(base.width(), accent.height() + gap + base.height());
    let baseline = base_pos.y + base.ascent();

    let base_italics_correction = base.italics_correction();
    let base_text_like = base.is_text_like();
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let base_ascent = match &base {
        MathFragment::Frame(frame) => frame.base_ascent,
        _ => base.ascent(),
    };
<<<<<<< HEAD
=======
    let base_descent = match &base {
        MathFragment::Frame(frame) => frame.base_descent,
        _ => base.descent(),
    };
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    let mut frame = Frame::soft(size);
    frame.set_baseline(baseline);
    frame.push_frame(accent_pos, accent);
    frame.push_frame(base_pos, base.into_frame());
    ctx.push(
        FrameFragment::new(styles, frame)
            .with_class(base_class)
            .with_base_ascent(base_ascent)
<<<<<<< HEAD
=======
            .with_base_descent(base_descent)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            .with_italics_correction(base_italics_correction)
            .with_accent_attach(base_attach)
            .with_text_like(base_text_like),
    );

    Ok(())
}
