RBUILD_DIR = target/riscv32imc-unknown-none-elf/release/lrnrtos
DBUILD_DIR = target/riscv32imc-unknown-none-elf/debug/lrnrtos
DEBUG_GDB_FLAGS = -S -gdb tcp::1234
DEBUG_FLAGS = -d int -D out.log
QEMU_MACHINE = virt
QEMU_BIOS = default
QEMU_DUMP_DTB = ,dumpdtb=qemu_dtb.dtb
MEMORY_MAP_FLAGS = RUSTFLAGS="-Clink-arg=-Map=memory.map"

run:
	qemu-system-riscv32 -machine $(QEMU_MACHINE) -nographic -bios $(QEMU_BIOS) -kernel $(DBUILD_DIR)

rrun:
	qemu-system-riscv32 -machine $(QEMU_MACHINE) -nographic -bios default -kernel $(RBUILD_DIR)

clean:
	cargo clean
	rm out.log log.txt

build:
	cargo build --profile=dev

rbuild:
	cargo build --release

cbuild:
	cargo clean && cargo build --release

debug:
	riscv64-elf-gdb $(DBUILD_DIR)

objdump:
	objdump -Sr $(DBUILD_DIR) > log.txt
