include rust.mk

CC         := arm-none-eabi-gcc
RUSTC      := rustc
RUSTPKG    := rustpkg

OPT        := 2
ARCH       := thumbv6m
CPU        := cortex-m0
FLOAT      := soft

RUSTLIBS   := 
RUSTFLAGS  = --target $(ARCH)-linux-eabi --target-cpu $(CPU) --cfg libc $(RUSTLIBS) \
              --opt-level $(OPT) -Z $(FLOAT)-float -Z debug-info -Z no-landing-pads
LDFLAGS    := -mcpu=cortex-m0pllus -mthumb -O$(OPT) -nostartfiles \
              -ffreestanding -fno-builtin

all: cortex freescale usb

$(eval $(call RUST_CRATE, external/rust-core/core/))
$(eval $(call RUST_CRATE, src/cortex/, $(LIB_core)))
$(eval $(call RUST_CRATE, src/freescale/, $(LIB_core) $(LIB_cortex)))
$(eval $(call RUST_CRATE, src/usb/, $(LIB_core) $(LIB_freescale)))

clean:
	find . -name "lib.d" -delete
	find . -name "*.rlib" -delete
