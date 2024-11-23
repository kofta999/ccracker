pub mod cracker;
pub mod md5;
pub mod rainbow;
use clap::{Args, Parser, Subcommand};
use cracker::{crack_bruteforce, crack_dict, crack_rainbow};
use rainbow::{create_dict_table, create_n_len_table};
use std::{fs::File, io::Read};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Crack(CrackArgs),
    Rainbow(RainbowArgs),
}

#[derive(Args)]
struct CrackArgs {
    /// The hash you want to crack
    hash: String,

    /// Path to a dictionary file for a dictionary attack, if not supplied bruteforce will be used
    #[arg(short, long)]
    dict_file: Option<std::path::PathBuf>,

    #[arg(short)]
    rainbow_table: Option<std::path::PathBuf>,
}

#[derive(Args)]
struct RainbowArgs {
    /// Length of the rainbow table password
    #[arg(short)]
    length: Option<u8>,

    /// Path to a dictionary file to generate a rainbow table from it
    #[arg(short, long)]
    dict_file: Option<std::path::PathBuf>,
}

pub fn run(config: Config) {
    match config.command {
        Commands::Crack(subcommand) => {
            if let Some(password) = match subcommand.dict_file {
                Some(path) => {
                    let mut file = File::open(path).unwrap();
                    let mut dict = String::new();
                    file.read_to_string(&mut dict).unwrap();
                    crack_dict(dict, &subcommand.hash)
                }
                None => match subcommand.rainbow_table {
                    Some(path) => {
                        let mut file = File::open(path).unwrap();
                        let mut dict = String::new();
                        file.read_to_string(&mut dict).unwrap();
                        crack_rainbow(dict, &subcommand.hash)
                    }
                    None => crack_bruteforce(&subcommand.hash),
                },
            } {
                println!("{}", password)
            }
        }
        Commands::Rainbow(subcommand) => match subcommand.dict_file {
            Some(path) => {
                let mut file = File::open(path).unwrap();
                let mut dict = String::new();
                file.read_to_string(&mut dict).unwrap();
                create_dict_table(dict);
            }
            None => {
                if let Some(n) = subcommand.length {
                    create_n_len_table(n);
                }
            }
        },
    }
}
