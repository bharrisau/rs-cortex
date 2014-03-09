# Modified from github.com/metajack/rust-geom

RUST_LIBS :=
RUST_DEPS :=

define RUST_CRATE
_rust_crate_dir     := $(dir $(1))
_rust_crate_lib     := $$(_rust_crate_dir)lib.rs
_rust_crate_test    := $$(_rust_crate_dir)test.rs

_rust_crate_name    := $$(shell rustc --crate-name $$(_rust_crate_lib))

_rust_crate_rlib    := build/$$(shell rustc --crate-file-name $$(_rust_crate_lib))
LIB_$$(_rust_crate_name) := $$(_rust_crate_rlib)
RUST_LIBS += $$(_rust_crate_rlib)
RUST_DEPS += build/$$(_rust_crate_name).d

.PHONY : $$(_rust_crate_name)
$$(_rust_crate_name) : $$(_rust_crate_rlib)

$$(_rust_crate_rlib) : $$(_rust_crate_lib) $(2)
	$$(RUSTC) $$(RUSTFLAGS) --out-dir build $$< --dep-info

-include $$(patsubst %.rs,%.d,$$(_rust_crate_lib))

ifneq ($$(wildcard $$(_rust_crate_test)),"")

.PHONY : check-$$(_rust_crate_name)
check-$$(_rust_crate_name): $$(_rust_crate_name)-test
	./$$(_rust_crate_name)-test

$$(_rust_crate_name)-test : $$(_rust_crate_test)
	$$(RUSTC) $$(RUSTFLAGS) --dep-info --test $$< -o $$@

-include $$(patsubst %.rs,%.d,$$(_rust_crate_test))

endif

endef
