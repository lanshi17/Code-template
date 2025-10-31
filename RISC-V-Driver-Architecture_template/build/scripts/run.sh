#!/bin/bash
qemu-system-riscv32 -machine virt -bios none -kernel target/riscv32imac-unknown-none-elf/release/riscv-driver-project
