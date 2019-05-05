TARGET = main
# Define the linker script location and chip architecture.
LD_SCRIPT = ./boot/stm32f446.ld
MCU_SPEC  = cortex-m4
DEBUG = -g
# Toolchain definitions (ARM bare metal defaults)
# TOOLCHAIN_PATH = /home/olivier/gcc-arm-none-eabi-8-2018-q4-major/bin/
CC = $(TOOLCHAIN_PATH)arm-none-eabi-gcc
AS = $(TOOLCHAIN_PATH)arm-none-eabi-as
LD = $(TOOLCHAIN_PATH)arm-none-eabi-ld
OC = $(TOOLCHAIN_PATH)arm-none-eabi-objcopy
OD = $(TOOLCHAIN_PATH)arm-none-eabi-objdump
OS = $(TOOLCHAIN_PATH)arm-none-eabi-size
# Assembly directives.
ASFLAGS += -c
ASFLAGS += -O0
ASFLAGS += -mcpu=$(MCU_SPEC)
ASFLAGS += -mthumb
ASFLAGS += -Wall
# (Set error messages to appear on a single line.)
ASFLAGS += -fmessage-length=0
# RUST compilation directives
# RUST_FLAGS += --emit=obj
RUST_FLAGS += -C panic=abort
RUST_FLAGS += -C opt-level=0
RUST_FLAGS += -C debuginfo=2
RUST_FLAGS += --target=thumbv7em-none-eabihf
# RUST_FLAGS += -C lto=yes
RUST_FLAGS += -C codegen-units=1
RUST_FLAGS_BIN += --crate-type=staticlib
RUST_FLAGS_LIB += --crate-type=lib
RUST_LIB_NAME += HAL
RUST_LIB += ./hal/src/lib.rs
RUST_LIB_BIN += $(RUST_LIB:.rs=.rlib)
RUST_INCLUDE_LIB += --extern $(RUST_LIB_NAME)=$(RUST_LIB_BIN)
# Linker directives.
LSCRIPT = ./$(LD_SCRIPT)
LFLAGS += -mcpu=$(MCU_SPEC)
LFLAGS += -mthumb
LFLAGS += -Wall
LFLAGS += --specs=nosys.specs
LFLAGS += -nostdlib
LFLAGS += -lgcc
LFLAGS += -T$(LSCRIPT)
VECT_TBL = ./boot/vector_table.S
AS_SRC   = ./boot/core.S
RUST_SRC += ./src/main.rs
RUST_FILES = $(shell find -type f -name '*.rs')
OBJS =  $(VECT_TBL:.S=.o)
OBJS += $(AS_SRC:.S=.o)
OBJS += $(RUST_SRC:.rs=.o)
INCLUDE =
.PHONY: all
all: $(TARGET).elf
%.o: %.S
	$(CC) -mfloat-abi=hard $(DEBUG) $(ASFLAGS) $< -o $@
%.o: %.rs lib_kernel lib_hal_alloc
	rustc $(RUST_FLAGS) $(RUST_FLAGS_BIN) --extern allocator=./allocator/src/lib.rlib --extern hal=./hal/src/lib.rlib --extern kernel=./kernel/src/lib.rlib --extern hal_alloc=./hal_alloc/src/lib.rlib $< -o $@ 
$(TARGET).elf: $(OBJS)
	$(CC) -mfloat-abi=hard $^ $(LFLAGS) -o $@
	
.PHONY: lib_kernel
lib_kernel: lib_hal lib_allocator
	rustc $(RUST_FLAGS) $(RUST_FLAGS_LIB) --crate-name=kernel --extern allocator=./allocator/src/lib.rlib --extern hal=./hal/src/lib.rlib ./kernel/src/lib.rs -o ./kernel/src/lib.rlib

.PHONY: lib_hal_alloc
lib_hal_alloc: lib_hal lib_allocator
	rustc $(RUST_FLAGS) $(RUST_FLAGS_LIB) --crate-name=hal_alloc --extern allocator=./allocator/src/lib.rlib --extern hal=./hal/src/lib.rlib ./hal_alloc/src/lib.rs -o ./hal_alloc/src/lib.rlib

.PHONY: lib_hal
lib_hal:
	rustc $(RUST_FLAGS) $(RUST_FLAGS_LIB)  --crate-name=hal ./hal/src/lib.rs -o ./hal/src/lib.rlib

.PHONY: lib_allocator
lib_allocator:
	rustc $(RUST_FLAGS) $(RUST_FLAGS_LIB) --crate-name=allocator ./allocator/src/lib.rs -o ./allocator/src/lib.rlib

.PHONY: clean
clean:
	rm $(OBJS) $(TARGET).elf $(RUST_LIB_BIN); echo

.PHONY: remake
remake: clean all