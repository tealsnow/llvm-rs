.PHONY: all setup clean llvm_c_2 llvm_c_2-sys llvm-rs

all: llvm-rs

setup: llvm_c_2

clean:
	cd ./llvm_c_2; ./rnn.ts clean
	cargo clean

llvm_c_2:
	git submodule update --init
	cd ./llvm_c_2; ./rnn.ts build

llvm_c_2-sys: llvm_c_2

llvm-rs: llvm_c_2-sys
	cargo build
