use gtk::prelude::*;
use gtk::{Application};


mod naval_board;
const APP_ID: &str = "com.gtk_rs.escadre.game";

fn build_ui(application: &Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Escadre")
        .default_width(640)
        .default_height(480)
        .build();

    let drawing_area = naval_board::NavalBoard::new();

    window.set_child(Some(&drawing_area));

    window.present();
}

fn main() {
    let application = Application::new(Some(APP_ID), Default::default());
    application.connect_activate(build_ui);
    application.run();
}