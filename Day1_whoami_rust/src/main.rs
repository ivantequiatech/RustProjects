//Day 1: System Tool by IvanTech
use std::process::Command:
fn main(){
    let output = Command::new("whoami").output().expect("Failed to execute command!");
    let user = String::from_utf8_lossy(&output.stdout);

    println!("--- HostPapa System Check ---");
    println!("Current User: {}", user.trim());
    println!("System Status: READY");
}
