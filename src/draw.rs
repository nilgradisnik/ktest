use gtk::prelude::*;

const START: f64 = 0.25;
const KEY_SIZE: f64 = 0.45;
const KEY_PAD: f64 = 0.5;

fn draw_key(context: &cairo::Context, x: f64, y: f64, width: f64) {
    context.rectangle(x, y, width, KEY_SIZE);
    context.stroke();
}

pub fn draw_surface(_area: &gtk::DrawingArea, context: &cairo::Context) -> Inhibit {
    context.scale(120.0, 120.0);

    context.set_line_width(0.025);
    context.set_source_rgb(0.5, 0.5, 0.5);

    //  Row 1
    for n in 0..14 {
        let x = START + (n as f64 * KEY_PAD);

        if n == 13 {
            draw_key(context, x, START, KEY_SIZE * 2.0);
            continue;
        }

        draw_key(context, x, START, KEY_SIZE);
    }

    //  Row 2
    let mut row_2_pad = 0.0;
    for n in 0..14 {
        let x = START + (n as f64 * KEY_PAD) + row_2_pad;

        if n == 0 {
            draw_key(context, x, START + KEY_PAD, KEY_SIZE * 1.5);
            row_2_pad = row_2_pad + (KEY_SIZE * 1.5 - KEY_SIZE);
            continue;
        }

        if n == 13 {
            draw_key(context, x, START + KEY_PAD, KEY_SIZE * 1.5);
            continue;
        }

        draw_key(context, x, START + KEY_PAD, KEY_SIZE);
    }

    //  Row 3
    let mut row_3_pad = 0.0;
    for n in 0..13 {
        let x = START + (n as f64 * KEY_PAD) + row_3_pad;

        if n == 0 {
            let width = KEY_SIZE * 1.8;
            draw_key(context, x, START + 1.0, width);
            row_3_pad = row_3_pad + (width - KEY_SIZE);
            continue;
        }

        if n == 12 {
            let width = KEY_SIZE * 2.31;
            draw_key(context, x, START + 1.0, width);
            continue;
        }

        draw_key(context, x, START + 1.0, KEY_SIZE);
    }

    //  Row 4
    let mut row_4_pad = 0.0;
    for n in 0..12 {
        let x = START + (n as f64 * KEY_PAD) + row_4_pad;

        if n == 0 {
            let width = KEY_SIZE * 2.31;
            draw_key(context, x, START + 1.5, width);
            row_4_pad = row_4_pad + (width - KEY_SIZE);
            continue;
        }

        if n == 11 {
            let width = KEY_SIZE * 2.92;
            draw_key(context, x, START + 1.5, width);
            continue;
        }

        draw_key(context, x, START + 1.5, KEY_SIZE);
    }

    //  Row 5
    let mut row_5_pad = 0.0;
    for n in 0..8 {
        let x = START + (n as f64 * KEY_PAD) + row_5_pad;

        if n == 3 {
            let width = KEY_SIZE * 8.68;
            draw_key(context, x, START + 2.0, width);
            row_5_pad = row_5_pad + (width - KEY_SIZE);
            continue;
        }

        draw_key(context, x, START + 2.0, KEY_SIZE);
    }

    Inhibit(false)
}
