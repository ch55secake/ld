mod args;
mod output;

use args::Args;
use clap::Parser;
use ld::{DirectoryItem, find_directory_items, path_to_str};
use output::output;

fn main() {
    let args = Args::parse();

    let path: &std::path::Path = path_to_str(&args.directory);
    let dir_items: Vec<DirectoryItem> = find_directory_items(path);

    let colourised_output: String = output(&dir_items);
    println!("{}", colourised_output);
}
