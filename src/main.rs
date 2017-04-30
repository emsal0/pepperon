use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;

fn main() {
    let mut x = env::args().nth(1);
    match x {
        Some(x) => {
            let mut file = File::open(&x);

            match file {
                Ok(mut f) => {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents);
                    println!("{}", contents)
                },

                Err(e) => {
                    println!("{}", e);
                },
            }
        },

        None =>
            println!("No argument at 1"),
    }
}
