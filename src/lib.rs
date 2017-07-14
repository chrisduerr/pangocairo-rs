// Copyright 2017, Reizner Evgeniy <razrfalcon@gmail.com>.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate pangocairo_sys as ffi;
extern crate cairo;
extern crate pango;
extern crate pango_sys;
#[macro_use]
extern crate glib;
extern crate gobject_sys as gobject_ffi;

use glib::translate::*;

mod font_map;

pub use font_map::FontMap;

//glib_wrapper! {
//    pub struct Font(Object<ffi::PangoCairoFont>);
//
//    match fn {
//        get_type => || ffi::pango_cairo_font_get_type(),
//    }
//}

//impl Font {
//    fn get_scaled_font(&self) -> cairo::ScaledFont;
//}

//pub struct ShapeRendererFunc(...);


pub trait PangoContextExt {
    fn get_font_options(&self) -> cairo::FontOptions;
    fn set_font_options(&self, options: cairo::FontOptions);
    fn get_resolution(&self) -> f64;
    fn set_resolution(&self, dpi: f64);
    // fn get_shape_renderer(&self, data: *mut gpointer) -> ShapeRendererFunc;
    // fn set_shape_renderer(&self, func: ShapeRendererFunc, data: gpointer, dnotify: glib::GDestroyNotify);
}

impl PangoContextExt for pango::Context {
    fn get_font_options(&self) -> cairo::FontOptions {
        unsafe {
            cairo::FontOptions::from_raw_full(ffi::pango_cairo_context_get_font_options(self.to_glib_none().0))
        }
    }

    fn set_font_options(&self, options: cairo::FontOptions) {
        unsafe {
            ffi::pango_cairo_context_set_font_options(self.to_glib_none().0, options.to_raw_none())
        }
    }

    fn get_resolution(&self) -> f64 {
        unsafe { ffi::pango_cairo_context_get_resolution(self.to_glib_none().0) }
    }

    fn set_resolution(&self, dpi: f64) {
        unsafe { ffi::pango_cairo_context_set_resolution(self.to_glib_none().0, dpi); }
    }

    // ffi::pango_cairo_context_get_shape_renderer
    // ffi::pango_cairo_context_set_shape_renderer
}


pub trait CairoContextExt {
    fn create_pango_context(&self) -> pango::Context;
    fn update_pango_context(&self, context: &pango::Context);
    fn create_pango_layout(&self) -> pango::Layout;
    fn show_pango_layout(&self, layout: &pango::Layout);
    fn update_pango_layout(&self, layout: &pango::Layout);
    fn pango_layout_path(&self, layout: &pango::Layout);
    fn pango_layout_line_path(&self, line: &pango::LayoutLine);
    fn show_pango_layout_line(&self, line: &pango::LayoutLine);
    fn error_underline_path(&self, x: f64, y: f64, width: f64, height: f64);
    fn show_error_underline(&self, x: f64, y: f64, width: f64, height: f64);
    // fn show_glyph_item(&self, text: &str, glyph_item: /*unimplemented*/ pango::GlyphItem);
    // fn glyph_string_path(&self, font: /*unimplemented*/ pango::Font, glyphs: pango::GlyphString);
    // fn show_glyph_string(&self, font: /*unimplemented*/ pango::Font, glyphs: pango::GlyphString);
}

impl CairoContextExt for cairo::Context {
    fn create_pango_context(&self) -> pango::Context {
        unsafe { from_glib_full(ffi::pango_cairo_create_context(self.to_glib_none().0)) }
    }

    fn update_pango_context(&self, context: &pango::Context) {
        unsafe { ffi::pango_cairo_update_context(self.to_glib_none().0, context.to_glib_none().0); };
    }

    fn create_pango_layout(&self) -> pango::Layout {
        unsafe { from_glib_full(ffi::pango_cairo_create_layout(self.to_glib_none().0)) }
    }

    fn show_pango_layout(&self, layout: &pango::Layout) {
        unsafe { ffi::pango_cairo_show_layout(self.to_glib_none().0, layout.to_glib_none().0); };
    }

    fn update_pango_layout(&self, layout: &pango::Layout) {
        unsafe { ffi::pango_cairo_update_layout(self.to_glib_none().0, layout.to_glib_none().0); };
    }

    fn pango_layout_path(&self, layout: &pango::Layout) {
        unsafe { ffi::pango_cairo_layout_path(self.to_glib_none().0, layout.to_glib_none().0); };
    }

    fn show_pango_layout_line(&self, line: &pango::LayoutLine) {
        unsafe { ffi::pango_cairo_show_layout_line(self.to_glib_none().0, line.to_glib_none().0); };
    }

    fn pango_layout_line_path(&self, line: &pango::LayoutLine) {
        unsafe { ffi::pango_cairo_layout_line_path(self.to_glib_none().0, line.to_glib_none().0); };
    }

    fn error_underline_path(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe { ffi::pango_cairo_error_underline_path(self.to_glib_none().0, x, y, width, height); };
    }

    fn show_error_underline(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe { ffi::pango_cairo_show_error_underline(self.to_glib_none().0, x, y, width, height); };
    }

    // ffi::pango_cairo_show_glyph_item
    // ffi::pango_cairo_glyph_string_path
    // ffi::pango_cairo_show_glyph_string
}
