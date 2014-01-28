# Modified from github.com/metajack/rust-geom

RUST_CRATE_PKGID = $(shell sed -ne 's/^\#\[ *crate_id *= *"\(.*\)" *];$$/\1/p' $(firstword $(1)))
RUST_CRATE_PATH = $(shell printf $(1) | sed -ne 's/^\([^\#]*\)\/.*$$/\1/p')
RUST_CRATE_NAME = $(shell printf $(1) | sed -ne 's/^\([^\#]*\/\)\{0,1\}\([^\#]*\).*$$/\2/p')
RUST_CRATE_VERSION = $(shell printf $(1) | sed -ne 's/^[^\#]*\#\(.*\)$$/\1/p')
RUST_CRATE_HASH = $(shell printf $(strip $(1)) | shasum -a 256 | sed -ne 's/^\(.\{8\}\).*$$/\1/p')

define RUST_CRATE
_rust_crate_dir     := $(dir $(1))
_rust_crate_lib     := $$(_rust_crate_dir)lib.rs
_rust_crate_test    := $$(_rust_crate_dir)test.rs

_rust_crate_pkgid   := $$(call RUST_CRATE_PKGID, $$(_rust_crate_lib))
_rust_crate_name    := $$(call RUST_CRATE_NAME, $$(_rust_crate_pkgid))
_rust_crate_version := $$(call RUST_CRATE_VERSION, $$(_rust_crate_pkgid))
RUSTLIBS += -L $$(_rust_crate_dir)

ifeq ($$(strip $$(_rust_crate_version)),)
	_rust_crate_version := 0.0
endif

_rust_crate_hash    := $$(call RUST_CRATE_HASH, $$(_rust_crate_name)\#$$(_rust_crate_version))
_rust_crate_rlib    := $$(_rust_crate_dir)lib$$(_rust_crate_name)-$$(_rust_crate_hash)-$$(_rust_crate_version).rlib
LIB_$$(_rust_crate_name) := $$(_rust_crate_rlib)

.PHONY : $$(_rust_crate_name)
$$(_rust_crate_name) : $$(_rust_crate_rlib)

$$(_rust_crate_rlib) : $$(_rust_crate_lib) $(2)
	$$(RUSTC) $$(RUSTFLAGS) $$< --dep-info

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
