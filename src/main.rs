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
        .margin_bottom(20)
        .margin_top(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    drawing_area.connect_resize(|a, b, c| resize_drawing_area(a, b, c));
    drawing_area.connect_realize(|d| realize_drawing_area(d));
    
    let gesture = gtk::GestureClick::new();

    gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);

    gesture.connect_pressed(|g, n, x, y| {
        println!("Gesture pressed");
        println!("Details: {} {} {} {}", g, n, x, y);
    });

    gesture.connect_released(|g, n, x, y| {
        println!("Gesture released");
        println!("Details: {} {} {} {}", g, n, x, y);
    });

    let motion = gtk::GestureDrag::new();

    motion.set_button(gtk::gdk::ffi::GDK_BUTTON_MIDDLE as u32);

    motion.connect_drag_update(|g, x, y| {
        println!("Gesture drag update");
        println!("Details: {} {} {}", g, x, y);
    });



    //drawing_area.add_controller(gesture);
    drawing_area.add_controller(motion);
    
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