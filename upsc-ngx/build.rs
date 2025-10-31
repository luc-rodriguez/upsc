use std::{env, path::PathBuf};
// use dircpy::copy_dir;

fn main() {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");

    let sdk_dir = format!("{cargo_manifest_dir}\\DLSS");
    let headers: String = format!("{sdk_dir}\\include");

    let vulkan_headers = format!("{cargo_manifest_dir}\\..\\Vulkan-Headers\\include");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-search=native={sdk_dir}/lib/Linux_x86_64");
        println!("cargo:rustc-link-lib=static=nvsdk_ngx");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=dl");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=native={sdk_dir}/lib/Windows_x86_64/x64");
        #[cfg(not(target_feature = "crt-static"))]
        println!("cargo:rustc-link-lib=static=nvsdk_ngx_d");
        #[cfg(target_feature = "crt-static")]
        println!("cargo:rustc-link-lib=static=nvsdk_ngx_s");
    }

    let mut builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"))
        .clang_arg(format!("-I{headers}"))
        .allowlist_item(".*NGX.*")
        .blocklist_item(".*Cuda.*")
        .blocklist_item(".*CUDA.*");

    #[cfg(feature = "vulkan")]
    {
        builder = builder
            .headers([
                format!("{vulkan_headers}\\vulkan\\vulkan.h"),
                format!("{headers}\\nvsdk_ngx_helpers.h"),
                format!("{headers}\\nvsdk_ngx_helpers_dlssd.h"),
                format!("{headers}\\nvsdk_ngx_helpers_vk.h"),
                format!("{headers}\\nvsdk_ngx_helpers_dlssd_vk.h"),
            ])
            .clang_arg(format!("-I{vulkan_headers}"))
            .blocklist_item("Vk.*")
            .blocklist_item("PFN_vk.*");
    }

    /*
    #[cfg(feature = "dx12")]
    {
        #include "nvsdk_ngx.h"
        #include "nvsdk_ngx_defs.h"
        #include "nvsdk_ngx_params.h"
        #include "nvsdk_ngx_helpers.h"

        builder = builder
            .header(format!("{}\\src\\dx12.h", cargo_manifest_dir));
    }
    */

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();

    cc::Build::new()
        .file(out_dir.join("wrap_static_fns.c"))
        .includes([
            headers,
            #[cfg(feature = "vulkan")]
            vulkan_headers,
        ])
        .compile("wrap_static_fns");
}