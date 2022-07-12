{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "llvm-rs env";

  # Build-time deps
  nativeBuildInputs = [
    # llvm_c_2 deps
    pkgs.deno
    pkgs.ninja
    pkgs.cmake
    pkgs.clang_13

    pkgs.git
    pkgs.rustup
    pkgs.llvmPackages_13.libclang
  ];

  # Run/link time deps
  buildInputs = [
    pkgs.llvmPackages_13.libraries.libcxx
    pkgs.llvmPackages_13.libraries.libcxxabi
    pkgs.llvmPackages_13.libllvm
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages_13.libclang.lib}/lib";
}
