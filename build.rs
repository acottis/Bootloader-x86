use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let lib = "boot";
    let asm_dir = Path::new("asm");
    let asm_file = asm_dir.join(lib).with_extension("asm");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join(lib).with_extension("o");

    let output = Command::new("nasm")
        .args(&["-f", "elf32", "-o"])
        .arg(&out_file)
        .arg(&asm_file)
        .output()
        .expect("Failed to assemble ASM file");
    if !output.status.success() {
        panic!("NASM failed file {}: {:?}", asm_file.display(), output)
    }

    println!("cargo:rustc-link-arg={}", out_file.display());
    println!("cargo:rustc-link-arg=-Tlink.ld");
    println!("cargo:rustc-link-arg=-otarget/stage0.bin");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-nostdlib");
}
