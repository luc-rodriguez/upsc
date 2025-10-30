use std::{env, path::PathBuf};
// use dircpy::copy_dir;

fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let sdk: String = format!("{}\\DLSS", dir);
    let vkh: String = format!("{}\\..\\Vulkan-Headers", dir);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-search=native={sdk}/lib/Linux_x86_64");
        println!("cargo:rustc-link-lib=static=nvsdk_ngx");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=dl");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=native={sdk}/lib/Windows_x86_64/x64");
        #[cfg(not(target_feature = "crt-static"))]
        println!("cargo:rustc-link-lib=static=nvsdk_ngx_d");
        #[cfg(target_feature = "crt-static")]
        println!("cargo:rustc-link-lib=static=nvsdk_ngx_s");
    }

    let mut builder = bindgen::Builder::default();

    #[cfg(feature = "vulkan")]
    {
        builder = builder
            .header(format!("{}\\src\\vk.h", dir))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .wrap_static_fns(true)
            .wrap_static_fns_path(out_dir.join("wrap_static_fns"))
            
            .clang_arg(format!("-I{sdk}\\include"))
            .allowlist_item(".*NGX.*")
            .blocklist_item(".*Cuda.*")
            .blocklist_item(".*CUDA.*")

            .clang_arg(format!("-I{vkh}\\include"))
            .blocklist_item("Vk.*")
            .blocklist_item("PFN_vk.*");
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("deps.rs"))
        .unwrap();

    cc::Build::new()
        .file(out_dir.join("wrap_static_fns.c"))
        .includes([
            format!("{sdk}\\include"),
            #[cfg(feature = "vulkan")]
            format!("{vkh}\\include"),
        ])
        .compile("wrap_static_fns");
}