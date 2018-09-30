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
    let mut portc = dp.PORTC.split();
    let mut portd = dp.PORTD.split();

    let mut delay = arduino_leonardo::Delay::new();

    let mut led = portc.pc7.into_output(&mut portc.ddr);
    let txled = portd.pd5.into_output(&mut portd.ddr);

    // This pin needs to be accessed in the isr, so we need to make it globally
    // available
    TXLED.set(txled);

    // Initialize INT6
    ei.eicrb.write(|w| w.isc6().edge_both());
    ei.eimsk.write(|w| w.int6().set_bit());

    // The bootloader leaves USB enabled;  this is problematic
    // because USB continuously generates interrupts until some
    // code handles it.  Just disable USB altogether for this
    // example.
    dp.USB.usbcon.reset();

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
