extern crate gio;
extern crate gtk;

mod surface;

use std::{convert::TryInto, env};

use gio::prelude::*;
use gtk::prelude::*;

use surface::{Surface, RESET_KEYCODE};

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let surface = Surface::new();

    window.set_default_size(960, 360);

    window.add(&surface.lock().unwrap().area);

    window.show_all();

    window.set_position(gtk::WindowPosition::CenterAlways);

    let surface1 = surface.clone();
    window.connect_key_press_event(move |_, event| {
        surface1
            .lock()
            .unwrap()
            .set_keycode(event.get_hardware_keycode());

        Inhibit(true)
    });

    let surface2 = surface.clone();
    window.connect_key_release_event(move |_, _| {
        surface2
            .lock()
            .unwrap()
            .set_keycode(RESET_KEYCODE.try_into().unwrap());

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
