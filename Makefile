TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
OBJCOPY := rust-objcopy

$(KERNEL_BIN): kernel
	$(OBJCOPY) --binary-architecture=riscv64 $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@cargo build --release

KERNEL_ENTRY_PA := 0x80200000
BOARD                ?= qemu
SBI                  ?= rustsbi
BOOTLOADER   := ../rCore-Tutorial-Code-2024S/bootloader/$(SBI)-$(BOARD).bin

run: run-inner

run-inner: build
	qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios $(BOOTLOADER) \
        -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

build: $(KERNEL_BIN)