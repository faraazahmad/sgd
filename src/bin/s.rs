extern crate clap;
extern crate file;

use clap::{Arg, App};
use std::process::Command;

fn main(){
    let pwd = Command::new("pwd")
        .output()
        .expect("FAILED TO INSTALL");
    let pwd = String::from_utf8_lossy(&pwd.stdout);
    let pwd = pwd.trim().to_owned();
    let s = App::new("s")
        .version("v0.1")
        .author("Parker K")
        .about("Saves the current location")
        .arg(Arg::with_name("NAME")
            .help("Names the alias for this saved location")
            .required(true)
            .index(1)).
        get_matches();
    let save_name = s.value_of("NAME").unwrap().to_owned();
    //io(&save_name).expect("failed to find file");
    println!("{}", save_name);

}
//TODO: fix file pathing for writing to the saved locations text file
fn io(name: &String) -> file::Result<()>{
    let sgdloc = file::get_text("../sgd_location.txt")?;
    let savedloc = sgdloc + "src/saved_locations.txt";
    file::put(savedloc, name)?;
    Ok(())
}
