# Build dir path
BUILD_PROFILE = debug
BUILD_ARCH = riscv32imc-unknown-none-elf
BUILD_DIR = target/$(BUILD_ARCH)/$(BUILD_PROFILE)/lrnrtos
# Runner (like qemu)
# Becarefull of the config, use correct flag for correct runner
RUNNER = qemu-system-riscv32
QEMU_MACHINE = virt
QEMU_BIOS = none
# Debugger(like gdb)
DEBUGGER = riscv64-elf-gdb

# Check bin in $PATH
RUNNER_EXISTS := $(shell which $(RUNNER))
DEBUGGER_EXISTS := $(shell which $(DEBUGGER))

# Condition to check if binary exist

ifeq ($(RUNNER_EXISTS),)
$(error "Runner $(RUNNER) not found in PATH")
endif

ifeq ($(DEBUGGER_EXISTS),)
$(error "Debugger $(DEBUGGER) not found in PATH")
endif

# Condition to use flags or not

ifeq ($(DEBUG),1)
DEBUG_RUN_FLAGS += -S -gdb tcp::1234
endif

ifeq ($(DUMP_LOGS),1)
DUMP_RUN_FLAGS += -d int -D logs/out.log
endif

ifeq ($(DUMP_DTB),1)
DUMP_DTB_RUN_FLAGS += ,dumpdtb=logs/qemu_dtb.dtb
endif

run:
	$(RUNNER) -machine $(QEMU_MACHINE)$(DUMP_DTB_RUN_FLAGS) -nographic -bios $(QEMU_BIOS) -kernel $(BUILD_DIR) $(DEBUG_RUN_FLAGS) $(DUMP_RUN_FLAGS)

debug:
	$(DEBUGGER) $(BUILD_DIR)

objdump:
	objdump -Sr $(DBUILD_DIR) > log.txt

dtc:
	dtc -I dtb -O dts logs/qemu_dtb.dtb > logs/qemu_dtb.dts

clean:
	rm -rf target/*
	rm -rf logs/*
