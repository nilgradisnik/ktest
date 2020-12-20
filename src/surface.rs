use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

use cairo::Context;
use gtk::prelude::*;

pub const RESET_KEYCODE: usize = 999;

const ROW_1: [u16; 14] = [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];
const ROW_2: [u16; 14] = [23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 51];
const ROW_3: [u16; 13] = [888, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 36];
const ROW_4: [u16; 12] = [50, 52, 53, 54, 55, 56, 57, 58, 59, 60, 111, 62];
const ROW_5: [u16; 8] = [37, 888, 64, 65, 113, 116, 114, 888];

const START: f64 = 0.25;
const KEY_SIZE: f64 = 0.45;
const KEY_PAD: f64 = 0.5;

pub struct Surface {
    pub area: gtk::DrawingArea,
    keycode: u16,
}

impl Surface {
    pub fn new() -> Arc<Mutex<Surface>> {
        let surface = Arc::new(Mutex::new(Surface {
            area: Box::new(gtk::DrawingArea::new)(),
            keycode: RESET_KEYCODE.try_into().unwrap(),
        }));

        let clone = surface.clone();
        surface
            .lock()
            .unwrap()
            .area
            .connect_draw(move |_area, context| clone.lock().unwrap().redraw(context.to_owned()));

        surface
    }

    fn draw_key(context: &cairo::Context, x: f64, y: f64, width: f64, press: bool) {
        if press {
            context.set_source_rgb(1.0, 0.5, 0.5);
        } else {
            context.set_source_rgb(0.5, 0.5, 0.5);
        }
        context.rectangle(x, y, width, KEY_SIZE);
        context.stroke();
    }

    pub fn set_keycode(&mut self, keycode: u16) {
        self.keycode = keycode;
        self.area.queue_draw();
    }

    fn redraw(&self, context: Context) -> Inhibit {
        if self.keycode != 999 && self.keycode != 888 {
            println!("Hardware keycode: {}", self.keycode);
        }

        context.scale(120.0, 120.0);
        context.set_line_width(0.025);

        let row_1_index = ROW_1.iter().position(|&k| k == self.keycode);
        let row_2_index = ROW_2.iter().position(|&k| k == self.keycode);
        let row_3_index = ROW_3.iter().position(|&k| k == self.keycode);
        let row_4_index = ROW_4.iter().position(|&k| k == self.keycode);
        let row_5_index = ROW_5.iter().position(|&k| k == self.keycode);

        //  Row 1
        for n in 0..14 {
            let press = row_1_index.unwrap_or(RESET_KEYCODE) == n;

            let x = START + (n as f64 * KEY_PAD);

            if n == 13 {
                Surface::draw_key(&context, x, START, KEY_SIZE * 2.0, press);
                continue;
            }

            Surface::draw_key(&context, x, START, KEY_SIZE, press);
        }

        //  Row 2
        let mut row_2_pad = 0.0;
        for n in 0..14 {
            let press = row_2_index.unwrap_or(RESET_KEYCODE) == n;
            let x = START + (n as f64 * KEY_PAD) + row_2_pad;

            if n == 0 {
                Surface::draw_key(&context, x, START + KEY_PAD, KEY_SIZE * 1.5, press);
                row_2_pad = row_2_pad + (KEY_SIZE * 1.5 - KEY_SIZE);
                continue;
            }

            if n == 13 {
                Surface::draw_key(&context, x, START + KEY_PAD, KEY_SIZE * 1.5, press);
                continue;
            }

            Surface::draw_key(&context, x, START + KEY_PAD, KEY_SIZE, press);
        }

        //  Row 3
        let mut row_3_pad = 0.0;
        for n in 0..13 {
            let press = row_3_index.unwrap_or(RESET_KEYCODE) == n;
            let x = START + (n as f64 * KEY_PAD) + row_3_pad;

            if n == 0 {
                let width = KEY_SIZE * 1.8;
                Surface::draw_key(&context, x, START + 1.0, width, press);
                row_3_pad = row_3_pad + (width - KEY_SIZE);
                continue;
            }

            if n == 12 {
                let width = KEY_SIZE * 2.31;
                Surface::draw_key(&context, x, START + 1.0, width, press);
                continue;
            }

            Surface::draw_key(&context, x, START + 1.0, KEY_SIZE, press);
        }

        //  Row 4
        let mut row_4_pad = 0.0;
        for n in 0..12 {
            let press = row_4_index.unwrap_or(RESET_KEYCODE) == n;
            let x = START + (n as f64 * KEY_PAD) + row_4_pad;

            if n == 0 {
                let width = KEY_SIZE * 2.31;
                Surface::draw_key(&context, x, START + 1.5, width, press);
                row_4_pad = row_4_pad + (width - KEY_SIZE);
                continue;
            }

            if n == 11 {
                let width = KEY_SIZE * 2.92;
                Surface::draw_key(&context, x, START + 1.5, width, press);
                continue;
            }

            Surface::draw_key(&context, x, START + 1.5, KEY_SIZE, press);
        }

        //  Row 5
        let mut row_5_pad = 0.0;
        for n in 0..8 {
            let press = row_5_index.unwrap_or(RESET_KEYCODE) == n;
            let x = START + (n as f64 * KEY_PAD) + row_5_pad;

            if n == 3 {
                let width = KEY_SIZE * 8.68;
                Surface::draw_key(&context, x, START + 2.0, width, press);
                row_5_pad = row_5_pad + (width - KEY_SIZE);
                continue;
            }

            Surface::draw_key(&context, x, START + 2.0, KEY_SIZE, press);
        }

        Inhibit(true)
    }
}
