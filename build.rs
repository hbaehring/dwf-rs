use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // cfg!(target_os) in a build script describes the host, not the target.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let sdk_include_path = match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-arg=-Wl,-rpath,/Library/Frameworks");
            println!("cargo:rustc-link-search=framework=/Library/Frameworks");
            println!("cargo:rustc-link-lib=framework=dwf");
            "/Library/Frameworks/dwf.framework/Headers"
        }
        "windows" => {
            let lib_arch = match target_arch.as_str() {
                "x86" => "x86",
                "x86_64" => "x64",
                "aarch64" => "arm64",
                "arm64ec" => "arm64ec",
                _ => unimplemented!("Unsupported Windows architecture: {}", target_arch),
            };
            println!(
                "cargo:rustc-link-search=native=C:\\Program Files (x86)\\Digilent\\WaveFormsSDK\\lib\\{}",
                lib_arch
            );
            println!("cargo:rustc-link-lib=dylib=dwf");
            "C:\\Program Files (x86)\\Digilent\\WaveFormsSDK\\inc"
        }
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=dwf");
            "/usr/include/digilent/waveforms"
        }
        _ => unimplemented!("Only Linux, macOS and Windows are supported!"),
    };

    // Fall back to the bundled header so builds without the SDK (docs.rs, CI) still get bindings.
    let include_path = if Path::new(sdk_include_path).exists() {
        PathBuf::from(sdk_include_path)
    } else {
        println!(
            "cargo:warning=WaveForms SDK headers not found at {}, using bundled includes/dwf.h",
            sdk_include_path
        );
        manifest_dir.join("includes")
    };

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.display()))
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
