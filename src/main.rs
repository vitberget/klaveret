#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

use rp_pico as bsp;

use usb_device::{class_prelude::*, prelude::*};
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

const USB_HOST_POLL_MS: u8 = 10;
const KEY_I: u8 = 0x0c;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn entry() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();
    let mut i_pin = pins.gpio14.into_pull_down_input();
    let mut o_pin = pins.gpio17.into_push_pull_output();

    o_pin.set_high().unwrap();

    let usb_bus = UsbBusAllocator::new(bsp::hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut usb_hid = HIDClass::new(&usb_bus, KeyboardReport::desc(), USB_HOST_POLL_MS);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27da))
        .manufacturer("Kenneth Hedman")
        .product("Klaveret")
        .serial_number("0")
        .device_class(0)
        .build();

    let mut counter: u32 = 0;

    let mut prev_i = false;

    loop {
        usb_dev.poll(&mut [&mut usb_hid]);
        let now_i = i_pin.is_high().unwrap();

        if now_i != prev_i {
            prev_i = now_i;
            if now_i {
                led_pin.set_high().unwrap();
                send_key(&usb_hid, KEY_I);
            } else {
                led_pin.set_low().unwrap();
                send_key(&usb_hid, 0);
            }
            delay.delay_ms(USB_HOST_POLL_MS.into());
        }
    }
}

fn send_key(usb_hid: &HIDClass<bsp::hal::usb::UsbBus>, key_code: u8) {
    let mut keyboard_report = KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0; 6],
    };

    keyboard_report.keycodes[0] = key_code;
    usb_hid.push_input(&keyboard_report).unwrap();


}

// End of file
