extern crate gio;
extern crate gtk;

mod draw;

use std::env;

use gio::prelude::*;
use gtk::prelude::*;

use draw::draw_surface;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(gtk::DrawingArea::new)();

    drawing_area.connect_draw(draw_surface);

    window.set_default_size(960, 360);

    window.add(&drawing_area);

    window.show_all();

    window.connect_key_press_event(|_, event| {
        println!("Event: {:?}", event.get_keyval().name(),);
        Inhibit(true)
    });
}

fn main() {
    let application = gtk::Application::new(Some("com.github.ktest"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&env::args().collect::<Vec<_>>());
}
