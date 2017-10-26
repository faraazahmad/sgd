extern crate clap;
extern crate file;

use clap::{Arg, App};
use std::process::Command;
use std::env;

fn main(){
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
    println!("{}", save_name);
    let cur = env::current_dir().unwrap();
    let cur = cur.to_str().unwrap();

    let home = env::home_dir().unwrap();
    let home = home.to_str().unwrap();
    println!("{}", home);
    
    file::put(home.to_owned() + "/sgd/src/saved_locations.txt", save_name + "=" + cur);
}

