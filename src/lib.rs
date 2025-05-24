use derive_builder::Builder;
use std::fs;

#[derive(Builder, Debug)]
pub(crate) struct DirectoryItem {
    /// Name of the item within the directory
    name: String,

    // Type of the item within the directory
    is_dir: bool,
}

/// Find items within a directory and return back a vector with the directory items.
pub(crate) fn read_find_directory_items(directory: &std::path::Path) -> Vec<DirectoryItem> {
    let entries = fs::read_dir(directory).unwrap();

    let mut items: Vec<DirectoryItem> = vec![];

    entries.flatten().for_each(|entry| {
        let dir_item = DirectoryItemBuilder::default()
            .name(entry.file_name().to_str().unwrap().to_owned())
            .is_dir(entry.file_type().unwrap().to_owned().is_dir())
            .build();

        items.push(dir_item.unwrap())
    });

    items
}
