TARGET = aarch64-unknown-none
SFILES = $(wildcard *.S)
OFILES = $(SFILES:.S=.o)
GCCPATH = ../gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf/bin
GCCFLAGS = -Wall -O2 -ffreestanding -nostdinc -nostdlib -nostartfiles
RUST_PART := target/$(TARGET)/debug/libewos.a

$(info $(SFILES))
$(info $(OFILES))
.PHONY: all clean

$(OFILES): $(SFILES)
	@$(GCCPATH)/aarch64-none-elf-gcc $(GCCFLAGS) -c $< -o $@

all: kernel $(OFILES) $(RUST_PART)
	@$(GCCPATH)/aarch64-none-elf-ld -nostdlib $(OFILES) $(RUST_PART) -T linker.ld -o ewos.elf
	@aarch64-linux-gnu-objdump -D ./ewos.elf > ewos.asm
	@aarch64-linux-gnu-objcopy -O binary ./ewos.elf ./kernel8.img
	@echo build completed!

kernel:
	@cargo build --target $(TARGET)

clean:
	@rm -rf target/
	@rm -rf *.o *.elf
