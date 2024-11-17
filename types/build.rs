fn main() {
    let prefix = "../include/twizzler";
    let mut bg = std::process::Command::new("bindgen");
    bg.arg("--override-abi").arg(".*=C-unwind");
    bg.arg("--use-core");
    bg.arg(format!("{}/types.h", prefix));
    bg.arg("-o")
        .arg(format!("src/bindings.rs"))
        .arg("--")
        .arg("-nostdlibinc");
    let _status = bg.status().expect("failed to generate bindings");
    println!("cargo::rerun-if-changed=../include");
}
