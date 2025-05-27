mod args;
mod output;

use crate::output::output_with_permissions;
use args::Args;
use clap::Parser;
use lx_lib::{DirectoryItem, find_directory_items};
use output::output;

fn main() {
    let args = Args::parse();

    let dir_items: Vec<DirectoryItem> = find_directory_items(&args.directory);

    let output = match args {
        args if args.all && args.permissions => output_with_permissions(&dir_items, show_all),
        args if args.all => output(&dir_items, show_all),
        args if args.permissions => output_with_permissions(&dir_items, hide_hidden),
        _ => output(&dir_items, hide_hidden),
    };

    println!("{}", output);
}

/// Include all items in output including hidden files
fn show_all(_: &DirectoryItem) -> bool {
    true
}

/// Remove hidden files from the output
fn hide_hidden(item: &DirectoryItem) -> bool {
    !item.is_hidden
}
