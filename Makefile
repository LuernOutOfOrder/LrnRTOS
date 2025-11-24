# Build dir path
BUILD_PROFILE = debug
BUILD_ARCH = riscv32imc-unknown-none-elf
BUILD_DIR = target/$(BUILD_ARCH)/$(BUILD_PROFILE)/lrnrtos
# Runner (like qemu)
# Becarefull of the config, use correct flag for correct runner
RUNNER = qemu-system-riscv32
QEMU_MACHINE = virt
QEMU_BIOS = none
QEMU_DUMP_DTB = ,dumpdtb=qemu_dtb.dtb
# Debugger(like gdb)
DEBUGGER = riscv64-elf-gdb
# Debug flags (for debugger or runner)
DEBUG_GDB_FLAGS = -S -gdb tcp::1234
DEBUG_FLAGS = -d int -D out.log

# Check bin in $PATH
RUNNER_EXISTS := $(shell which $(RUNNER))

ifeq ($(RUNNER_EXISTS),)
$(error "Runner $(RUNNER) not found in PATH")
endif

DEBUGGER_EXISTS := $(shell which $(DEBUGGER))

ifeq ($(DEBUGGER_EXISTS),)
$(error "Debugger $(DEBUGGER) not found in PATH")
endif


run:
	$(RUNNER) -machine $(QEMU_MACHINE) -nographic -bios $(QEMU_BIOS) -kernel $(BUILD_DIR)

debug:
	$(DEBUGGER) $(BUILD_DIR)

objdump:
	objdump -Sr $(DBUILD_DIR) > log.txt
