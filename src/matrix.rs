use embedded_hal::digital::v2::{InputPin, OutputPin};
use crate::keycodes::*;
use rp_pico as bsp;

pub(crate) const KEY_MATRIX_SWEDISH: [[u8; 2]; 2] =
    [
        [KEY_I, KEY_0],
        [KEY_D, KEY_N]
    ];




pub(crate) fn get_key_code(output: usize, input: usize) -> u8 {
    return KEY_MATRIX_SWEDISH[output][input];
}

