use std::process::Command;

pub fn main() {
    Command::new("make")
        .args(&["-c", "../../lib/asc-coredump"])
        .status()
        .expect("failed to make");
    Command::new("cp")
        .args(&["../../lib/asc-coredump/build/runtime.wasm", "."])
        .status()
        .expect("failed to cp");
}
