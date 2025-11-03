use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        dlss: {
            all(
                any(target_os = "windows", target_os = "linux"),
                any(target_arch = "x86_64", target_arch = "aarch64")
            )
        }
    }
}