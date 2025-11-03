use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        ngx: {
            all(
                any(target_os = "windows", target_os = "linux"),
                any(target_arch = "x86_64", target_arch = "aarch64")
            )
        },
        ffx: {
            all(
                target_arch = "x86_64",
                any(target_os = "windows", target_os = "linux")
            )
        },
        xess: {
            all(
                any(target_os = "windows", target_os = "linux"),
                any(target_arch = "x86_64", target_arch = "aarch64")
            )
        }
    }
}