mod args;

use clap::Parser;
use args::Args;

fn main() {
   let args = Args::parse(); 
    
    if !args.directory.eq(".") { 
        println!("Searching through: {}", args.directory);
    }
}
