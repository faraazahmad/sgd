//does things to files saved with s
extern crate clap;
extern crate file;

use clap::{Arg, App};
use std::env;

fn main(){
    let s = App::new("d")
        .version("v0.1")
        .author("Parker K")
        .about("Runs a command on a file saved with s")
        .arg(Arg::with_name("COMMAND")
            .help("Specifies the command to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("FILE")
             .help("Specifies the files to run the COMMAND on, as saved with 's'")
             .required(true)
             .index(2))
        .get_matches();
        
    let loc_name = s.value_of("FILE").unwrap().to_owned();
    let com_name = s.value_of("COMMAND").unwrap().to_owned();

    let home = env::home_dir().unwrap();
    let home = home.to_str().unwrap().to_owned();
    let go_location = home +"/sgd/src/.saved/" + &loc_name + ".sgd";
    
    let check = file::get_text(&go_location);
    match check {
        Err(_string) => show_error(&go_location),
        Ok(_string) => go_to(&go_location),
    
    };
}

fn show_error(gl: &str) -> String
{
    println!("Error: no file found at {}", gl);
    String::from("error")
}

fn go_to(gl: &str) -> String
{
    let dest = file::get_text(gl).expect("Failed to read file");
    dest
} 




