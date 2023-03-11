use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib};

#[derive(Default)]
pub struct NavalBoard;

#[glib::object_subclass]
impl ObjectSubclass for NavalBoard {
    const NAME: &'static str = "NavalBoard";
    type Type = super::NavalBoard;
    type ParentType = gtk::DrawingArea;
}

impl ObjectImpl for NavalBoard {
    fn constructed(&self) {
        self.parent_constructed();
        let gesture = gtk::GestureClick::new();
        gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
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
        self.obj().add_controller(motion);
        self.obj().add_controller(gesture);
    }
}

impl WidgetImpl for NavalBoard {}

impl DrawingAreaImpl for NavalBoard {
}
