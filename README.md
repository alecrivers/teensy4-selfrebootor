# teensy4-selfrebootor

[![Crates.io](https://img.shields.io/crates/v/teensy4-selfrebootor)](https://crates.io/crates/teensy4-selfrebootor)
[![Crates.io](https://img.shields.io/crates/d/teensy4-selfrebootor)](https://crates.io/crates/teensy4-selfrebootor)
[![License](https://img.shields.io/crates/l/teensy4-selfrebootor)](https://github.com/Finomnis/teensy4-selfrebootor/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/teensy4-selfrebootor/ci.yml)](https://github.com/Finomnis/teensy4-selfrebootor/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/teensy4-selfrebootor)](https://docs.rs/teensy4-selfrebootor)



# Examples

*- examples are intended for the [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) board -*

## Prerequisites

The following hardware is required for the examples:
- A [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) development board
- Optional: A way to read the Teensy's UART, like a USB-UART-converter

The following software tools have to be installed:
- Python3 (as `python3`, or modify `run.py` to use the `python` binary)
- `llvm-objcopy`
  - Install [`LLVM`](https://github.com/llvm/llvm-project/releases) tool suite
- [`teensy-loader-cli`](https://www.pjrc.com/teensy/loader_cli.html)


## Run

- Connect the Teensy to PC via USB cable.
- Run `cargo run --release --example rebootor`.
