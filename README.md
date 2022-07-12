# llvm-rs

Rust bindings for [`llvm_c_2`](https://github.com/tealsnow/llvm_c_2)

## Building

Building does not necessarily require nix, but it does make everything a lot easier.

To build using nix is as such:

``` sh
nix-shell
make
```

### Without Nix

#### Requirements

- deno
- ninja
- cmake
- clang-13 or later
- libclang-13
- libllvm-13
- rust

#### Building

If you have all of the requirements running `make` should work.

