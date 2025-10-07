BUILD_DIR = target/riscv32imc-unknown-none-elf/release/lrnrtos

run:
	qemu-system-riscv32 -M virt -nographic -bios default -kernel $(BUILD_DIR)

clean:
	cargo clean

build:
	cargo build --release

cbuild:
	cargo clean && cargo build --release
