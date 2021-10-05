ifneq ($(MAKECMDGOALS),clean)

_A:=$(shell rustc -C opt-level=2 \
		--target thumbv6m-none-eabi --emit asm \
		-o main.S --crate-type rlib --color=always main.rs;)

ifeq ($(.SHELLSTATUS),1)
$(error Rust error, quitting)
endif

# Instead of using compiler properly,
# patch some things with a script
_B:=$(shell python3 patch.py)

endif

TOP_DIR=../..
MODULE_NAME=mlrust
MODULE_OBJS=mlrust.o main.o
include $(TOP_DIR)/modules/Makefile.modules
