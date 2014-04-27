CC         := arm-none-eabi-gcc
RUSTC      := rustc
RUSTPKG    := rustpkg

OPT        := 2
ARCH       := thumbv6m
CPU        := cortex-m0
FLOAT      := soft

RUSTLIBS   :=-L ../../  -L ../../build
RUSTFLAGS  = --target $(ARCH)-unknown-eabi -C target-cpu=$(CPU) $(RUSTLIBS) \
              --opt-level $(OPT) -C $(FLOAT)-float -g -Z no-landing-pads \
	      -A dead_code -A unused_variable -C llvm-args=-ffunction-sections
LDFLAGS    := -mcpu=cortex-m0plus -mthumb -O$(OPT) -nostartfiles \
              -ffreestanding -fno-builtin -Wl,-Map=blink.map \
	      -fno-asynchronous-unwind-tables -TMKL25Z64.ld startup.o \
	      --specs=nano.specs -Wl,--gc-sections

all: build/blink

build:
	mkdir -p build

build/blink.o: build main.rs
	$(RUSTC) $(RUSTFLAGS) --emit=obj --out-dir build -Z lto main.rs

build/blink: build main.rs
	$(RUSTC) $(RUSTFLAGS) --emit=link --out-dir build -Z lto -C linker=$(CC) -C link-args="$(LDFLAGS)" main.rs

clean:
	rm -f build/*
