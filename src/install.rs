use std::process::Command;
use std::fs;
use std::env;
fn main(){
    println!("Building...");
    let os_str = Command::new("uname").arg("-s").output().expect("Failed to find OS");
    let os_str = String::from_utf8_lossy(&os_str.stdout).into_owned();
    match env::home_dir(){
        Some(home) => fs::create_dir(home.display().to_string().to_owned() +"/sgd/src/.saved").expect("Directory already exists or you lack permissions to create it"),
        None => println!("Failed to find location of your home directory"),
        //Some(home) => println!("{}", home.display().to_string().to_owned() +"/sgd/src/.saved"),
    };
    match &os_str[..]{
        "Darwin\n" => darwin(),
	"Linux\n" => linux(),
        "SunOS\n" => solaris(),
	_ => println!(""),
    };
    
}

fn darwin(){
    Command::new("mv")
        .arg("./target/debug/s")
        .arg("/usr/local/bin/s")
        .output()
        .expect("Failed to install.");
    Command::new("mv")
        .arg("./target/debug/g")
        .arg("/usr/local/bin/g")
        .output()
        .expect("Failed to install.");
}

fn linux(){
    Command::new("./.linux").output().expect("Failed to run .linux");
}

fn solaris(){
    println!("OS not yet supported");
}
