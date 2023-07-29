#!/usr/bin/env python

from pathlib import Path
from tempfile import TemporaryDirectory

import subprocess
import sys


def main():
    if len(sys.argv) < 2:
        print("Please provide the binary as first argument!")
        exit(1)

    binary = sys.argv[1]

    with TemporaryDirectory() as tmpdir:
        tmpdir = Path(tmpdir)
        hexfile = tmpdir / "firmware.hex"

        subprocess.run(["llvm-objcopy", "-O", "ihex", binary, hexfile], check=True)
        subprocess.run(["teensy_loader_cli", "--mcu=TEENSY_MICROMOD", "-wrv", hexfile], check=True)

    print("Teensy successfully flashed.")


if __name__ == "__main__":
    main()