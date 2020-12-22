use std::sync::{Arc, Mutex};

use cairo::{Context, FontSlant, FontWeight};
use gdk::EventKey;
use gtk::prelude::*;

use super::constants::{
    KEY_PAD, KEY_SIZE, MISSING_KEYCODE, RESET_KEYCODE, ROW_1, ROW_2, ROW_3, ROW_4, ROW_5, X_START,
};

pub struct Surface {
    pub area: gtk::DrawingArea,
    key: Option<EventKey>,
}

impl Surface {
    pub fn new() -> Arc<Mutex<Surface>> {
        let surface = Arc::new(Mutex::new(Surface {
            area: Box::new(gtk::DrawingArea::new)(),
            key: None,
        }));

        let clone = surface.clone();
        surface
            .lock()
            .unwrap()
            .area
            .connect_draw(move |_, context| clone.lock().unwrap().redraw(context.to_owned()));

        surface
    }

    pub fn set_key(&mut self, key: Option<EventKey>) {
        self.key = key;
        self.area.queue_draw();
    }

    fn draw_key(&self, context: &cairo::Context, x: f64, y: f64, width: f64, press: bool) {
        if press {
            context.set_source_rgb(1.0, 0.5, 0.5);

            if let Some(key) = &self.key {
                let char = key.get_keyval().to_unicode();
                println!("Char: {:?}", char);

                if let Some(c) = char {
                    context.move_to(x + 0.1, y + 0.2);
                    context.show_text(&c.to_string());
                }
            }
        } else {
            context.set_source_rgb(0.5, 0.5, 0.5);
        }

        context.rectangle(x, y, width, KEY_SIZE);
        context.stroke();
    }

    fn redraw(&self, context: Context) -> Inhibit {
        let mut row_1_index = None;
        let mut row_2_index = None;
        let mut row_3_index = None;
        let mut row_4_index = None;
        let mut row_5_index = None;

        if let Some(key) = &self.key {
            let hardware_keycode = key.get_hardware_keycode();

            row_1_index = ROW_1.iter().position(|&k| k == hardware_keycode);
            row_2_index = ROW_2.iter().position(|&k| k == hardware_keycode);
            row_3_index = ROW_3.iter().position(|&k| k == hardware_keycode);
            row_4_index = ROW_4.iter().position(|&k| k == hardware_keycode);
            row_5_index = ROW_5.iter().position(|&k| k == hardware_keycode);

            if hardware_keycode != RESET_KEYCODE && hardware_keycode != MISSING_KEYCODE {
                println!("Hardware keycode: {}", hardware_keycode);
            }
        }

        context.scale(120.0, 120.0);
        context.set_line_width(0.025);

        context.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        context.set_font_size(0.2);

        //  Row 1
        for n in 0..14 {
            let press = row_1_index.unwrap_or(RESET_KEYCODE.into()) == n;

            let x = X_START + (n as f64 * KEY_PAD);

            if n == 13 {
                self.draw_key(&context, x, X_START, KEY_SIZE * 2.0, press);
                continue;
            }

            self.draw_key(&context, x, X_START, KEY_SIZE, press);
        }

        //  Row 2
        let mut row_2_pad = 0.0;
        for n in 0..14 {
            let press = row_2_index.unwrap_or(RESET_KEYCODE.into()) == n;
            let x = X_START + (n as f64 * KEY_PAD) + row_2_pad;

            if n == 0 {
                self.draw_key(&context, x, X_START + KEY_PAD, KEY_SIZE * 1.5, press);
                row_2_pad = row_2_pad + (KEY_SIZE * 1.5 - KEY_SIZE);
                continue;
            }

            if n == 13 {
                self.draw_key(&context, x, X_START + KEY_PAD, KEY_SIZE * 1.5, press);
                continue;
            }

            self.draw_key(&context, x, X_START + KEY_PAD, KEY_SIZE, press);
        }

        //  Row 3
        let mut row_3_pad = 0.0;
        for n in 0..13 {
            let press = row_3_index.unwrap_or(RESET_KEYCODE.into()) == n;
            let x = X_START + (n as f64 * KEY_PAD) + row_3_pad;

            if n == 0 {
                let width = KEY_SIZE * 1.8;
                self.draw_key(&context, x, X_START + 1.0, width, press);
                row_3_pad = row_3_pad + (width - KEY_SIZE);
                continue;
            }

            if n == 12 {
                let width = KEY_SIZE * 2.31;
                self.draw_key(&context, x, X_START + 1.0, width, press);
                continue;
            }

            self.draw_key(&context, x, X_START + 1.0, KEY_SIZE, press);
        }

        //  Row 4
        let mut row_4_pad = 0.0;
        for n in 0..12 {
            let press = row_4_index.unwrap_or(RESET_KEYCODE.into()) == n;
            let x = X_START + (n as f64 * KEY_PAD) + row_4_pad;

            if n == 0 {
                let width = KEY_SIZE * 2.31;
                self.draw_key(&context, x, X_START + 1.5, width, press);
                row_4_pad = row_4_pad + (width - KEY_SIZE);
                continue;
            }

            if n == 11 {
                let width = KEY_SIZE * 2.92;
                self.draw_key(&context, x, X_START + 1.5, width, press);
                continue;
            }

            self.draw_key(&context, x, X_START + 1.5, KEY_SIZE, press);
        }

        //  Row 5
        let mut row_5_pad = 0.0;
        for n in 0..8 {
            let press = row_5_index.unwrap_or(RESET_KEYCODE.into()) == n;
            let x = X_START + (n as f64 * KEY_PAD) + row_5_pad;

            if n == 3 {
                let width = KEY_SIZE * 8.68;
                self.draw_key(&context, x, X_START + 2.0, width, press);
                row_5_pad = row_5_pad + (width - KEY_SIZE);
                continue;
            }

            self.draw_key(&context, x, X_START + 2.0, KEY_SIZE, press);
        }

        Inhibit(true)
    }
}
