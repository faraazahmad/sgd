extern crate clap;
extern crate file;
use std::convert::AsRef;
use std::ffi::OsString;
use std::process::Command;

fn main() {
    let pwd = Command::new("pwd")
        .output()
        .expect("FAILED TO INSTALL");
    let pwd = String::from_utf8_lossy(&pwd.stdout);
    let pwd = pwd.trim().to_owned();
    //io(&pwd); 

    Command::new("mv")
        .arg("target/debug/s")
        .arg("/usr/local/bin")
        .spawn()
        .expect("FAILED TO INSTALL");
    println!("Building...");
    
}


fn io(wd: &String) -> file::Result<()>{
    let wd = wd.to_owned();
    file::put("src/sgd_location.txt", wd)?;
    Ok(())
}
