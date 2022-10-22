GCCPATH = ../gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf/bin

BSP ?= rpi4

ifeq ($(BSP),rpi4)
    TARGET            = aarch64-unknown-none
    KERNEL_BIN        = kernel8.img
    OBJDUMP_BINARY    = $(GCCPATH)/aarch64-none-elf-objdump
    NM_BINARY         = $(GCCPATH)/aarch64-none-elf-nm
    READELF_BINARY    = $(GCCPATH)/aarch64-none-elf-readelf
    LD_SCRIPT_PATH    = $(shell pwd)
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72
endif

# Export for build.rs.
export LD_SCRIPT_PATH

##--------------------------------------------------------------------------------------------------
## Targets and Prerequisites
##--------------------------------------------------------------------------------------------------
KERNEL_LINKER_SCRIPT = kernel.ld

LAST_BUILD_CONFIG = target/$(BSP).build_config

KERNEL_ELF      = target/$(TARGET)/release/kernel
# This parses cargo's dep-info file.
# https://doc.rust-lang.org/cargo/guide/build-cache.html#dep-info-files
KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(LAST_BUILD_CONFIG)

##--------------------------------------------------------------------------------------------------
## Command building blocks
##--------------------------------------------------------------------------------------------------
RUSTFLAGS = $(RUSTC_MISC_ARGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT) 
	

RUSTFLAGS_PEDANTIC = $(RUSTFLAGS)

FEATURES      = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) $(FEATURES) --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy --strip-all -O binary

##--------------------------------------------------------------------------------------------------
## Targets
##--------------------------------------------------------------------------------------------------
.PHONY: all doc clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Save the configuration as a file, so make understands if it changed.
##------------------------------------------------------------------------------
$(LAST_BUILD_CONFIG):
	@rm -f target/*.build_config
	@mkdir -p target
	@touch $(LAST_BUILD_CONFIG)

##------------------------------------------------------------------------------
## Compile the kernel ELF
##------------------------------------------------------------------------------
$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)


##------------------------------------------------------------------------------
## Generate the stripped kernel binary
##------------------------------------------------------------------------------
$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	@echo $(KERNEL_BIN)
	@printf '%s KiB\n' `du -k $(KERNEL_BIN) | cut -f1`

##------------------------------------------------------------------------------
## Generate the documentation
##------------------------------------------------------------------------------
doc:
	@$(DOC_CMD) --document-private-items --open

##------------------------------------------------------------------------------
## Run clippy
##------------------------------------------------------------------------------
clippy:
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf target $(KERNEL_BIN) kernel.asm

##------------------------------------------------------------------------------
## Run readelf
##------------------------------------------------------------------------------
readelf: $(KERNEL_ELF)
	@$(DOCKER_TOOLS) $(READELF_BINARY) --headers $(KERNEL_ELF) > kernel.elf.header

##------------------------------------------------------------------------------
## Run objdump
##------------------------------------------------------------------------------
objdump: $(KERNEL_ELF)
	@$(OBJDUMP_BINARY) --disassemble --demangle \
                --section .text   \
                $(KERNEL_ELF) > kernel.asm

##------------------------------------------------------------------------------
## Run nm
##------------------------------------------------------------------------------
nm: $(KERNEL_ELF)
	@$(DOCKER_TOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt > kernel.nm

