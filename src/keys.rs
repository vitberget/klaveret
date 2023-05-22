use cortex_m::delay::Delay;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp_pico::hal::gpio::DynPin;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;
use rp_pico as bsp;

use crate::key_layouts::matrix::get_key_code;

pub(crate) fn get_keys(outputs: &mut [DynPin; 2],
            inputs: &[DynPin; 2],
            delay: &mut Delay) -> [u8; 6] {

    let mut result = [0; 6];
    let mut r_idx = 0;

    for (output_idx, output) in outputs.iter_mut().enumerate() {
        output.set_high().unwrap();
        delay.delay_us(20); // TODO What's a good delay?

        r_idx = get_keys_for_output(inputs, &mut result, r_idx, output_idx);

        output.set_low().unwrap();

        if r_idx > 5 { break; }
    }

    result
}

fn get_keys_for_output(inputs: &[DynPin; 2],
                       result: &mut [u8; 6],
                       r_idx: usize,
                       output_idx: usize) -> usize {

    let mut r_idx = r_idx;

    for (input_idx, input) in inputs.iter().enumerate() {
        if let Ok(true) = input.is_high() {
            result[r_idx] = get_key_code(output_idx, input_idx);
            r_idx += 1;
        }

        if r_idx > 5 { break; }
    }

    r_idx
}

pub(crate) fn send_key(usb_hid: &HIDClass<bsp::hal::usb::UsbBus>, keys: [u8; 6]) {
    let keyboard_report = KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: keys,
    };

    usb_hid.push_input(&keyboard_report).unwrap();
}