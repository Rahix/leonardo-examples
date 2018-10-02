//! Blink
//! =====
//!
//! Basic blinking example, using the User LED, TX LED and RX LED
#![no_std]
#![no_main]
extern crate arduino_leonardo;

use arduino_leonardo::prelude::*;

#[no_mangle]
pub extern fn main() {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
    let mut led1 = pins.led_tx.into_output(&mut pins.ddr);
    let mut led2 = pins.d13.into_output(&mut pins.ddr);

    led0.set_high();
    led1.set_high();
    led2.set_high();

    let mut leds = [
        led0.downgrade().downgrade(),
        led1.downgrade().downgrade(),
        led2.downgrade().downgrade(),
    ];

    loop {
        for i in 0..3 {
            leds[i].toggle();
            leds[(i+2)%3].toggle();
            delay.delay_ms(200);
        }
    }
}
