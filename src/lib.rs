use derive_builder::Builder;
use std::fs;

#[derive(Builder, Debug)]
pub struct DirectoryItem {
    /// Name of the item within the directory
    pub name: String,

    // Type of the item within the directory
    pub is_dir: bool,
}

/// Find items within a directory and return back a vector with the directory items.
pub fn find_directory_items(directory: &std::path::Path) -> Vec<DirectoryItem> {
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

/// Convert a string to a path object
pub fn path_to_str(path: &String) -> &std::path::Path {
    std::path::Path::new(path)
}
