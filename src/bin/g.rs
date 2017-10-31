//goes to locations
extern crate clap;
extern crate file;

use clap::{Arg, App};
use std::env;
use std::path::Path;

fn main(){
    let s = App::new("g")
        .version("v0.1")
        .author("Parker K")
        .about("Goes to a specified location")
        .arg(Arg::with_name("NAME")
            .help("Specifies the location to go to, as saved by 's'")
            .required(true)
            .index(1)).
        get_matches();
    let loc_name = s.value_of("NAME").unwrap().to_owned();

    let home = env::home_dir().unwrap();
    let home = home.to_str().unwrap().to_owned();
    let go_location = home +"/sgd/src/.saved/" + &loc_name + ".sgd";
    
    let check = file::get_text(&go_location);
    match check {
        Err(_) => show_error(&go_location),
        Ok(_) => go_to(&go_location),
    
    };
}

fn show_error(gl: &str){
    println!("Error: no file found at {}", gl);
}

fn go_to(gl: &str){
    let dest = file::get_text(gl).expect("Failed to read file");
    println!("{}", dest);
    match env::set_current_dir(&dest){
        Ok(_) => success(),
        Err(_) => fail(),

    };
}
fn success() { }
fn fail() { println!("Error: Invalid path") }
