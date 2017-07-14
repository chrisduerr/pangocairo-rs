use cairo;
use ffi;
use glib::translate::*;
use gobject_ffi;
use pango;
use pango_sys;

glib_wrapper! {
    pub struct FontMap(Object<ffi::PangoCairoFontMap>): [pango::FontMap => pango_sys::PangoFontMap];

    match fn {
        get_type => || ffi::pango_cairo_font_map_get_type(),
    }
}

impl FontMap {
    pub fn new() -> FontMap {
        unsafe {
            from_glib_full(ffi::pango_cairo_font_map_new() as *mut ffi::PangoCairoFontMap)
        }
    }

    pub fn from_font_type(fonttype: cairo::FontType) -> Option<FontMap> {
        unsafe {
            from_glib_full(ffi::pango_cairo_font_map_new_for_font_type(fonttype)
                           as *mut ffi::PangoCairoFontMap)
        }
    }

    pub fn get_default() -> FontMap {
        unsafe {
            from_glib_none(ffi::pango_cairo_font_map_get_default() as *mut ffi::PangoCairoFontMap)
        }
    }

    pub fn set_default(fontmap: &FontMap) {
        unsafe {
            ffi::pango_cairo_font_map_set_default(fontmap.to_glib_none().0)
        }
    }

    pub fn get_font_type(&self) -> cairo::enums::FontType {
        unsafe {
            ffi::pango_cairo_font_map_get_font_type(self.to_glib_none().0)
        }
    }

    pub fn get_resolution(&self) -> f64 {
        unsafe {
            ffi::pango_cairo_font_map_get_resolution(self.to_glib_none().0)
        }
    }

    pub fn set_resolution(&self, dpi: f64) {
        unsafe {
            ffi::pango_cairo_font_map_set_resolution(self.to_glib_none().0, dpi)
        }
    }
}
