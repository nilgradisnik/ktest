extern crate gdk;
extern crate gio;
extern crate gtk;

mod constants;
mod surface;

use std::env;

use gio::prelude::*;
use gtk::prelude::*;

use surface::Surface;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let surface = Surface::new();

    window.set_default_size(960, 360);

    window.add(&surface.lock().unwrap().area);

    window.show_all();

    window.set_position(gtk::WindowPosition::CenterAlways);

    let surface1 = surface.clone();
    window.connect_key_press_event(move |_, event| {
        surface1.lock().unwrap().set_key(Some(event.to_owned()));

        Inhibit(true)
    });

    // let surface2 = surface.clone();
    // window.connect_key_release_event(move |_, _| {
    //     surface2.lock().unwrap().set_key(None);

    //     Inhibit(true)
    // });
}

fn main() {
    let application = gtk::Application::new(Some("com.github.ktest"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&env::args().collect::<Vec<_>>());
}
