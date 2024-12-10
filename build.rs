use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Create output directories
    let firmware_dir = out_dir.join("cyw43-firmware");
    fs::create_dir_all(&firmware_dir).unwrap();

    // Download firmware
    let firmware_files = [
        (
            "https://github.com/embassy-rs/embassy/raw/refs/heads/main/cyw43-firmware/43439A0.bin",
            "F43439A0_bin",
        ),
        (
            "https://github.com/embassy-rs/embassy/raw/refs/heads/main/cyw43-firmware/43439A0_clm.bin",
            "F43439A0_clm_bin",
        ),
        (
            "https://github.com/embassy-rs/embassy/raw/refs/heads/main/cyw43-firmware/43439A0_btfw.bin",
            "F43439A0_btfw_bin",
        ),
    ];

    let mut bindings = String::new();
    for (url, const_name) in firmware_files {
        let response = reqwest::blocking::get(url).unwrap();
        let bytes = response.bytes().unwrap();
        let hex_bytes = bytes
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(", ");

        bindings.push_str(&format!(
            "pub const {}: &[u8] = &[{}];\n",
            const_name, hex_bytes
        ));
    }

    // Write firmware bindings to a Rust file
    let firmware_bindings = out_dir.join("firmware.rs");
    fs::write(firmware_bindings, bindings).unwrap();

    // Handle memory.x
    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // Inform the linker about the custom memory layout
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Ensure rebuilds when memory.x changes
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");

    // Link arguments
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
