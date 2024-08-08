use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = BufReader::new(File::open("test.txt")?);

    let arg: Vec<String> = env::args().collect();

    match arg[1].as_ref() {
        "c" => {
            println!("{}",f.bytes().count());
        }
        _ => {
            println!("Illegal argument");
        }
    }
    Ok(())
}
