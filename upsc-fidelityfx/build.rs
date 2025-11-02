use std::{env, path::PathBuf};

fn main() {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");

    let sdk_dir = format!("{}\\FidelityFX-SDK\\sdk\\", cargo_manifest_dir);
    
    let vulkan_headers = format!("{cargo_manifest_dir}\\..\\Vulkan-Headers\\include");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut builder = bindgen::Builder::default()
        // ...
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"))
        // ...
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false) // Not the default, but changes nothing
        .clang_arg("-xc++")
        .clang_args([
            "-xc++",
            format!("-I{sdk_dir}\\include").as_str(),
        ])
        .trust_clang_mangling(false)
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .allowlist_recursively(false)
        .dynamic_library_name("Functions")
        .dynamic_link_require_all(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        // ...
        .header(format!("{sdk_dir}\\include\\FidelityFX\\host\\ffx_interface.h"))
        // Unlike all other bindings
        ;

    if cfg!(not(windows)) {
        builder = builder
            .clang_args([
                "-DFFX_GCC",
            ]);
    }

    let components = vec![   
        // fsr1 and fsr2 need to be enabled (though don't need to have bindings) to get access
        // to fsr3 shaders (see hardcoded "shared" implementation in ffxGetPermutationBlobByIndex())
        "fsr1",
        "fsr2",
        "fsr3",
        "fsr3upscaler",
        "opticalflow",
        "frameinterpolation",
    ];

    for c in components.iter() {
        let f = format!("{sdk_dir}\\include\\FidelityFX\\host\\ffx_{c}.h");

        builder = builder
            .header(&f)
            .allowlist_file(&f);
    }

    builder = builder
        // These are specific per component, but it's harmless to pass them to other bindgen instances
        .bitfield_enum("FfxOpticalflowInitializationFlagBits")
        .bitfield_enum("FfxFrameInterpolationInitializationFlagBits")
        .bitfield_enum("FfxFrameInterpolationDispatchFlags")
        .bitfield_enum("FfxFsr1InitializationFlagBits")
        .bitfield_enum("FfxFsr2InitializationFlagBits")
        .bitfield_enum("FfxFsr3InitializationFlagBits")
        .bitfield_enum("FfxFsr3UpscalerInitializationFlagBits")
        .bitfield_enum("FfxFsr3UpscalerDispatchFlags")
        .bitfield_enum("FfxFsr3FrameGenerationFlags")
        .bitfield_enum("FfxFsr3UpscalingFlags");

    /*
    components.iter().for_each(|c| {
        let f = format!("{sdk_dir}\\include\\FidelityFX\\host\\ffx_{c}.h");

        builder = builder
            .header(f)
            .allowlist_file(f);
    });
    */

    #[cfg(feature = "vulkan")]
    {
        let ffx_vk = format!("{sdk_dir}\\include\\FidelityFX\\host\\backends\\vk\\ffx_vk.h");

        builder = builder
            .clang_arg(format!("-I{vulkan_headers}"))
            .header(&ffx_vk)
            .allowlist_file(&ffx_vk)
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();

    cc::Build::new()
        .file(out_dir.join("wrap_static_fns.c"))
        .includes([
            #[cfg(feature = "vulkan")]
            vulkan_headers,
        ])
        .compile("wrap_static_fns");

    /*
    let mut builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"))
        .header(format!("{sdk_dir}\\include\\FidelityFX\\host\\ffx_interface.h"));

    #[cfg(feature = "vulkan")]
    {
        builder = builder
            .headers([
                format!("{vulkan_headers}\\vulkan\\vulkan.h"),
            ])
            .clang_arg(format!("-I{vulkan_headers}"))
            .blocklist_item("Vk.*")
            .blocklist_item("PFN_vk.*");
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();

    cc::Build::new()
        .file(out_dir.join("wrap_static_fns.c"))
        .includes([
            #[cfg(feature = "vulkan")]
            vulkan_headers,
        ])
        .compile("wrap_static_fns");
    */
}

/*
#[derive(Debug)]
struct FFxFmt;

impl bindgen::callbacks::ParseCallbacks for FFxFmt {
    fn item_name(&self, _item_info: bindgen::callbacks::ItemInfo) -> Option<String> {

    }
}
*/

/*
fn bindgen_def(sdk_dir: &String) -> bindgen::Builder {
    let mut builder = bindgen::Builder::default()
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false) // Not the default, but changes nothing
        .clang_arg("-xc++")
        .clang_args([
            "-xc++",
            format!("-I{sdk_dir}\\include").as_str(),
        ])
        .trust_clang_mangling(false)
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .allowlist_recursively(false)
        .dynamic_library_name("Functions")
        .dynamic_link_require_all(true)
        // .parse_callbacks(Box::new(   ));
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        });

    if cfg!(not(windows)) {
        builder = builder
            .clang_args([
                "-DFFX_GCC",
            ]);
    }

    builder
}
*/