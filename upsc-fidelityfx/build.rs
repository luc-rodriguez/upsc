use std::{env, path::PathBuf};

fn main() {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");

    let sdk_dir = format!("{}\\FidelityFX-SDK\\sdk\\", cargo_manifest_dir);
    
    let vulkan_headers = format!("{cargo_manifest_dir}\\..\\Vulkan-Headers\\include");
}