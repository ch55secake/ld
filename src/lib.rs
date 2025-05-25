use derive_builder::Builder;
use std::ffi::OsString;
use std::fs;

#[derive(Builder, Debug, PartialEq)]
pub struct DirectoryItem {
    /// Name of the item within the directory
    pub name: String,

    /// Type of the item within the directory
    pub is_dir: bool,

    /// Is item hidden within the directory
    pub is_hidden: bool,
}

/// Find items within a directory and return back a vector with the directory items.
pub fn find_directory_items(directory: &std::path::Path) -> Vec<DirectoryItem> {
    let entries = fs::read_dir(directory).unwrap();

    let mut items: Vec<DirectoryItem> = vec![];

    entries.flatten().for_each(|entry| {
        let dir_item = DirectoryItemBuilder::default()
            .name(entry.file_name().to_str().unwrap().to_owned())
            .is_dir(entry.file_type().unwrap().to_owned().is_dir())
            .is_hidden(is_file_hidden(entry.file_name()))
            .build();

        items.push(dir_item.unwrap())
    });

    items
}

/// Convert a string to a path object
pub fn path_to_str(path: &String) -> &std::path::Path {
    std::path::Path::new(path)
}

/// Check if a file or directory is hidden
fn is_file_hidden(file_name: OsString) -> bool {
    file_name.to_str().unwrap().starts_with(".")
}

#[cfg(test)]
mod tests {

    use crate::{DirectoryItem, find_directory_items};
    use assert_fs::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn test_find_directory_items_with_static_dir() {
        let temp = TempDir::new().unwrap();

        temp.child("file.txt").touch().unwrap();
        temp.child("subdir").create_dir_all().unwrap();

        let mut items = find_directory_items(temp.path());

        items.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(items.len(), 2);

        let expected: Vec<DirectoryItem> = vec![
            DirectoryItem {
                name: "file.txt".to_string(),
                is_dir: false,
                is_hidden: false,
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
            },
        ];

        assert_eq!(items, expected);
    }

    #[test]
    fn test_find_directory_items_with_hidden_file() {
        let temp = TempDir::new().unwrap();
        temp.child(".file.txt").touch().unwrap();

        let mut items = find_directory_items(temp.path());

        items.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(items.len(), 1);

        let expected: Vec<DirectoryItem> = vec![DirectoryItem {
            name: ".file.txt".to_string(),
            is_dir: false,
            is_hidden: true,
        }];

        assert_eq!(items, expected);
    }
}
