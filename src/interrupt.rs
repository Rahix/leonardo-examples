//! Interrupt
//! =========
//!
//! Example of defining a custom interrupt handler
#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

#[macro_use]
extern crate atmega32u4;
extern crate atmega32u4_hal;
extern crate arduino_leonardo;

use atmega32u4_hal::prelude::*;
use atmega32u4_hal::port;
use atmega32u4_hal::port::mode;

static TXLED: atmega32u4_hal::Global<port::portd::PD5<mode::io::Output>> = atmega32u4_hal::Global::new();

#[no_mangle]
pub extern fn main() {
    let dp = atmega32u4::Peripherals::take().unwrap();
    let ei = dp.EXT_INT;

    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut led = pins.d13.into_output(&mut pins.ddr);
    let txled = pins.led_tx.into_output(&mut pins.ddr);

    // In theory this should not be necessary ... But if you previously had
    // a sketch from Arduino loaded, the USB device will not have been reset.
    // Because of this we will be spammed with interrupts which will never
    // stop because they are never handled.
    dp.USB.usbcon.reset();

    // This pin needs to be accessed in the isr, so we need to make it globally
    // available
    TXLED.set(txled);

    // Initialize INT6
    ei.eicrb.write(|w| w.isc6().edge_both());
    ei.eimsk.write(|w| w.int6().set_bit());

    // Enable interrupts
    atmega32u4::interrupt::enable();

    // Do something
    loop {
        led.set_high();
        delay.delay_ms(100);

        led.set_low();
        delay.delay_ms(400);
    }
}

// The [interrupt!] macro is used to define a custom isr
interrupt!(INT6, isr);
fn isr() {
    let mut delay = arduino_leonardo::Delay::new();
    TXLED.get(|txled| {
        txled.set_low();
        delay.delay_ms(100);
        txled.set_high();
        delay.delay_ms(100);
    }).expect("Interrupt fired before txled was set");
}
