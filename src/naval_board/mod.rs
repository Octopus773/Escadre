mod imp;
use glib::Object;
use gtk::prelude::*;
use gtk::{cairo, glib};

glib::wrapper! {
    pub struct NavalBoard(ObjectSubclass<imp::NavalBoard>)
        @extends gtk::Widget, gtk::DrawingArea,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NavalBoard {
    pub fn new() -> Self {
        let nb: NavalBoard = Object::builder().build();
        nb.set_size_request(300, 300);
        DrawingAreaExtManual::set_draw_func(&nb, |_d, c, w, h| NavalBoard::draw(c, w, h));
        nb
    }

    pub fn draw(c: &cairo::Context, w: i32, h: i32) {
        c.set_source_rgb(0.0, 0.0, 0.0);
        // draw lines for a naval battle board in a 10x10 grid
        // the board is viewed isometrically

        let w = w as f64;
        let h = h as f64;

        let num_lines = 10;
        let step = ((w/ 2. / num_lines as f64) , (h / 2. / num_lines as f64));
        let mut pos_start = (0., h / 2.);
        let v = (w / 2., h / 2.);

        // draw the horizontal lines
        for _ in 0..=10 {
            c.move_to(pos_start.0, pos_start.1);
            c.line_to(pos_start.0 + v.0, pos_start.1 - v.1);
            pos_start = (pos_start.0 + step.0, pos_start.1 + step.1);
        }
        // draw the vertical lines
        pos_start = (pos_start.0 - step.0, pos_start.1 - step.1);
        for _ in 0..=10 {
            c.move_to(pos_start.0, pos_start.1);
            c.line_to(pos_start.0 - v.0, pos_start.1 - v.1);
            pos_start = (pos_start.0 + step.0, pos_start.1 - step.1);
        }
        c.stroke().unwrap();
    }
}
