#!/bin/sh
if [[ $# != 1 ]]; then
    echo "usage: $0 <elf-name>"
    exit 1
fi

set -xe

avr-objcopy -j .text -j .data -O ihex target/avr-atmega32u4/release/$1.elf target/$1.hex

set +x

BYTES=$(cat target/$1.hex | wc -c)
echo "$(numfmt --to=si $BYTES) Bytes used ($BYTES exact)."
