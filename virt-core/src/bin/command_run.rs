use std::process::Command;

fn main() {
    Command::new("C:/Users/dillb/Documents/Rust_Projects/virt/target/debug/main.exe")
    .spawn()
    .unwrap();
}