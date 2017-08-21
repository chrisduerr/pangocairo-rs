extern crate cairo;
extern crate pango;
extern crate pangocairo;
extern crate gtk;

use std::f64::consts::PI;

use cairo::Context;
use gtk::prelude::*;
use gtk::DrawingArea;
use pangocairo::CairoContextExt;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    drawable(500, 500, |_, cr| {
        let font = pango::FontDescription::from_string("Sans Bold 27");

        let layout = cr.create_pango_layout();
        layout.set_text("Hello");
        layout.set_font_description(Some(&font));

        let n_words = 12;
        let radius = 250.;

        cr.translate(radius, radius);
        for i in 0..n_words {
            let angle = (360. * i as f64) / n_words as f64;

            cr.save();

            /* Gradient from red at angle == 60 to blue at angle == 240 */
            let red = (1. + ((angle - 60.) * PI / 180.).cos()) / 2.;
            cr.set_source_rgb(red, 0., 1. - red);

            cr.rotate(angle * PI / 180.);

            /* Inform Pango to re-layout the text with the new transformation */
            cr.update_pango_layout(&layout);

            let (width, _) = layout.get_size();
            cr.move_to(- (width as f64 / pango::SCALE as f64) / 2., - radius);
            cr.show_pango_layout(&layout);

            cr.restore();
        }

        Inhibit(false)
    });

    gtk::main();
}

pub fn drawable<F>(width: i32, height: i32, draw_fn: F)
where F: Fn(&DrawingArea, &Context) -> Inhibit + 'static {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let drawing_area = Box::new(DrawingArea::new)();
    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.add(&drawing_area);
    window.show_all();
}
