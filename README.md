# LrnRTOS
A hybrid RTOS

## Project's goal 

The goal of this project is to explore a new way to create a kernel using a hybrid architecture based on a monolithic and a microkernel, to see if it's possible to develop a kernel for an RTOS with some features of a GPOS, with real-time constraints and security from user-space and kernel-space.
Being my first real kernel, wanting to really learn how a kernel works, and taking every chance to create something new. I want to make everything from scratch. From parsing the FDT to IPC passing by a filesystem.

### Actual features:

Features that are currently working:

- Parsing FDT.
- Init drivers from parsed node in FDT.
- Printing with format from core::fmt::Write using an initialized driver.

### Current target:

Target where the kernel can build, boot, and run:

- riscv32imc-unknown-none-elf

## Run

To build the kernel, use the following command:

```bash
make rbuild
```

To run the kernel in qemu, use the following command:

```bash
make run
```

## Makefile commands

List of other makefile commands:

```bash
# Clean the target folder and generate log files.
make clean

# Build for debug
make dbuild

# Debug run collecting basic log from qemu
make drun

# Clean target folder and build for release
make cbuild

# Run gdb with debug target for debugging
make debug

# Run objdump and redirect output to a log.txt from debug binary
make objdump
```

## Resources

- Device tree specification: https://devicetree-specification.readthedocs.io/en/stable/
