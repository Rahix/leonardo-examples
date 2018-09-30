#!/bin/sh

avr-objcopy -j .text -j .data -O ihex target/avr-atmega32u4/release/$1.elf target/$1.hex
BYTES=$(cat target/$1.hex | wc -c)
echo "$(numfmt --to=si $BYTES) Bytes used ($BYTES exact)."
