# LrnRTOS
A hybrid RTOS

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
