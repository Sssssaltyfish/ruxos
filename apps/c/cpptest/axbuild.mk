C_COMPILER := $(shell which $(CC))
CXX_COMPILER := $(shell which $(CC))
AR := $(shell which $(AR))
CROSS_COMPILE_PATH := $(shell dirname $(C_COMPILER))/..
CXX_STD ?= 20

APP_CXXFLAGS := -std=c++$(CXX_STD) -Wall -Werror -c -nostdlib -static -no-pie -e main
APP_LDFLAGS := -nostdlib -static -no-pie -r -e main

source := $(APP)/main.cpp
app-objs := cpptest.o

build_dir := $(APP)/build

$(APP)/$(app-objs):
	export CC=$(C_COMPILER) && export CXX=$(CXX_COMPILER) && export AR=$(AR)
	mkdir -p $(build_dir)
	$(CXX_COMPILER) $(APP_CXXFLAGS) $(source) -o $(build_dir)/main.o
	mkdir -p $(build_dir)/libgcc && cd $(build_dir)/libgcc && \
		ln -s -f $(CROSS_COMPILE_PATH)/lib/gcc/*-linux-musl/*/libgcc.a ./ && \
		$(AR) x libgcc.a _clrsbsi2.o
	$(LD) -o $(app-objs) $(APP_LDFLAGS) \
		$(build_dir)/main.o \
		$(CROSS_COMPILE_PATH)/*-linux-musl/lib/libstdc++.a \
		$(CROSS_COMPILE_PATH)/lib/gcc/*-linux-musl/*/libgcc_eh.a \
		$(build_dir)/libgcc/_clrsbsi2.o

clean_c::
	rm -rf $(build_dir)/

.PHONY: build_cpptest clean_c
