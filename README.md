# Deprecation Note:
**Refer to the examples in [`avr-hal`](https://github.com/Rahix/avr-hal) instead.**

For Arduino Leonardo, they can be found in [`boards/arduino-leonardo/examples`](https://github.com/Rahix/avr-hal/tree/master/boards/arduino-leonardo/examples).


# Arduino Leonardo Examples in Rust

A few examples of using the `arduino-leonardo`, `atmega32u4-hal` and `atmega32u4` crates.

* [blink](src/blink.rs): Blink the three onboard LEDs
* [interrupt](src/interrupt.rs): Setup a pin interrupt and show how to install a handler
* [pwm](src/pwm.rs): Demo of setting a pin into PWM mode

For easily uploading the compiled demos to your Leonardo, there are two scripts available:

* [`mkhex.sh`](mkhex.sh): Convert the `.elf` that rust produces to a `.hex` that can be flashed.
  Also outputs the size of the final binary. Supply either `blink`, `interrupt` or `pwm` as commandline
  argument.
* [`flash.sh`](flash.sh): Upload a `.hex` file to your board. You need to press the reset button
  before calling this script! Supply either `blink`, `interrupt` or `pwm` as commandline argument.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
