pub const RESET_KEYCODE: u16 = 999;
pub const MISSING_KEYCODE: u16 = 888;

pub const X_START: f64 = 0.25;
pub const KEY_SIZE: f64 = 0.45;
pub const KEY_PAD: f64 = 0.5;

pub const ROW_1: [u16; 14] = [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];
pub const ROW_2: [u16; 14] = [23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 51];
pub const ROW_3: [u16; 13] = [
    MISSING_KEYCODE,
    38,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,
    36,
];
pub const ROW_4: [u16; 12] = [50, 52, 53, 54, 55, 56, 57, 58, 59, 60, 111, 62];
pub const ROW_5: [u16; 8] = [37, MISSING_KEYCODE, 64, 65, 113, 116, 114, MISSING_KEYCODE];
