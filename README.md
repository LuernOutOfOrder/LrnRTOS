# LrnRTOS

A hybrid RTOS

## Project goal

The goal of this project is to explore a new way to create a kernel using a hybrid architecture based on a monolithic and a microkernel, to see if it's possible to develop a kernel for an RTOS with some features of a GPOS, with real-time constraints and security from user-space and kernel-space.
As this is my first real kernel, I want to make everything from scratch to really learn how a kernel works and take every chance to create something new. This includes parsing the FDT, IPC, and implementing a filesystem.

### Actual features

Features that are currently working:

- Platform layer using FDT or statically defined devices.
- Init drivers from platform layer.
- Sub-systems for each type of devices(serial, timer, etc).
- Machine memory handling.
- Traps handling(interruptions and exceptions handling).

### Current target

Target where the kernel can build, boot, and run:

- riscv32imc-unknown-none-elf

## Run

To build the kernel, use the following command:

```bash
make build
```

To run the kernel in qemu, use the following command:

```bash
make
```

## Makefile commands

List of other makefile commands:

```bash
# Run the kernel using config in makefile(qemu config and targeted binary)
make run

# Available flags for run commands:
# DEBUG=1 -> run qemu with gdb flags, use to debug using gdb from `make debug` command.
# DUMP_LOGS=1 -> dump logs from target binary into a out.log in logs/.
# DUMP_DTB=1 -> dump the dtb from qemu, generate a .dtb file in logs/. Make it readable using the `make dtc` command

# Build the kernel using cargo commands
make build

# Build the kernel for testing env
make test_build

# Check all kernel source code, correctly formated files and no warnings
make check

# Clean the logs and target directories
make clean

# Run gdb with target for debugging
make debug

# Run objdump and redirect output to a log.txt in logs/ dir from debug binary
make objdump

# Convert the dump DTB from qemu to a dts file
make dtc
```

## Documentation

All architecture and design choices are documented here: `Documentation/`.

A documentation for configuration and usage is coming soon.

## Resources

- Device tree specification: <https://devicetree-specification.readthedocs.io/en/stable/>
