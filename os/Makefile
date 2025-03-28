# run cargo build
# look for the file format
# look for the file head
# compile the file to assembly file


FILE := target/riscv64gc-unknown-none-elf/debug/os
TARGET := riscv64gc-unknown-none-elf
BIOS := ../bootloader/rustsbi-qemu.bin

# Rule to run cargo
run:
	@cargo run $(FILE)

build:
	@cargo build --target $(TARGET)

format:
	@file $(FILE)

head:
	@rust-readobj -h $(FILE)
# Rule to run rust-objdump
objdump:
	@rust-objdump -S $(FILE)
qemu:
	# default memory 128M set by [-m 128M]    range[0x80000000, 0x88000000]
	# default entry=0x80000000 fixed by Qemu, we can't change it
	# set bootloader entry addr=0x80000000.
	# our RustSBI set the next stage entry addr=0x80200000
	# so we need to set the kernel entry addr=0x80200000
	@echo "Running QEMU..."
	@qemu-system-riscv64 -machine virt \
		-bios $(BIOS) \
		-nographic \
		-device loader,file=$(FILE),addr=0x80300000
	#qemu-system-riscv64 $(FILE)
	@echo "QEMU running. Press Ctrl+A, X to exit."

# Default target
all: build format head objdump