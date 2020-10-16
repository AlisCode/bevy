use std::{cell::RefCell, collections::HashMap};

use ab_glyph::FontArc;
use bevy_asset::{Assets, Handle};
use bevy_math::Vec2;
use glyph_brush::{FontId, GlyphBrush, GlyphBrushBuilder};

use crate::Font;

pub struct TextPipeline {
    pub draw_brush: RefCell<GlyphBrush<FontArc>>,
    pub measure_brush: RefCell<GlyphBrush<FontArc>>,
    pub map_font_id: HashMap<Handle<Font>, FontId>,
}

impl Default for TextPipeline {
    fn default() -> Self {
        let draw_brush = GlyphBrushBuilder::using_fonts::<FontArc>(vec![]).build();
        let draw_brush = RefCell::new(draw_brush);
        let measure_brush = GlyphBrushBuilder::using_fonts::<FontArc>(vec![]).build();
        let measure_brush = RefCell::new(measure_brush);
        let map_font_id = Default::default();
        TextPipeline {
            measure_brush,
            draw_brush,
            map_font_id,
        }
    }
}

impl TextPipeline {
    fn measure(
        &self,
        font_handle: &Handle<Font>,
        font_storage: &Assets<Font>,
        contents: &str,
        size: f32,
        bounds: Vec2,
    ) -> Option<(f32, f32)> {
        use glyph_brush::GlyphCruncher;
        let font = font_storage.get(font_handle)?;
        let font_id = self.get_or_insert_font_id(font_handle, font);

        let section = glyph_brush::Section {
            bounds: (bounds.x(), bounds.y()),
            text: vec![glyph_brush::Text {
                text: contents,
                scale: size.into(),
                font_id,
                extra: glyph_brush::Extra::default(),
            }],
            // todo: handle Layout (h_align, v_align)
            ..Default::default()
        };

        self.measure_brush
            .borrow_mut()
            .glyph_bounds(section)
            .map(|bounds| (bounds.width().ceil(), bounds.height().ceil()))
    }

    pub fn get_or_insert_font_id(&self, handle: &Handle<Font>, font: &Font) -> FontId {
        if let Some(font_id) = self.map_font_id.get(handle) {
            return font_id.clone();
        }

        let _ = self.draw_brush.borrow_mut().add_font(font.font.clone());
        self.measure_brush.borrow_mut().add_font(font.font.clone())
    }
}
