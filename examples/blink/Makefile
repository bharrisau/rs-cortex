CC         := arm-none-eabi-gcc
AR         := arm-none-eabi-ar
RUSTC      := rustc
RUSTPKG    := rustpkg

OPT        := 2
ARCH       := thumbv6m
CPU        := cortex-m0
FLOAT      := soft

RUSTLIBS   :=-L ../../  -L ../../build -L build
RUSTFLAGS  = --target $(ARCH)-unknown-eabi -C target-cpu=$(CPU) $(RUSTLIBS) \
              --opt-level $(OPT) -C $(FLOAT)-float -g -Z no-landing-pads \
	      -A dead_code -A unused_variable -C llvm-args=-ffunction-sections
LDFLAGS    := -mcpu=cortex-m0plus -mthumb -O$(OPT) -nostartfiles \
              -ffreestanding -fno-builtin -Wl,-Map=blink.map \
	      -fno-asynchronous-unwind-tables -TMKL25Z64.ld startup.o \
	      --specs=nano.specs -Wl,--gc-sections

DEPS	   := build/libcompiler-rt.a build/libmorestack.a

all: build/blink

build/.touch:
	mkdir -p build
	touch build/.touch

build/libcompiler-rt.a: build/.touch
	$(AR) cr build/libcompiler-rt.a

build/libmorestack.a: build/.touch
	$(AR) cr build/libmorestack.a

build/blink.o: $(DEPS) main.rs
	$(RUSTC) $(RUSTFLAGS) --emit=obj --out-dir build -Z lto main.rs

build/blink: $(DEPS) main.rs
	$(RUSTC) $(RUSTFLAGS) --emit=link --out-dir build -Z lto -C linker=$(CC) -C link-args="$(LDFLAGS)" main.rs

clean:
	rm -f build/*

.PHONY: clean
