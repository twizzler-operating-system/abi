fn main() {
    let prefix = "include/twizzler/rt";
    let mut bg = std::process::Command::new("bindgen");
    bg.arg("--override-abi").arg(".*=C-unwind");
    bg.arg("--use-core");
    bg.arg(format!("{}/__all.h", prefix));
    bg.arg("-o").arg(format!("src/bindings.rs")).arg("--").arg("-nostdlibinc");
    let _status = bg.status().expect("failed to generate bindings");
}
