extern crate bindgen;
extern crate cmake;

fn main() {
    let llvmc2_dir_out = cmake::Config::new("llvm_c_2")
        .build_target("llvm_c_2")
        .build();

    let cwd = std::env::current_dir().unwrap();
    let llvmc2_dir = cwd.join("llvm_c_2");
    let llvmc2_dir_include = llvmc2_dir.join("include");

    let llvm_libdir = get_llvm_libdir();

    println!(
        "cargo:rustc-link-search=native={}/build",
        llvmc2_dir_out.display()
    );
    println!("cargo:rustc-link-search={}", llvm_libdir);

    println!("cargo:rustc-link-lib=static=llvm_c_2");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=LLVM-13");

    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_args(&[&format!("-I{}", llvmc2_dir_include.display())])
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("bindings should be able to generate");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("bindings file should be able to be written");
}

fn get_llvm_libdir() -> String {
    let cmd = std::process::Command::new("llvm_c_2/llvm-config")
        .arg("--libdir")
        .output()
        .unwrap();
    String::from_utf8(cmd.stdout).unwrap()
}
