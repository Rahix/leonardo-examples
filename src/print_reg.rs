//! Print Register
//! ==============
//!
//! Debug to to display the contents of a register using
//! the led.  Only needed until some sort of serial interface
//! is working.
#![no_std]
#![no_main]

extern crate atmega32u4;
extern crate atmega32u4_hal;
extern crate arduino_leonardo;

use atmega32u4_hal::prelude::*;

#[no_mangle]
pub extern fn main() {
    let dp = atmega32u4::Peripherals::take().unwrap();

    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    // Set val to whatever register you want to check
    let val: u8 = 0xAA;
    // EG:
    // let val: u8 = dp.USB.usbcon.read().bits();

    delay.delay_ms(500);

    for _ in 0..3 {
        led.set_high();
        delay.delay_ms(100);
        led.set_low();
        delay.delay_ms(200);
    }

    delay.delay_ms(1000);

    for i in 0..8 {
        led.set_high();

        if val & (0x80 >> i) != 0 {
            delay.delay_ms(1000);
        } else {
            delay.delay_ms(100);
        }

        led.set_low();
        delay.delay_ms(700);
    }
}
