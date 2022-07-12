use std::process::Command;

extern crate bindgen;

fn main() {
    let llvm_libdir = Command::new("../llvm_c_2/llvm-config")
        .arg("--libdir")
        .output()
        .unwrap();
    let llvm_libdir = String::from_utf8(llvm_libdir.stdout).unwrap();

    let cwd = std::env::current_dir().unwrap();

    let llvmc2_out_dir = cwd.join("../llvm_c_2/build");
    let llvmc2_include_dir = cwd.join("../llvm_c_2/include");

    println!("cargo:rustc-link-search={}", llvm_libdir);
    println!("cargo:rustc-link-search={}", llvmc2_out_dir.display());

    println!("cargo:rustc-link-lib=llvm_c_2");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=LLVM-13");

    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_args(&[&format!("-I{}", llvmc2_include_dir.display())])
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
