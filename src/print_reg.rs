#![no_std]
#![no_main]

extern crate atmega32u4;
extern crate atmega32u4_hal;
extern crate arduino_leonardo;

use atmega32u4_hal::prelude::*;

#[no_mangle]
pub extern fn main() {
    let dp = atmega32u4::Peripherals::take().unwrap();
    let ei = dp.EXT_INT;
    let mut portc = dp.PORTC.split();
    let mut porte = dp.PORTE.split();

    let mut delay = arduino_leonardo::Delay::new();

    let mut led = portc.pc7.into_output(&mut portc.ddr);

    // Set val to whatever register you want to check
    let val: u8 = 0xAA;
    let val: u8 = dp.USB.usbcon.read().bits();

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
