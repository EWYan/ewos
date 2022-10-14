TARGET = aarch64-unknown-none
SFILES = $(wildcard ./boot/*.S)
OFILES = $(SFILES:.S=.o)
GCCPATH = ../gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf/bin
GCCFLAGS = -Wall -O2 -ffreestanding -nostdinc -nostdlib -nostartfiles

RUST_PART := target/$(TARGET)/release/libewos.a

.PHONY: all clean

$(OFILES): $(SFILES)
	@$(GCCPATH)/aarch64-none-elf-gcc $(GCCFLAGS) -c $< -o $@

all: kernel $(OFILES) $(RUST_PART)
	@$(GCCPATH)/aarch64-none-elf-ld -nostdlib $(OFILES) $(RUST_PART) -T linker.ld -o ewos.elf
	@$(GCCPATH)/aarch64-none-elf-objdump -D ./ewos.elf > ewos.asm
	@$(GCCPATH)/aarch64-none-elf-objcopy -O binary ./ewos.elf ./kernel8.img
	@echo build completed!

kernel:
	@cargo build --release --target $(TARGET)

clean:
	@rm -rf target/
	@rm -rf boot/*.o
	@rm -rf *.elf *.asm *.img
