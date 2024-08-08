use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::process::exit;

fn main() -> io::Result<()> {
    let mut f = BufReader::new(File::open("test.txt")?);

    let arg: Vec<String> = env::args().collect();

    if arg.len() < 2 {
        println!("Please provide an argument");
        exit(0);
    }

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    match arg[1].as_ref() {
        "c" => {
            println!("{}",f.bytes().count());
        }
        "l" => {
            println!("{}",f.lines().count());
        }
        "w" => {
            println!("{:?}", buffer.split_whitespace().count());
        }
        "m" => {
            println!("{:?}", buffer.chars().count());
        }
        _ => {
            println!("Illegal argument");
        }
    }
    Ok(())
}
