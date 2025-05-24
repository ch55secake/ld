mod args;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    if !args.directory.eq(".") {
        println!("Searching through: {}", args.directory);
    }
}
