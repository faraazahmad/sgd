use std::process::Command;

fn main(){
    println!("Building...");
    Command::new("mv")
        .arg("./target/debug/s")
        .arg("/usr/local/bin/s")
        .output()
        .expect("Failed to install.");
}
