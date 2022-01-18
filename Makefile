RARCH=thumbv6m-none-eabi
RFLAGS=-C opt-level=2 --target $(RARCH) --emit asm --crate-type rlib
main.S: src/main.rs src/ml.rs
	rustc $(RFLAGS) -o main.S src/main.rs
	python3 patch.py

rclean: clean
	rm -rf *.S

rinstall_qemu: main.S
	$(MAKE) install_qemu

TOP_DIR=../..
MODULE_NAME=mlrust
MODULE_OBJS=mlrust.o main.o
include $(TOP_DIR)/modules/Makefile.modules
