pub mod cracker;
pub mod md5;
use std::{fs::File, io::Read};

use clap::Parser;
use cracker::{crack_bruteforce, crack_dict};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The hash you want to crack
    hash: String,

    /// Path to a dictionary file for a dictionary attack, if not supplied bruteforce will be used
    #[arg(short, long)]
    dict_file: Option<std::path::PathBuf>,
}

pub fn run(config: Config) {
    let password = match config.dict_file {
        Some(path) => {
            let mut file = File::open(path).unwrap();
            let mut dict = String::new();
            file.read_to_string(&mut dict).unwrap();

            crack_dict(dict, &config.hash)
        }

        None => crack_bruteforce(&config.hash),
    };

    match password {
        Some(p) => println!("{p}"),
        None => println!("Password not found"),
    }
}
