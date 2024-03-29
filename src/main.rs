#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::clocks::{Clock, init_clocks_and_plls};
use bsp::hal::pac;
use bsp::hal::sio::Sio;
use bsp::hal::watchdog::Watchdog;
use core::fmt::Write;
use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::prelude::_embedded_hal_serial_Write;
use fugit::RateExtU32;
use panic_probe as _;
use rp_pico as bsp;
use rp_pico::hal::gpio::DynPin;
use rp_pico::hal::uart::{DataBits, StopBits, UartConfig, UartPeripheral};
use rp_pico::hal;
use usb_device::prelude::*;
use usb_device::class_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;

use crate::keys::{get_keys, send_key};

mod keys;
mod key_layouts;

const USB_HOST_POLL_MS: u8 = 10;

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

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let _led_pin = pins.led.into_push_pull_output();

    let usb_bus = UsbBusAllocator::new(bsp::hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut usb_keyboard_hid = HIDClass::new(&usb_bus, KeyboardReport::desc(), USB_HOST_POLL_MS);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27da))
        .manufacturer("Kenneth Hedman")
        .product("Klaveret")
        .serial_number("0")
        .device_class(0)
        .build();

    let mut p0: DynPin = pins.gpio14.into();
    let mut p1: DynPin = pins.gpio13.into();

    p0.into_pull_down_input();
    p1.into_pull_down_input();
    let input_pins = [p0, p1];
    let mut p2: DynPin = pins.gpio17.into();
    let mut p3: DynPin = pins.gpio18.into();

    p2.into_push_pull_output();
    p3.into_push_pull_output();
    let mut output_pins = [p2, p3];

    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_mode::<hal::gpio::FunctionUart>(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_mode::<hal::gpio::FunctionUart>(),
    );

    let hz = RateExtU32::Hz(9600);

    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(hz, DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    uart.write_full_blocking(b"UART redo\r\n");

    let mut prev_keys: [u8; 6] = [0; 6];

    loop {
        // TODO use ticks instead?
        usb_dev.poll(&mut [&mut usb_keyboard_hid]);
        let now_keys = get_keys(&mut output_pins, &input_pins, &mut delay);

        if now_keys != prev_keys {
            let _ = uart.write_str("Key stuff\r\n");

            prev_keys = now_keys;
            send_key(&usb_keyboard_hid, now_keys);
            delay.delay_ms(USB_HOST_POLL_MS.into());
        }
    }
}
