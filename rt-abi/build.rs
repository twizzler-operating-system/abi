fn main() {
    let headers = std::env::var("TWIZZLER_ABI_BUILTIN_HEADERS").ok();
    let sysroots = std::env::var("TWIZZLER_ABI_SYSROOTS").ok();
    let target = std::env::var("TARGET").unwrap();

    let path = std::env::var("PATH").unwrap();
    std::env::set_var("PATH", format!("toolchain/install/bin:{}", path));

    let prefix = "../include/twizzler/rt";
    let mut bg = std::process::Command::new("bindgen");
    if let Some(val) = std::env::var("TWIZZLER_ABI_LLVM_CONFIG").ok() {
        bg.env("LLVM_CONFIG_PATH", val);
    }
    bg.arg("--override-abi").arg(".*=C-unwind");
    bg.arg("--use-core");
    bg.arg(format!("{}/__all.h", prefix));
    bg.arg("-o")
        .arg(format!("src/bindings.rs"))
        .arg("--")
        .arg("-target")
        .arg(&target);

    if headers.is_some() {
        bg.arg("-nostdinc");
    } else {
        bg.arg("-nostdlibinc");
    }

    if let Some(headers) = headers {
        bg.arg("-I").arg(headers);
    }

    if let Some(sysroots) = sysroots {
        let sysheaders = format!("{}/{}", sysroots, target);
        bg.arg("-I").arg(sysheaders);
    }
    let status = bg.status().expect("failed to generate bindings");
    if !status.success() {
        panic!("failed to generate bindings");
    }
    println!("cargo::rerun-if-changed=../include");
}
