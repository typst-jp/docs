use std::fmt::{self, Debug, Formatter};

<<<<<<< HEAD
use rustybuzz::Feature;
use ttf_parser::gsub::{AlternateSubstitution, SingleSubstitution, SubstitutionSubtable};
use ttf_parser::opentype_layout::LayoutTable;
use ttf_parser::{GlyphId, Rect};
use typst_library::foundations::StyleChain;
use typst_library::introspection::Tag;
use typst_library::layout::{
    Abs, Axis, Corner, Em, Frame, FrameItem, Point, Size, VAlignment,
};
use typst_library::math::{EquationElem, MathSize};
use typst_library::text::{Font, Glyph, Lang, Region, TextElem, TextItem};
use typst_library::visualize::Paint;
use typst_syntax::Span;
use typst_utils::default_math_class;
use unicode_math_class::MathClass;

use super::{stretch_glyph, MathContext, Scaled};
use crate::modifiers::{FrameModifiers, FrameModify};

#[derive(Debug, Clone)]
pub enum MathFragment {
    Glyph(GlyphFragment),
    Variant(VariantFragment),
=======
use az::SaturatingAs;
use comemo::Tracked;
use rustybuzz::{BufferFlags, UnicodeBuffer};
use ttf_parser::GlyphId;
use ttf_parser::math::{GlyphAssembly, GlyphConstruction, GlyphPart};
use typst_library::World;
use typst_library::diag::{At, HintedStrResult, SourceResult, bail, warning};
use typst_library::foundations::{Repr, StyleChain};
use typst_library::introspection::Tag;
use typst_library::layout::{
    Abs, Axes, Axis, Corner, Em, Frame, FrameItem, Point, Size, VAlignment,
};
use typst_library::math::{EquationElem, MathSize};
use typst_library::text::{
    Font, FontFamily, FontVariant, Glyph, TextElem, TextItem, features, language, variant,
};
use typst_library::visualize::Paint;
use typst_syntax::Span;
use typst_utils::{Get, default_math_class};
use unicode_math_class::MathClass;
use unicode_segmentation::UnicodeSegmentation;

use super::{MathContext, families};
use crate::inline::create_shape_plan;
use crate::modifiers::{FrameModifiers, FrameModify};

/// Maximum number of times extenders can be repeated.
const MAX_REPEATS: usize = 1024;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum MathFragment {
    Glyph(GlyphFragment),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Frame(FrameFragment),
    Spacing(Abs, bool),
    Space(Abs),
    Linebreak,
    Align,
    Tag(Tag),
}

impl MathFragment {
    pub fn size(&self) -> Size {
<<<<<<< HEAD
        Size::new(self.width(), self.height())
=======
        match self {
            Self::Glyph(glyph) => glyph.size,
            Self::Frame(fragment) => fragment.frame.size(),
            Self::Spacing(amount, _) => Size::with_x(*amount),
            Self::Space(amount) => Size::with_x(*amount),
            _ => Size::zero(),
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    pub fn width(&self) -> Abs {
        match self {
<<<<<<< HEAD
            Self::Glyph(glyph) => glyph.width,
            Self::Variant(variant) => variant.frame.width(),
=======
            Self::Glyph(glyph) => glyph.size.x,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.frame.width(),
            Self::Spacing(amount, _) => *amount,
            Self::Space(amount) => *amount,
            _ => Abs::zero(),
        }
    }

    pub fn height(&self) -> Abs {
        match self {
<<<<<<< HEAD
            Self::Glyph(glyph) => glyph.height(),
            Self::Variant(variant) => variant.frame.height(),
=======
            Self::Glyph(glyph) => glyph.size.y,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.frame.height(),
            _ => Abs::zero(),
        }
    }

    pub fn ascent(&self) -> Abs {
        match self {
<<<<<<< HEAD
            Self::Glyph(glyph) => glyph.ascent,
            Self::Variant(variant) => variant.frame.ascent(),
            Self::Frame(fragment) => fragment.frame.baseline(),
=======
            Self::Glyph(glyph) => glyph.ascent(),
            Self::Frame(fragment) => fragment.frame.ascent(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            _ => Abs::zero(),
        }
    }

    pub fn descent(&self) -> Abs {
        match self {
<<<<<<< HEAD
            Self::Glyph(glyph) => glyph.descent,
            Self::Variant(variant) => variant.frame.descent(),
=======
            Self::Glyph(glyph) => glyph.descent(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.frame.descent(),
            _ => Abs::zero(),
        }
    }

    pub fn is_ignorant(&self) -> bool {
        match self {
            Self::Frame(fragment) => fragment.ignorant,
            Self::Tag(_) => true,
            _ => false,
        }
    }

    pub fn class(&self) -> MathClass {
        match self {
            Self::Glyph(glyph) => glyph.class,
<<<<<<< HEAD
            Self::Variant(variant) => variant.class,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.class,
            Self::Spacing(_, _) => MathClass::Space,
            Self::Space(_) => MathClass::Space,
            Self::Linebreak => MathClass::Space,
            Self::Align => MathClass::Special,
            Self::Tag(_) => MathClass::Special,
        }
    }

    pub fn math_size(&self) -> Option<MathSize> {
        match self {
            Self::Glyph(glyph) => Some(glyph.math_size),
<<<<<<< HEAD
            Self::Variant(variant) => Some(variant.math_size),
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => Some(fragment.math_size),
            _ => None,
        }
    }

<<<<<<< HEAD
    pub fn font_size(&self) -> Option<Abs> {
        match self {
            Self::Glyph(glyph) => Some(glyph.font_size),
            Self::Variant(variant) => Some(variant.font_size),
=======
    #[inline]
    pub fn font(&self, ctx: &MathContext, styles: StyleChain) -> (Font, Abs) {
        (
            match self {
                Self::Glyph(glyph) => glyph.item.font.clone(),
                _ => ctx.font().clone(),
            },
            self.font_size().unwrap_or_else(|| styles.resolve(TextElem::size)),
        )
    }

    pub fn font_size(&self) -> Option<Abs> {
        match self {
            Self::Glyph(glyph) => Some(glyph.item.size),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => Some(fragment.font_size),
            _ => None,
        }
    }

    pub fn set_class(&mut self, class: MathClass) {
        match self {
            Self::Glyph(glyph) => glyph.class = class,
<<<<<<< HEAD
            Self::Variant(variant) => variant.class = class,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.class = class,
            _ => {}
        }
    }

    pub fn set_limits(&mut self, limits: Limits) {
        match self {
            Self::Glyph(glyph) => glyph.limits = limits,
<<<<<<< HEAD
            Self::Variant(variant) => variant.limits = limits,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.limits = limits,
            _ => {}
        }
    }

    pub fn is_spaced(&self) -> bool {
        if self.class() == MathClass::Fence {
            return true;
        }

        matches!(
            self,
            MathFragment::Frame(FrameFragment {
                spaced: true,
                class: MathClass::Normal | MathClass::Alphabetic,
                ..
            })
        )
    }

    pub fn is_text_like(&self) -> bool {
        match self {
            Self::Glyph(glyph) => !glyph.extended_shape,
<<<<<<< HEAD
            Self::Variant(variant) => !variant.extended_shape,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            MathFragment::Frame(frame) => frame.text_like,
            _ => false,
        }
    }

    pub fn italics_correction(&self) -> Abs {
        match self {
            Self::Glyph(glyph) => glyph.italics_correction,
<<<<<<< HEAD
            Self::Variant(variant) => variant.italics_correction,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.italics_correction,
            _ => Abs::zero(),
        }
    }

<<<<<<< HEAD
    pub fn accent_attach(&self) -> Abs {
        match self {
            Self::Glyph(glyph) => glyph.accent_attach,
            Self::Variant(variant) => variant.accent_attach,
            Self::Frame(fragment) => fragment.accent_attach,
            _ => self.width() / 2.0,
=======
    pub fn accent_attach(&self) -> (Abs, Abs) {
        match self {
            Self::Glyph(glyph) => glyph.accent_attach,
            Self::Frame(fragment) => fragment.accent_attach,
            _ => (self.width() / 2.0, self.width() / 2.0),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }

    pub fn into_frame(self) -> Frame {
        match self {
            Self::Glyph(glyph) => glyph.into_frame(),
<<<<<<< HEAD
            Self::Variant(variant) => variant.frame,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Frame(fragment) => fragment.frame,
            Self::Tag(tag) => {
                let mut frame = Frame::soft(Size::zero());
                frame.push(Point::zero(), FrameItem::Tag(tag));
                frame
            }
            _ => Frame::soft(self.size()),
        }
    }

    pub fn limits(&self) -> Limits {
        match self {
            MathFragment::Glyph(glyph) => glyph.limits,
<<<<<<< HEAD
            MathFragment::Variant(variant) => variant.limits,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            MathFragment::Frame(fragment) => fragment.limits,
            _ => Limits::Never,
        }
    }

<<<<<<< HEAD
    /// If no kern table is provided for a corner, a kerning amount of zero is
    /// assumed.
    pub fn kern_at_height(&self, ctx: &MathContext, corner: Corner, height: Abs) -> Abs {
        match self {
            Self::Glyph(glyph) => {
                kern_at_height(ctx, glyph.font_size, glyph.id, corner, height)
                    .unwrap_or_default()
=======
    pub fn fill(&self) -> Option<Paint> {
        match self {
            Self::Glyph(glyph) => Some(glyph.item.fill.clone()),
            _ => None,
        }
    }

    pub fn stretch_vertical(
        &mut self,
        ctx: &mut MathContext,
        height: Abs,
        short_fall: Abs,
    ) {
        if let Self::Glyph(glyph) = self {
            glyph.stretch_vertical(ctx, height, short_fall)
        }
    }

    pub fn stretch_horizontal(
        &mut self,
        ctx: &mut MathContext,
        width: Abs,
        short_fall: Abs,
    ) {
        if let Self::Glyph(glyph) = self {
            glyph.stretch_horizontal(ctx, width, short_fall)
        }
    }

    pub fn center_on_axis(&mut self) {
        if let Self::Glyph(glyph) = self {
            glyph.center_on_axis()
        }
    }

    /// If no kern table is provided for a corner, a kerning amount of zero is
    /// assumed.
    pub fn kern_at_height(&self, corner: Corner, height: Abs) -> Abs {
        match self {
            Self::Glyph(glyph) => {
                // For glyph assemblies we pick either the start or end glyph
                // depending on the corner.
                let is_vertical =
                    glyph.item.glyphs.iter().all(|glyph| glyph.y_advance != Em::zero());
                let glyph_index = match (is_vertical, corner) {
                    (true, Corner::TopLeft | Corner::TopRight) => {
                        glyph.item.glyphs.len() - 1
                    }
                    (false, Corner::TopRight | Corner::BottomRight) => {
                        glyph.item.glyphs.len() - 1
                    }
                    _ => 0,
                };

                kern_at_height(
                    &glyph.item.font,
                    GlyphId(glyph.item.glyphs[glyph_index].id),
                    corner,
                    Em::from_abs(height, glyph.item.size),
                )
                .unwrap_or_default()
                .at(glyph.item.size)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            }
            _ => Abs::zero(),
        }
    }
}

impl From<GlyphFragment> for MathFragment {
    fn from(glyph: GlyphFragment) -> Self {
        Self::Glyph(glyph)
    }
}

<<<<<<< HEAD
impl From<VariantFragment> for MathFragment {
    fn from(variant: VariantFragment) -> Self {
        Self::Variant(variant)
    }
}

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
impl From<FrameFragment> for MathFragment {
    fn from(fragment: FrameFragment) -> Self {
        Self::Frame(fragment)
    }
}

#[derive(Clone)]
pub struct GlyphFragment {
<<<<<<< HEAD
    pub id: GlyphId,
    pub c: char,
    pub font: Font,
    pub lang: Lang,
    pub region: Option<Region>,
    pub fill: Paint,
    pub shift: Abs,
    pub width: Abs,
    pub ascent: Abs,
    pub descent: Abs,
    pub italics_correction: Abs,
    pub accent_attach: Abs,
    pub font_size: Abs,
    pub class: MathClass,
    pub math_size: MathSize,
    pub span: Span,
    pub modifiers: FrameModifiers,
    pub limits: Limits,
    pub extended_shape: bool,
}

impl GlyphFragment {
    pub fn new(ctx: &MathContext, styles: StyleChain, c: char, span: Span) -> Self {
        let id = ctx.ttf.glyph_index(c).unwrap_or_default();
        let id = Self::adjust_glyph_index(ctx, id);
        Self::with_id(ctx, styles, c, id, span)
    }

    pub fn try_new(
=======
    // Text stuff.
    pub item: TextItem,
    pub base_glyph: Glyph,
    // Math stuff.
    pub size: Size,
    pub baseline: Option<Abs>,
    pub italics_correction: Abs,
    pub accent_attach: (Abs, Abs),
    pub math_size: MathSize,
    pub class: MathClass,
    pub limits: Limits,
    pub extended_shape: bool,
    pub mid_stretched: Option<bool>,
    // External frame stuff.
    pub modifiers: FrameModifiers,
    pub shift: Abs,
    pub align: Abs,
}

impl GlyphFragment {
    /// Calls `new` with the given character.
    pub fn new_char(
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        ctx: &MathContext,
        styles: StyleChain,
        c: char,
        span: Span,
<<<<<<< HEAD
    ) -> Option<Self> {
        let id = ctx.ttf.glyph_index(c)?;
        let id = Self::adjust_glyph_index(ctx, id);
        Some(Self::with_id(ctx, styles, c, id, span))
    }

    pub fn with_id(
        ctx: &MathContext,
        styles: StyleChain,
        c: char,
        id: GlyphId,
        span: Span,
    ) -> Self {
        let class = EquationElem::class_in(styles)
            .or_else(|| default_math_class(c))
            .unwrap_or(MathClass::Normal);

        let mut fragment = Self {
            id,
            c,
            font: ctx.font.clone(),
            lang: TextElem::lang_in(styles),
            region: TextElem::region_in(styles),
            fill: TextElem::fill_in(styles).as_decoration(),
            shift: TextElem::baseline_in(styles),
            font_size: TextElem::size_in(styles),
            math_size: EquationElem::size_in(styles),
            width: Abs::zero(),
            ascent: Abs::zero(),
            descent: Abs::zero(),
            limits: Limits::for_char(c),
            italics_correction: Abs::zero(),
            accent_attach: Abs::zero(),
            class,
            span,
            modifiers: FrameModifiers::get_in(styles),
            extended_shape: false,
        };
        fragment.set_id(ctx, id);
        fragment
    }

    /// Apply GSUB substitutions.
    fn adjust_glyph_index(ctx: &MathContext, id: GlyphId) -> GlyphId {
        if let Some(glyphwise_tables) = &ctx.glyphwise_tables {
            glyphwise_tables.iter().fold(id, |id, table| table.apply(id))
        } else {
            id
        }
=======
    ) -> SourceResult<Option<Self>> {
        Self::new(ctx.engine.world, styles, c.encode_utf8(&mut [0; 4]), span)
    }

    /// Selects a font to use and then shapes text.
    #[comemo::memoize]
    pub fn new(
        world: Tracked<dyn World + '_>,
        styles: StyleChain,
        text: &str,
        span: Span,
    ) -> SourceResult<Option<GlyphFragment>> {
        assert!(text.graphemes(true).count() == 1);

        let Some((c, font, mut glyph)) = shape(
            world,
            variant(styles),
            features(styles),
            language(styles),
            styles.get(TextElem::fallback),
            text,
            families(styles).collect(),
        )
        .at(span)?
        else {
            return Ok(None);
        };
        glyph.span.0 = span;

        let limits = Limits::for_char(c);
        let class = styles
            .get(EquationElem::class)
            .or_else(|| default_math_class(c))
            .unwrap_or(MathClass::Normal);

        let item = TextItem {
            font,
            size: styles.resolve(TextElem::size),
            fill: styles.get_ref(TextElem::fill).as_decoration(),
            stroke: styles.resolve(TextElem::stroke).map(|s| s.unwrap_or_default()),
            lang: styles.get(TextElem::lang),
            region: styles.get(TextElem::region),
            text: text.into(),
            glyphs: vec![glyph.clone()],
        };

        let mut fragment = Self {
            item,
            base_glyph: glyph,
            // Math
            math_size: styles.get(EquationElem::size),
            class,
            limits,
            mid_stretched: None,
            // Math in need of updating.
            extended_shape: false,
            italics_correction: Abs::zero(),
            accent_attach: (Abs::zero(), Abs::zero()),
            size: Size::zero(),
            baseline: None,
            // Misc
            align: Abs::zero(),
            shift: styles.resolve(TextElem::baseline),
            modifiers: FrameModifiers::get_in(styles),
        };
        fragment.update_glyph();
        Ok(Some(fragment))
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    /// Sets element id and boxes in appropriate way without changing other
    /// styles. This is used to replace the glyph with a stretch variant.
<<<<<<< HEAD
    pub fn set_id(&mut self, ctx: &MathContext, id: GlyphId) {
        let advance = ctx.ttf.glyph_hor_advance(id).unwrap_or_default();
        let italics = italics_correction(ctx, id, self.font_size).unwrap_or_default();
        let bbox = ctx.ttf.glyph_bounding_box(id).unwrap_or(Rect {
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,
        });

        let mut width = advance.scaled(ctx, self.font_size);
        let accent_attach =
            accent_attach(ctx, id, self.font_size).unwrap_or((width + italics) / 2.0);

        let extended_shape = is_extended_shape(ctx, id);
        if !extended_shape {
            width += italics;
        }

        self.id = id;
        self.width = width;
        self.ascent = bbox.y_max.scaled(ctx, self.font_size);
        self.descent = -bbox.y_min.scaled(ctx, self.font_size);
        self.italics_correction = italics;
        self.accent_attach = accent_attach;
        self.extended_shape = extended_shape;
    }

    pub fn height(&self) -> Abs {
        self.ascent + self.descent
    }

    pub fn into_variant(self) -> VariantFragment {
        VariantFragment {
            c: self.c,
            font_size: self.font_size,
            italics_correction: self.italics_correction,
            accent_attach: self.accent_attach,
            class: self.class,
            math_size: self.math_size,
            span: self.span,
            limits: self.limits,
            extended_shape: self.extended_shape,
            frame: self.into_frame(),
            mid_stretched: None,
        }
    }

    pub fn into_frame(self) -> Frame {
        let item = TextItem {
            font: self.font.clone(),
            size: self.font_size,
            fill: self.fill,
            lang: self.lang,
            region: self.region,
            text: self.c.into(),
            stroke: None,
            glyphs: vec![Glyph {
                id: self.id.0,
                x_advance: Em::from_length(self.width, self.font_size),
                x_offset: Em::zero(),
                range: 0..self.c.len_utf8() as u16,
                span: (self.span, 0),
            }],
        };
        let size = Size::new(self.width, self.ascent + self.descent);
        let mut frame = Frame::soft(size);
        frame.set_baseline(self.ascent);
        frame.push(Point::with_y(self.ascent + self.shift), FrameItem::Text(item));
=======
    pub fn update_glyph(&mut self) {
        let id = GlyphId(self.item.glyphs[0].id);

        let extended_shape = is_extended_shape(&self.item.font, id);
        let italics = italics_correction(&self.item.font, id).unwrap_or_default();
        let width = self.item.width();
        if !extended_shape {
            self.item.glyphs[0].x_advance += italics;
        }
        let italics = italics.at(self.item.size);

        let (ascent, descent) =
            ascent_descent(&self.item.font, id).unwrap_or((Em::zero(), Em::zero()));

        // The fallback for accents is half the width plus or minus the italics
        // correction. This is similar to how top and bottom attachments are
        // shifted. For bottom accents we do not use the accent attach of the
        // base as it is meant for top acccents.
        let top_accent_attach = accent_attach(&self.item.font, id)
            .map(|x| x.at(self.item.size))
            .unwrap_or((width + italics) / 2.0);
        let bottom_accent_attach = (width - italics) / 2.0;

        self.baseline = Some(ascent.at(self.item.size));
        self.size = Size::new(
            self.item.width(),
            ascent.at(self.item.size) + descent.at(self.item.size),
        );
        self.italics_correction = italics;
        self.accent_attach = (top_accent_attach, bottom_accent_attach);
        self.extended_shape = extended_shape;
    }

    // Reset a GlyphFragment's text field and math properties back to its
    // base_id's. This is used to return a glyph to its unstretched state.
    pub fn reset_glyph(&mut self) {
        self.align = Abs::zero();
        self.item.glyphs = vec![self.base_glyph.clone()];
        self.update_glyph();
    }

    pub fn baseline(&self) -> Abs {
        self.ascent()
    }

    /// The distance from the baseline to the top of the frame.
    pub fn ascent(&self) -> Abs {
        self.baseline.unwrap_or(self.size.y)
    }

    /// The distance from the baseline to the bottom of the frame.
    pub fn descent(&self) -> Abs {
        self.size.y - self.ascent()
    }

    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::soft(self.size);
        frame.set_baseline(self.baseline());
        frame.push(
            Point::with_y(self.ascent() + self.shift + self.align),
            FrameItem::Text(self.item),
        );
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        frame.modify(&self.modifiers);
        frame
    }

<<<<<<< HEAD
    pub fn make_script_size(&mut self, ctx: &MathContext) {
        let alt_id =
            ctx.ssty_table.as_ref().and_then(|ssty| ssty.try_apply(self.id, None));
        if let Some(alt_id) = alt_id {
            self.set_id(ctx, alt_id);
        }
    }

    pub fn make_script_script_size(&mut self, ctx: &MathContext) {
        let alt_id = ctx.ssty_table.as_ref().and_then(|ssty| {
            // We explicitly request to apply the alternate set with value 1,
            // as opposed to the default value in ssty, as the former
            // corresponds to second level scripts and the latter corresponds
            // to first level scripts.
            ssty.try_apply(self.id, Some(1))
                .or_else(|| ssty.try_apply(self.id, None))
        });
        if let Some(alt_id) = alt_id {
            self.set_id(ctx, alt_id);
        }
    }

    pub fn make_dotless_form(&mut self, ctx: &MathContext) {
        let alt_id =
            ctx.dtls_table.as_ref().and_then(|dtls| dtls.try_apply(self.id, None));
        if let Some(alt_id) = alt_id {
            self.set_id(ctx, alt_id);
        }
    }

    pub fn make_flattened_accent_form(&mut self, ctx: &MathContext) {
        let alt_id =
            ctx.flac_table.as_ref().and_then(|flac| flac.try_apply(self.id, None));
        if let Some(alt_id) = alt_id {
            self.set_id(ctx, alt_id);
        }
    }

    /// Try to stretch a glyph to a desired height.
    pub fn stretch_vertical(
        self,
        ctx: &mut MathContext,
        height: Abs,
        short_fall: Abs,
    ) -> VariantFragment {
        stretch_glyph(ctx, self, height, short_fall, Axis::Y)
=======
    /// Try to stretch a glyph to a desired height.
    pub fn stretch_vertical(
        &mut self,
        ctx: &mut MathContext,
        height: Abs,
        short_fall: Abs,
    ) {
        self.stretch(ctx, height, short_fall, Axis::Y)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    /// Try to stretch a glyph to a desired width.
    pub fn stretch_horizontal(
<<<<<<< HEAD
        self,
        ctx: &mut MathContext,
        width: Abs,
        short_fall: Abs,
    ) -> VariantFragment {
        stretch_glyph(ctx, self, width, short_fall, Axis::X)
=======
        &mut self,
        ctx: &mut MathContext,
        width: Abs,
        short_fall: Abs,
    ) {
        self.stretch(ctx, width, short_fall, Axis::X)
    }

    /// Try to stretch a glyph to a desired width or height.
    ///
    /// The resulting frame may not have the exact desired width or height.
    pub fn stretch(
        &mut self,
        ctx: &mut MathContext,
        target: Abs,
        short_fall: Abs,
        axis: Axis,
    ) {
        self.reset_glyph();

        // If the base glyph is good enough, use it.
        let mut advance = self.size.get(axis);
        if axis == Axis::X && !self.extended_shape {
            // For consistency, we subtract the italics correction from the
            // glyph's width if it was added in `update_glyph`.
            advance -= self.italics_correction;
        }
        let short_target = target - short_fall;
        if short_target <= advance {
            return;
        }

        let id = GlyphId(self.item.glyphs[0].id);
        let font = self.item.font.clone();
        let Some(construction) = glyph_construction(&font, id, axis) else { return };

        // Search for a pre-made variant with a good advance.
        let mut best_id = id;
        let mut best_advance = advance;
        for variant in construction.variants {
            best_id = variant.variant_glyph;
            best_advance =
                self.item.font.to_em(variant.advance_measurement).at(self.item.size);
            if short_target <= best_advance {
                break;
            }
        }

        // This is either good or the best we've got.
        if short_target <= best_advance || construction.assembly.is_none() {
            self.item.glyphs[0].id = best_id.0;
            self.item.glyphs[0].x_advance =
                self.item.font.x_advance(best_id.0).unwrap_or_default();
            self.item.glyphs[0].x_offset = Em::zero();
            self.item.glyphs[0].y_advance =
                self.item.font.y_advance(best_id.0).unwrap_or_default();
            self.item.glyphs[0].y_offset = Em::zero();
            self.update_glyph();
            return;
        }

        // Assemble from parts.
        let assembly = construction.assembly.unwrap();
        let min_overlap = min_connector_overlap(&self.item.font)
            .unwrap_or_default()
            .at(self.item.size);
        assemble(ctx, self, assembly, min_overlap, target, axis);
    }

    /// Vertically adjust the fragment's frame so that it is centered
    /// on the axis.
    pub fn center_on_axis(&mut self) {
        self.align_on_axis(VAlignment::Horizon);
    }

    /// Vertically adjust the fragment's frame so that it is aligned
    /// to the given alignment on the axis.
    pub fn align_on_axis(&mut self, align: VAlignment) {
        let h = self.size.y;
        let axis = self.item.font.math().axis_height.at(self.item.size);
        self.align += self.baseline();
        self.baseline = Some(align.inv().position(h + axis * 2.0));
        self.align -= self.baseline();
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

impl Debug for GlyphFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
<<<<<<< HEAD
        write!(f, "GlyphFragment({:?})", self.c)
    }
}

#[derive(Clone)]
pub struct VariantFragment {
    pub c: char,
    pub italics_correction: Abs,
    pub accent_attach: Abs,
    pub frame: Frame,
    pub font_size: Abs,
    pub class: MathClass,
    pub math_size: MathSize,
    pub span: Span,
    pub limits: Limits,
    pub mid_stretched: Option<bool>,
    pub extended_shape: bool,
}

impl VariantFragment {
    /// Vertically adjust the fragment's frame so that it is centered
    /// on the axis.
    pub fn center_on_axis(&mut self, ctx: &MathContext) {
        self.align_on_axis(ctx, VAlignment::Horizon)
    }

    /// Vertically adjust the fragment's frame so that it is aligned
    /// to the given alignment on the axis.
    pub fn align_on_axis(&mut self, ctx: &MathContext, align: VAlignment) {
        let h = self.frame.height();
        let axis = ctx.constants.axis_height().scaled(ctx, self.font_size);
        self.frame.set_baseline(align.inv().position(h + axis * 2.0));
    }
}

impl Debug for VariantFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "VariantFragment({:?})", self.c)
=======
        write!(f, "GlyphFragment({:?})", self.item.text)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

#[derive(Debug, Clone)]
pub struct FrameFragment {
    pub frame: Frame,
    pub font_size: Abs,
    pub class: MathClass,
    pub math_size: MathSize,
    pub limits: Limits,
    pub spaced: bool,
    pub base_ascent: Abs,
<<<<<<< HEAD
    pub italics_correction: Abs,
    pub accent_attach: Abs,
=======
    pub base_descent: Abs,
    pub italics_correction: Abs,
    pub accent_attach: (Abs, Abs),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub text_like: bool,
    pub ignorant: bool,
}

impl FrameFragment {
    pub fn new(styles: StyleChain, frame: Frame) -> Self {
        let base_ascent = frame.ascent();
<<<<<<< HEAD
        let accent_attach = frame.width() / 2.0;
        Self {
            frame: frame.modified(&FrameModifiers::get_in(styles)),
            font_size: TextElem::size_in(styles),
            class: EquationElem::class_in(styles).unwrap_or(MathClass::Normal),
            math_size: EquationElem::size_in(styles),
            limits: Limits::Never,
            spaced: false,
            base_ascent,
            italics_correction: Abs::zero(),
            accent_attach,
=======
        let base_descent = frame.descent();
        let accent_attach = frame.width() / 2.0;
        Self {
            frame: frame.modified(&FrameModifiers::get_in(styles)),
            font_size: styles.resolve(TextElem::size),
            class: styles.get(EquationElem::class).unwrap_or(MathClass::Normal),
            math_size: styles.get(EquationElem::size),
            limits: Limits::Never,
            spaced: false,
            base_ascent,
            base_descent,
            italics_correction: Abs::zero(),
            accent_attach: (accent_attach, accent_attach),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            text_like: false,
            ignorant: false,
        }
    }

    pub fn with_class(self, class: MathClass) -> Self {
        Self { class, ..self }
    }

    pub fn with_limits(self, limits: Limits) -> Self {
        Self { limits, ..self }
    }

    pub fn with_spaced(self, spaced: bool) -> Self {
        Self { spaced, ..self }
    }

    pub fn with_base_ascent(self, base_ascent: Abs) -> Self {
        Self { base_ascent, ..self }
    }

<<<<<<< HEAD
=======
    pub fn with_base_descent(self, base_descent: Abs) -> Self {
        Self { base_descent, ..self }
    }

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn with_italics_correction(self, italics_correction: Abs) -> Self {
        Self { italics_correction, ..self }
    }

<<<<<<< HEAD
    pub fn with_accent_attach(self, accent_attach: Abs) -> Self {
=======
    pub fn with_accent_attach(self, accent_attach: (Abs, Abs)) -> Self {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        Self { accent_attach, ..self }
    }

    pub fn with_text_like(self, text_like: bool) -> Self {
        Self { text_like, ..self }
    }

    pub fn with_ignorant(self, ignorant: bool) -> Self {
        Self { ignorant, ..self }
    }
}

<<<<<<< HEAD
/// Look up the italics correction for a glyph.
fn italics_correction(ctx: &MathContext, id: GlyphId, font_size: Abs) -> Option<Abs> {
    Some(
        ctx.table
            .glyph_info?
            .italic_corrections?
            .get(id)?
            .scaled(ctx, font_size),
    )
}

/// Loop up the top accent attachment position for a glyph.
fn accent_attach(ctx: &MathContext, id: GlyphId, font_size: Abs) -> Option<Abs> {
    Some(
        ctx.table
            .glyph_info?
            .top_accent_attachments?
            .get(id)?
            .scaled(ctx, font_size),
    )
}

/// Look up whether a glyph is an extended shape.
fn is_extended_shape(ctx: &MathContext, id: GlyphId) -> bool {
    ctx.table
        .glyph_info
        .and_then(|info| info.extended_shapes)
        .and_then(|info| info.get(id))
=======
fn ascent_descent(font: &Font, id: GlyphId) -> Option<(Em, Em)> {
    let bbox = font.ttf().glyph_bounding_box(id)?;
    Some((font.to_em(bbox.y_max), -font.to_em(bbox.y_min)))
}

/// Look up the italics correction for a glyph.
fn italics_correction(font: &Font, id: GlyphId) -> Option<Em> {
    font.ttf()
        .tables()
        .math?
        .glyph_info?
        .italic_corrections?
        .get(id)
        .map(|value| font.to_em(value.value))
}

/// Loop up the top accent attachment position for a glyph.
fn accent_attach(font: &Font, id: GlyphId) -> Option<Em> {
    font.ttf()
        .tables()
        .math?
        .glyph_info?
        .top_accent_attachments?
        .get(id)
        .map(|value| font.to_em(value.value))
}

/// Look up whether a glyph is an extended shape.
fn is_extended_shape(font: &Font, id: GlyphId) -> bool {
    font.ttf()
        .tables()
        .math
        .and_then(|math| math.glyph_info)
        .and_then(|glyph_info| glyph_info.extended_shapes)
        .and_then(|coverage| coverage.get(id))
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        .is_some()
}

/// Look up a kerning value at a specific corner and height.
<<<<<<< HEAD
fn kern_at_height(
    ctx: &MathContext,
    font_size: Abs,
    id: GlyphId,
    corner: Corner,
    height: Abs,
) -> Option<Abs> {
    let kerns = ctx.table.glyph_info?.kern_infos?.get(id)?;
=======
fn kern_at_height(font: &Font, id: GlyphId, corner: Corner, height: Em) -> Option<Em> {
    let kerns = font.ttf().tables().math?.glyph_info?.kern_infos?.get(id)?;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let kern = match corner {
        Corner::TopLeft => kerns.top_left,
        Corner::TopRight => kerns.top_right,
        Corner::BottomRight => kerns.bottom_right,
        Corner::BottomLeft => kerns.bottom_left,
    }?;

    let mut i = 0;
<<<<<<< HEAD
    while i < kern.count() && height > kern.height(i)?.scaled(ctx, font_size) {
        i += 1;
    }

    Some(kern.kern(i)?.scaled(ctx, font_size))
=======
    while i < kern.count() && height > font.to_em(kern.height(i)?.value) {
        i += 1;
    }

    Some(font.to_em(kern.kern(i)?.value))
}

pub fn stretch_axes(font: &Font, id: u16) -> Axes<bool> {
    let id = GlyphId(id);
    let horizontal = font
        .ttf()
        .tables()
        .math
        .and_then(|math| math.variants)
        .and_then(|variants| variants.horizontal_constructions.get(id))
        .is_some();
    let vertical = font
        .ttf()
        .tables()
        .math
        .and_then(|math| math.variants)
        .and_then(|variants| variants.vertical_constructions.get(id))
        .is_some();

    Axes::new(horizontal, vertical)
}

fn min_connector_overlap(font: &Font) -> Option<Em> {
    font.ttf()
        .tables()
        .math?
        .variants
        .map(|variants| font.to_em(variants.min_connector_overlap))
}

fn glyph_construction(
    font: &Font,
    id: GlyphId,
    axis: Axis,
) -> Option<GlyphConstruction<'_>> {
    font.ttf()
        .tables()
        .math?
        .variants
        .map(|variants| match axis {
            Axis::X => variants.horizontal_constructions,
            Axis::Y => variants.vertical_constructions,
        })?
        .get(id)
}

/// Assemble a glyph from parts.
fn assemble(
    ctx: &mut MathContext,
    base: &mut GlyphFragment,
    assembly: GlyphAssembly,
    min_overlap: Abs,
    target: Abs,
    axis: Axis,
) {
    // Determine the number of times the extenders need to be repeated as well
    // as a ratio specifying how much to spread the parts apart
    // (0 = maximal overlap, 1 = minimal overlap).
    let mut full;
    let mut ratio;
    let mut repeat = 0;
    loop {
        full = Abs::zero();
        ratio = 0.0;

        let mut parts = parts(assembly, repeat).peekable();
        let mut growable = Abs::zero();

        while let Some(part) = parts.next() {
            let mut advance = base.item.font.to_em(part.full_advance).at(base.item.size);
            if let Some(next) = parts.peek() {
                let max_overlap = base
                    .item
                    .font
                    .to_em(part.end_connector_length.min(next.start_connector_length))
                    .at(base.item.size);
                if max_overlap < min_overlap {
                    // This condition happening is indicative of a bug in the
                    // font.
                    ctx.engine.sink.warn(warning!(
                       base.item.glyphs[0].span.0,
                       "glyph has assembly parts with overlap less than minConnectorOverlap";
                       hint: "its rendering may appear broken - this is probably a font bug";
                       hint: "please file an issue at https://github.com/typst/typst/issues"
                    ));
                }

                advance -= max_overlap;
                // In case we have that max_overlap < min_overlap, ensure we
                // don't decrease the value of growable.
                growable += (max_overlap - min_overlap).max(Abs::zero());
            }

            full += advance;
        }

        if full < target {
            let delta = target - full;
            ratio = (delta / growable).min(1.0);
            full += ratio * growable;
        }

        if target <= full || repeat >= MAX_REPEATS {
            break;
        }

        repeat += 1;
    }

    let mut glyphs = vec![];
    let mut parts = parts(assembly, repeat).peekable();
    while let Some(part) = parts.next() {
        let mut advance = base.item.font.to_em(part.full_advance).at(base.item.size);
        if let Some(next) = parts.peek() {
            let max_overlap = base
                .item
                .font
                .to_em(part.end_connector_length.min(next.start_connector_length))
                .at(base.item.size);
            advance -= max_overlap;
            advance += ratio * (max_overlap - min_overlap);
        }
        let (x_advance, y_advance, y_offset) = match axis {
            Axis::X => (Em::from_abs(advance, base.item.size), Em::zero(), Em::zero()),
            Axis::Y => (
                Em::zero(),
                Em::from_abs(advance, base.item.size),
                // Glyph parts used in vertical assemblies are typically aligned
                // at the vertical origin. This way, they combine properly when
                // drawn consecutively, as required by the MATH table spec.
                //
                // However, in some fonts, they aren't. To still have them align
                // properly, we are vertically offsetting such glyphs by their
                // bounding-box computed descent. (Positive descent means that
                // a glyph extends below the baseline and then we must move it
                // up for it to align properly. `y_advance` is Y-up, so that
                // matches up.)
                ascent_descent(&base.item.font, part.glyph_id)
                    .map(|x| x.1)
                    .unwrap_or_default(),
            ),
        };
        glyphs.push(Glyph {
            id: part.glyph_id.0,
            x_advance,
            x_offset: Em::zero(),
            y_advance,
            y_offset,
            ..base.item.glyphs[0].clone()
        });
    }

    match axis {
        Axis::X => base.size.x = full,
        Axis::Y => {
            base.baseline = None;
            base.size.y = full;
            base.size.x = glyphs
                .iter()
                .map(|glyph| base.item.font.x_advance(glyph.id).unwrap_or_default())
                .max()
                .unwrap_or_default()
                .at(base.item.size);
        }
    }

    base.item.glyphs = glyphs;
    base.italics_correction = base
        .item
        .font
        .to_em(assembly.italics_correction.value)
        .at(base.item.size);
    if axis == Axis::X {
        base.accent_attach = (full / 2.0, full / 2.0);
    }
    base.mid_stretched = None;
    base.extended_shape = true;
}

/// Return an iterator over the assembly's parts with extenders repeated the
/// specified number of times.
fn parts(
    assembly: GlyphAssembly<'_>,
    repeat: usize,
) -> impl Iterator<Item = GlyphPart> + '_ {
    assembly.parts.into_iter().flat_map(move |part| {
        let count = if part.part_flags.extender() { repeat } else { 1 };
        std::iter::repeat_n(part, count)
    })
}

pub fn has_dtls_feat(font: &Font) -> bool {
    font.ttf()
        .tables()
        .gsub
        .and_then(|gsub| gsub.features.index(ttf_parser::Tag::from_bytes(b"dtls")))
        .is_some()
}

#[comemo::memoize]
fn shape(
    world: Tracked<dyn World + '_>,
    variant: FontVariant,
    features: Vec<rustybuzz::Feature>,
    language: rustybuzz::Language,
    fallback: bool,
    text: &str,
    families: Vec<&FontFamily>,
) -> HintedStrResult<Option<(char, Font, Glyph)>> {
    let mut used = vec![];
    let buffer = UnicodeBuffer::new();
    shape_glyph(
        world,
        &mut used,
        buffer,
        variant,
        features,
        language,
        fallback,
        text,
        families.into_iter(),
    )
}

#[allow(clippy::too_many_arguments)]
fn shape_glyph<'a>(
    world: Tracked<'a, dyn World + 'a>,
    used: &mut Vec<Font>,
    mut buffer: rustybuzz::UnicodeBuffer,
    variant: FontVariant,
    features: Vec<rustybuzz::Feature>,
    language: rustybuzz::Language,
    fallback: bool,
    text: &str,
    mut families: impl Iterator<Item = &'a FontFamily> + Clone,
) -> HintedStrResult<Option<(char, Font, Glyph)>> {
    // Find the next available family.
    let book = world.book();
    let mut selection = None;
    let mut covers = None;
    for family in families.by_ref() {
        selection = book
            .select(family.as_str(), variant)
            .and_then(|id| world.font(id))
            .filter(|font| !used.contains(font));
        if selection.is_some() {
            covers = family.covers();
            break;
        }
    }

    // Do font fallback if the families are exhausted and fallback is enabled.
    if selection.is_none() && fallback {
        let first = used.first().map(Font::info);
        selection = book
            .select_fallback(first, variant, text)
            .and_then(|id| world.font(id))
            .filter(|font| !used.contains(font))
    }

    // Extract the font id or shape notdef glyphs if we couldn't find any font.
    let Some(font) = selection else {
        if let Some(font) = used.first().cloned() {
            // Shape tofu.
            let glyph = Glyph {
                id: 0,
                x_advance: font.x_advance(0).unwrap_or_default(),
                x_offset: Em::zero(),
                y_advance: Em::zero(),
                y_offset: Em::zero(),
                range: 0..text.len().saturating_as(),
                span: (Span::detached(), 0),
            };
            let c = text.chars().next().unwrap();
            return Ok(Some((c, font, glyph)));
        }
        return Ok(None);
    };

    // This font has been exhausted and will not be used again.
    if covers.is_none() {
        used.push(font.clone());
    }

    buffer.push_str(text);
    buffer.set_language(language.clone());
    // TODO: Use `rustybuzz::script::MATH` once
    // https://github.com/harfbuzz/rustybuzz/pull/165 is released.
    buffer.set_script(
        rustybuzz::Script::from_iso15924_tag(ttf_parser::Tag::from_bytes(b"math"))
            .unwrap(),
    );
    buffer.set_direction(rustybuzz::Direction::LeftToRight);
    buffer.set_flags(BufferFlags::REMOVE_DEFAULT_IGNORABLES);

    let plan = create_shape_plan(
        &font,
        buffer.direction(),
        buffer.script(),
        buffer.language().as_ref(),
        &features,
    );

    let buffer = rustybuzz::shape_with_plan(font.rusty(), &plan, buffer);
    match buffer.len() {
        0 => return Ok(None),
        1 => {}
        // TODO: Deal with multiple glyphs.
        _ => bail!(
            "shaping the text `{}` yielded more than one glyph", text.repr();
            hint: "please report this as a bug",
        ),
    }

    let info = buffer.glyph_infos()[0];
    let pos = buffer.glyph_positions()[0];
    let cluster = info.cluster as usize;
    let end = text[cluster..]
        .char_indices()
        .nth(1)
        .map(|(i, _)| i)
        .unwrap_or(text.len());

    if info.glyph_id != 0 && covers.is_none_or(|cov| cov.is_match(&text[cluster..end])) {
        let glyph = Glyph {
            id: info.glyph_id as u16,
            x_advance: font.to_em(pos.x_advance),
            x_offset: font.to_em(pos.x_offset),
            y_advance: font.to_em(pos.y_advance),
            y_offset: font.to_em(pos.y_offset),
            range: 0..text.len().saturating_as(),
            span: (Span::detached(), 0),
        };
        let c = text[cluster..].chars().next().unwrap();
        Ok(Some((c, font, glyph)))
    } else {
        shape_glyph(
            world,
            used,
            buffer.clear(),
            variant,
            features,
            language,
            fallback,
            text,
            families,
        )
    }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Describes in which situation a frame should use limits for attachments.
#[derive(Debug, Copy, Clone)]
pub enum Limits {
    /// Always scripts.
    Never,
    /// Display limits only in `display` math.
    Display,
    /// Always limits.
    Always,
}

impl Limits {
    /// The default limit configuration if the given character is the base.
    pub fn for_char(c: char) -> Self {
        match default_math_class(c) {
            Some(MathClass::Large) => {
                if is_integral_char(c) {
                    Limits::Never
                } else {
                    Limits::Display
                }
            }
            Some(MathClass::Relation) => Limits::Always,
            _ => Limits::Never,
        }
    }

    /// The default limit configuration for a math class.
    pub fn for_class(class: MathClass) -> Self {
        match class {
            MathClass::Large => Self::Display,
            MathClass::Relation => Self::Always,
            _ => Self::Never,
        }
    }

    /// Whether limits should be displayed in this context.
    pub fn active(&self, styles: StyleChain) -> bool {
        match self {
            Self::Always => true,
<<<<<<< HEAD
            Self::Display => EquationElem::size_in(styles) == MathSize::Display,
=======
            Self::Display => styles.get(EquationElem::size) == MathSize::Display,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            Self::Never => false,
        }
    }
}

/// Determines if the character is one of a variety of integral signs.
fn is_integral_char(c: char) -> bool {
    ('∫'..='∳').contains(&c) || ('⨋'..='⨜').contains(&c)
}
<<<<<<< HEAD

/// An OpenType substitution table that is applicable to glyph-wise substitutions.
pub enum GlyphwiseSubsts<'a> {
    Single(SingleSubstitution<'a>),
    Alternate(AlternateSubstitution<'a>, u32),
}

impl<'a> GlyphwiseSubsts<'a> {
    pub fn new(gsub: Option<LayoutTable<'a>>, feature: Feature) -> Option<Self> {
        let gsub = gsub?;
        let table = gsub
            .features
            .find(feature.tag)
            .and_then(|feature| feature.lookup_indices.get(0))
            .and_then(|index| gsub.lookups.get(index))?;
        let table = table.subtables.get::<SubstitutionSubtable>(0)?;
        match table {
            SubstitutionSubtable::Single(single_glyphs) => {
                Some(Self::Single(single_glyphs))
            }
            SubstitutionSubtable::Alternate(alt_glyphs) => {
                Some(Self::Alternate(alt_glyphs, feature.value))
            }
            _ => None,
        }
    }

    pub fn try_apply(
        &self,
        glyph_id: GlyphId,
        alt_value: Option<u32>,
    ) -> Option<GlyphId> {
        match self {
            Self::Single(single) => match single {
                SingleSubstitution::Format1 { coverage, delta } => coverage
                    .get(glyph_id)
                    .map(|_| GlyphId(glyph_id.0.wrapping_add(*delta as u16))),
                SingleSubstitution::Format2 { coverage, substitutes } => {
                    coverage.get(glyph_id).and_then(|idx| substitutes.get(idx))
                }
            },
            Self::Alternate(alternate, value) => alternate
                .coverage
                .get(glyph_id)
                .and_then(|idx| alternate.alternate_sets.get(idx))
                .and_then(|set| set.alternates.get(alt_value.unwrap_or(*value) as u16)),
        }
    }

    pub fn apply(&self, glyph_id: GlyphId) -> GlyphId {
        self.try_apply(glyph_id, None).unwrap_or(glyph_id)
    }
}
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
