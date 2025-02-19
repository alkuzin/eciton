// src/main.rs

fn main() {
    // read env variables that were set in build script
    let bios_path = env!("BIOS_PATH");

    // choose whether to start the UEFI or BIOS image
    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}