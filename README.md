# teensy4-selfrebootor

[![Crates.io](https://img.shields.io/crates/v/teensy4-selfrebootor)](https://crates.io/crates/teensy4-selfrebootor)
[![Crates.io](https://img.shields.io/crates/d/teensy4-selfrebootor)](https://crates.io/crates/teensy4-selfrebootor)
[![License](https://img.shields.io/crates/l/teensy4-selfrebootor)](https://github.com/Finomnis/teensy4-selfrebootor/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/teensy4-selfrebootor/ci.yml)](https://github.com/Finomnis/teensy4-selfrebootor/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/teensy4-selfrebootor)](https://docs.rs/teensy4-selfrebootor)



This crate provides a USB device that can be used by `teensy_loader_cli`'s `-r` flag to request a
reboot into bootloader.

This allows the board to be reprogrammed without having to press the `Reset`/`Boot` button.

A requirement of this crate is that there is no other use for the USB port, as it will fully consume it.


# Examples

*- examples are intended for the [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) or [Teensy MicroMod](https://www.sparkfun.com/products/16402) board -*

## Prerequisites

The following hardware is required for the examples:
- A [Teensy 4.0](https://www.pjrc.com/store/teensy40.html)/[Teensy MicroMod](https://www.sparkfun.com/products/16402) development board
- Optional: A way to read the Teensy's UART, like a USB-UART-converter

The following software tools have to be installed:
- Python3 (as `python3`, or modify `run.py` to use the `python` binary)
- `llvm-objcopy`
  - Install [`LLVM`](https://github.com/llvm/llvm-project/releases) tool suite
- [`teensy_loader_cli`](https://www.pjrc.com/teensy/loader_cli.html)


## Run

- Connect the Teensy to PC via USB cable.
- Run:
    - For the *Teensy 4.0*: `cargo run --release --example teensy_4_0`.
    - For the *Teensy MicroMod*: `cargo run --release --example teensy_micromod`.
