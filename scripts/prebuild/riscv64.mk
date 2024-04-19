ifeq ($(MUSL),y)
define run_prebuild
  git submodule update --init --recursive --remote patches/opensbi
endef

RISCV_BIOS := $(CURDIR)/patches/opensbi/build/platform/generic/firmware/fw_dynamic.bin

$(RISCV_BIOS): prebuild
	CROSS_COMPILE=riscv64-linux-musl- $(MAKE) -C patches/opensbi PLATFORM=generic

build: $(RISCV_BIOS)

else

define run_prebuild
endef

endif
