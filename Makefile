RBUILD_DIR = target/riscv32imc-unknown-none-elf/release/lrnrtos
DBUILD_DIR = target/riscv32imc-unknown-none-elf/debug/lrnrtos
DEBUG_GDB_FLAGS = -S -gdb tcp::1234

run:
	qemu-system-riscv32 -M virt -nographic -bios default -kernel $(RBUILD_DIR)

drun:
	qemu-system-riscv32 -M virt -nographic -bios default -kernel $(DBUILD_DIR) -d int -D out.log

clean:
	cargo clean

dbuild:
	cargo build --profile=dev

rbuild:
	cargo build --release

cbuild:
	cargo clean && cargo build --release

debug:
	riscv64-elf-gdb $(DBUILD_DIR)
