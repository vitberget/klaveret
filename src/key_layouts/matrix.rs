use crate::key_layouts::keycodes::*;

pub(crate) const KEY_MATRIX_SWEDISH: [[u8; 2]; 2] =
    [
        [KEY_I, KEY_O],
        [KEY_D, KEY_N]
    ];

pub(crate) fn get_key_code(output: usize, input: usize) -> u8 {
    KEY_MATRIX_SWEDISH[output][input]
}

