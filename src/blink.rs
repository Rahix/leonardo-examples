#![no_std]
#![no_main]
extern crate arduino_leonardo;

use arduino_leonardo::prelude::*;

#[no_mangle]
pub extern fn main() {
    let mut leo = arduino_leonardo::Leonardo::new().unwrap();

    let mut delay = arduino_leonardo::Delay::new();

    let mut led0: arduino_leonardo::LedBuiltinRX = leo.led_rx.into_output(&mut leo.ddr.portb);
    let mut led1: arduino_leonardo::LedBuiltinTX = leo.led_tx.into_output(&mut leo.ddr.portd);
    let mut led2: arduino_leonardo::LedBuiltin   = leo.d13.into_output(&mut leo.ddr.portc);

    loop {
        led0.set_low();
        led1.set_high();
        delay.delay_ms(100);

        led0.set_high();
        delay.delay_ms(100);
        led2.set_high();
        delay.delay_ms(100);

        led1.set_low();
        led2.set_low();
        delay.delay_ms(100);
    }
}
