# Install Rustup:
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install arm
# rustup target add thumbv6m-none-eabi

_A:=$(shell rustc -C debuginfo=1 \
		--target thumbv6m-none-eabi --emit asm \
		-o main.S --crate-type rlib --color=always main.rs)

TOP_DIR=../..
MODULE_NAME=mlrust
MODULE_OBJS=mlrust.o main.o
include $(TOP_DIR)/modules/Makefile.modules
