# Experimental Rust OS

## Building
Run `cargo make kernel {target}` to build the kernel for the specified target. 


## Targets
- `arm64` (default)
- `arm64-softfloat`


## Hardware
- Raspberry Pi 4B

## Operation
No bootloader built in. Place a standard Raspberry Pi OS on a bootable micro SD card. After building the kernel, copy the kernel8.img file to the micro SD card. Boot the Raspberry Pi with the micro SD card. The kernel will be loaded and executed.

## Current Features
- A light connected to GPIO 17 (pin 11) will blink on and off every second.
- A light connected to GPIO 26 (pin 37) be solid on.

