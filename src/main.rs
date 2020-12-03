#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
// extern crate atsamd_hal as atsamd;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals, SERCOM2, PM};
use hal::sercom;
use hal::prelude::*;
use hal::gpio;
use hal::pad::PadPin;
use arduino_nano33iot::time::Hertz;

pub fn uart2<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: SERCOM2,
    pm: &mut PM,
    rx: gpio::Pa11<gpio::Input<gpio::Floating>>,
    tx: gpio::Pa10<gpio::Input<gpio::Floating>>,
    port: &mut gpio::Port,
) -> sercom::UART2<
    sercom::Sercom2Pad3<gpio::Pa11<gpio::PfD>>,
    sercom::Sercom2Pad2<gpio::Pa10<gpio::PfD>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    sercom::UART2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        pm,
        (rx.into_pad(port), tx.into_pad(port)),
    )
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut uart5 = hal::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.rx,
        pins.tx,
        &mut pins.port,
    );

    let mut uart2 = uart2(
        &mut clocks,
        19200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.PM,
        pins.a2,
        pins.a3,
        &mut pins.port,
    );

    loop {
        // print ASCII characters from ! to ~
        for ch in 33..127 {
            uart5.write(ch).unwrap();
            uart2.write(ch).unwrap();
            delay.delay_ms(500u16);
        }
    }
}
