mod args;
mod output;

use args::Args;
use clap::Parser;
use ld::{DirectoryItem, find_directory_items};
use output::output;

fn main() {
    let args = Args::parse();

    let dir_items: Vec<DirectoryItem> = find_directory_items(&args.directory);

    let filter = match args {
        args if args.all => show_all,
        _ => hide_hidden,
    };

    let colourised_output: String = output(&dir_items, filter);

    println!("{}", colourised_output);
}

/// Include all items in output including hidden files
fn show_all(_: &DirectoryItem) -> bool {
    true
}

/// Remove hidden files from the output
fn hide_hidden(item: &DirectoryItem) -> bool {
    !item.is_hidden
}
