use gtk::prelude::*;
use gtk::{glib, Application, DrawingArea, Window, cairo};


const APP_ID: &str = "com.gtk_rs.escadre.game";

fn realize_drawing_area(d: &DrawingArea) {
    println!("Drawing area realized");
}

fn resize_drawing_area(drawing_area: &DrawingArea, width: i32, height: i32) {
    println!("Drawing area resized");
}

fn render_drawing_area(d: &DrawingArea, c: &cairo::Context, h: i32, w: i32) {
    // draw a penguin
    c.set_source_rgb(0.0, 0.0, 0.0);
    c.paint().unwrap();
    c.set_source_rgb(1.0, 1.0, 1.0);
    c.arc(w as f64 / 2.0, h as f64 / 2.0, 50.0, 0.0, 2.0 * std::f64::consts::PI);
    c.fill().unwrap();
}

fn build_ui(application: &Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Escadre")
        .default_width(640)
        .default_height(480)
        .build();

    let drawing_area = DrawingArea::builder()
        .build();

    drawing_area.connect_resize(|a, b, c| resize_drawing_area(a, b, c));
    drawing_area.connect_realize(|d| realize_drawing_area(d));
    // render on every frame
    DrawingAreaExtManual::set_draw_func(&drawing_area, |d, c, h, w| render_drawing_area(d, c, h, w));

    window.set_child(Some(&drawing_area));

    window.present();
}

fn main() {
    let application = Application::new(Some(APP_ID), Default::default());
    application.connect_activate(build_ui);
    application.run();
}