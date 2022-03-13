use colored::*;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::process;

fn main() {
    let file_path = get_file_path_from_argv();
    read_file(file_path);
}

fn get_file_path_from_argv() -> String {
    match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("{}", "missing argument file".red());
            println!("{}", "usage: rat <file>".green());
            process::exit(0);
        }
    }
}

fn read_file(file_path: String) {
    match fs::read_to_string(&file_path) {
        Ok(content) => println!("{}", content),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                println!(
                    "{} {} {}",
                    "Cannot find path".red(),
                    file_path.yellow(),
                    "because it does not exits".red()
                );
            } else {
                println!("{} {}", "Failed to read path".red(), file_path.yellow())
            }
        }
    }
}
