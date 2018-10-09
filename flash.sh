#!/bin/sh
if [[ $# != 1 ]]; then
    echo "usage: $0 <hex-name>"
    echo
    echo "Ensure the board is in bootloader mode before attempting to flash!"
    exit 1
fi

set -xe

avrdude -C/etc/avrdude.conf -patmega32u4 -cavr109 -v -v -v -v -P/dev/ttyACM0 -b57600 -D -Uflash:w:target/$1.hex:i
