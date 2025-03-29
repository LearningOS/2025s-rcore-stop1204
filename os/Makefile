# run cargo build
# look for the file format
# look for the file head
# compile the file to assembly file

DOCKER_NAME ?= rcore-tutorial-v3

FILE := target/riscv64gc-unknown-none-elf/debug/os
TARGET := riscv64gc-unknown-none-elf
BIOS := bootloader/rustsbi-qemu.bin


b:
	@cargo build --target $(TARGET)

# Rule to run cargo
run:
	@cargo run $(FILE)
format:
	@file $(FILE)

h:
	@rust-readobj -h $(FILE)
# Rule to run rust-objdump
d:
	@rust-objdump -S $(FILE)

bin:
	@rust-objcopy --binary-architecture=riscv64 $(FILE) \
		--strip-all -O binary $(FILE).bin
q:
	# default memory 128M set by [-m 128M]    range[0x80000000, 0x88000000]
	# default entry=0x80000000 fixed by Qemu, we can't change it
	# set bootloader entry addr=0x80000000.
	# our RustSBI set the next stage entry addr=0x80200000
	# so we need to set the kernel entry addr=0x80200000
	@echo "Running QEMU..."
	@qemu-system-riscv64 -machine virt \
		-nographic \
		-bios $(BIOS) \
		-device loader,file=$(FILE).bin,addr=0x80200000
	#qemu-system-riscv64 $(FILE)
	@echo "QEMU running. Press Ctrl+A, X to exit."

qemu:
	@qemu-riscv64 $(FILE)
qemu-version:
	@qemu-system-riscv64 --version
	@qemu-riscv64 --version

.PHONY: docker build_docker

docker:
	docker run --rm -it -v ${PWD}:/mnt -w /mnt ${DOCKER_NAME} bash

build_docker:
	docker build -t ${DOCKER_NAME} .

fmt:
	cd os ; cargo fmt;  cd ..



# Default target
all: b bin format h d