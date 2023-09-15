# teensy4-selfrebootor

[![Crates.io](https://img.shields.io/crates/v/teensy4-selfrebootor)](https://crates.io/crates/teensy4-selfrebootor)
[![Crates.io](https://img.shields.io/crates/d/teensy4-selfrebootor)](https://crates.io/crates/teensy4-selfrebootor)
[![License](https://img.shields.io/crates/l/teensy4-selfrebootor)](https://github.com/Finomnis/teensy4-selfrebootor/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/teensy4-selfrebootor/ci.yml)](https://github.com/Finomnis/teensy4-selfrebootor/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/teensy4-selfrebootor)](https://docs.rs/teensy4-selfrebootor)


This crate provides a USB device compatible with `teensy_loader_cli -r` to force the Teensy4 to reboot itself into bootloader.

This allows the board to be reprogrammed without having to press the `Reset`/`Boot` button.

A requirement of this crate is that there is no other use for the USB port, as it will fully consume it.


# Examples

*- examples are intended for the [Teensy 4.0](https://www.pjrc.com/store/teensy40.html), [Teensy 4.1](https://www.pjrc.com/store/teensy41.html) or [Teensy MicroMod](https://www.sparkfun.com/products/16402) board -*

## Prerequisites

The following hardware is required for the examples:
- A [Teensy 4.0](https://www.pjrc.com/store/teensy40.html)/[Teensy 4.1](https://www.pjrc.com/store/teensy41.html)/[Teensy MicroMod](https://www.sparkfun.com/products/16402) development board

The following software tools have to be installed:
- Python3 (as `python3`, or modify `run.py` to use the `python` binary)
- [`cargo-binutils`](https://crates.io/crates/cargo-binutils)
- [`teensy_loader_cli`](https://www.pjrc.com/teensy/loader_cli.html)


## Run

- Connect the Teensy to PC via USB cable.
- Press the `Reset`/`Boot` button on the Teensy.
- Run:
  ```bash
  cargo run --release --example teensy4_selfrebootor
  ```
- Run the previous command again. This time the command should not need
  the `Reset`/`Boot` button. The Teensy should switch to boot mode automatically.
