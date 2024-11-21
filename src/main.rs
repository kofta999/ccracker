use ccracker::{run, Config};
use clap::Parser;

fn main() {
    let args = Config::parse();
    run(args);
}
