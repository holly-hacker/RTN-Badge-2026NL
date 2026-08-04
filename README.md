# RTN Badge 2026 (NL edition)

The firmware for the 2026 RTN Badge (NL edition).

## Hardware

The standard version of this badge is built using a CH32V203F6P6 MCU. This is a 32-bit RISC-V chip (`riscv32imc`) with 32kb of flash and 10kb of RAM. It's connected to an 8x8 RGB LED matrix (using WS2812b-2020 LEDs) and has a USB-shaped edge connector allowing it to be inserted into a laptop or desktop PC.

As the CH32V203 family of MCUs contains an USB peripheral, so it can also serve as an arbitrary USB-2 device.

The full schematic and PCB design files can be found [in the PCB repository](https://github.com/holly-hacker/RTN-Badge-2026NL-PCB).

### Build

```bash
cargo build --release
```

### Flash

[wlink](https://github.com/ch32-rs/wlink) needs to be installed:

```bash
cargo install --git https://github.com/ch32-rs/wchisp
```

On linux, you may need to set up udev rules or add your user to a group to access USB devices. Do not run cargo as root!

Either insert the badge while holding the `BOOT` button, or hold the `BOOT` button while pressing `RESET` if it's
already inserted. Then, flash by "running" the firmware via cargo:

```bash
cargo run
```

### Debugging

You can also use a WCH-Link debug probe by soldering a 2.54mm pin header to the badge and connecting the following pins
on the debugger to the pins on the badge:
- `GND` to `GND` (if multiple are available, just pick any)
- `SWCLK` to `CLK`
- `SWDIO` to `DIO`
- `5V` to `5V`

Then, install the wlink tool:

```bash
cargo install --git https://github.com/ch32-rs/wlink
```

Finally, swap the runner command in `.cargo/config.toml` to the `wlink` one and run the firmware:

```bash
cargo run
```
