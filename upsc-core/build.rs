use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        dlss: { any(windows, target_os = "linux") }
    }
}