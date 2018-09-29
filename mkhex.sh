#!/bin/sh

avr-objcopy -j .text -j .data -O ihex target/avr-atmega32u4/release/$1.elf target/$1.hex
echo "$(cat target/$1.hex | wc -c | numfmt --to=si) Bytes used."
