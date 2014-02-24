include rust.mk

CC         := arm-none-eabi-gcc
RUSTC      := rustc
RUSTPKG    := rustpkg

OPT        := 2
ARCH       := thumbv6m
CPU        := cortex-m0
FLOAT      := soft

RUSTLIBS   := -L .
RUSTFLAGS  = --target $(ARCH)-linux-eabi -C target-cpu=$(CPU) --cfg libc $(RUSTLIBS) \
              --opt-level $(OPT) -C $(FLOAT)-float -g -Z no-landing-pads \
	      -A dead_code -A unused_variable
LDFLAGS    := -mcpu=cortex-m0pllus -mthumb -O$(OPT) -nostartfiles \
              -ffreestanding -fno-builtin

all: cortex freescale usb

dist: all
	mkdir -p build
	cp -f $(RUST_LIBS) build/

$(eval $(call RUST_CRATE, src/cortex/, $(LIB_core)))
$(eval $(call RUST_CRATE, src/usb/, $(LIB_core)))
$(eval $(call RUST_CRATE, src/freescale/, $(LIB_core) $(LIB_cortex) $(LIB_usb)))

clean:
	rm -f $(RUST_LIBS) $(RUST_DEPS) build/*
