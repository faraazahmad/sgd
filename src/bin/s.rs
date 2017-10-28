//saves locations and files
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
    let cur = env::current_dir().unwrap();
    let cur = cur.to_str().unwrap();

    let home = env::home_dir().unwrap();
    let home = home.to_str().unwrap().to_owned();
    let save_location = home +"/sgd/src/.saved/" + &save_name;
    println!("{} saved to {}", save_name, save_location);
    

    
}
