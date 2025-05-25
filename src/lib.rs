use chrono::{DateTime, Local, NaiveDate};
use derive_builder::Builder;
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;
use std::{fs, path};

#[derive(Builder, Debug, PartialEq)]
pub struct DirectoryItem {
    /// Name of the item within the directory
    pub name: String,

    /// Type of the item within the directory
    pub is_dir: bool,

    /// Is item hidden within the directory
    pub is_hidden: bool,

    /// File permissions represented as a num
    pub file_permissions: String,

    /// Time when file was created
    pub created_at: NaiveDate,
}

/// Find items within a directory and return back a vector with the directory items.
pub fn find_directory_items(directory: &String) -> Vec<DirectoryItem> {
    let directory = path::Path::new(directory);
    let entries = fs::read_dir(directory).unwrap();
    entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_str()?.to_owned();
            let is_dir = entry.file_type().ok()?.is_dir();
            let is_hidden = is_file_hidden(entry.file_name());
            let file_permissions = entry.metadata().ok()?.permissions().mode();
            let created_at = system_time_to_local_date(entry.metadata().ok()?.created().ok()?);

            DirectoryItemBuilder::default()
                .name(name)
                .is_dir(is_dir)
                .is_hidden(is_hidden)
                .file_permissions(mode_to_rwx(file_permissions))
                .created_at(created_at)
                .build()
                .ok()
        })
        .collect()
}

/// Check if a file or directory is hidden
fn is_file_hidden(file_name: OsString) -> bool {
    file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Convert unix mode bits into rwx, split into 3 categories of user, group and others
fn mode_to_rwx(mode: u32) -> String {
    let mut perms = String::with_capacity(9);

    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'), // User
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'), // Group
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'), // Others
    ];

    for &(bit, ch) in &flags {
        perms.push(if mode & bit != 0 { ch } else { '-' });
    }

    perms
}

/// Convert system time into a local date
pub fn system_time_to_local_date(system_time: SystemTime) -> NaiveDate {
    let datetime: DateTime<Local> = system_time.into();
    datetime.date_naive()
}

#[cfg(test)]
mod tests {
    use crate::{DirectoryItem, find_directory_items, mode_to_rwx, system_time_to_local_date};
    use assert_fs::TempDir;
    use assert_fs::prelude::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::time::SystemTime;

    #[test]
    fn test_find_directory_items_with_static_dir() {
        let temp = TempDir::new().unwrap();

        temp.child("file.txt").touch().unwrap();
        temp.child("subdir").create_dir_all().unwrap();

        let file_metadata = fs::metadata(temp.child("file.txt").path()).unwrap();
        let dir_metadata = fs::metadata(temp.child("subdir").path()).unwrap();

        let file_permissions = file_metadata.permissions();
        let dir_permissions = dir_metadata.permissions();

        let file_mode = file_permissions.mode();
        let dir_mode = dir_permissions.mode();

        let path_as_str = &temp.path().to_str().unwrap().to_owned();
        let mut items = find_directory_items(path_as_str);

        items.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(items.len(), 2);

        let expected: Vec<DirectoryItem> = vec![
            DirectoryItem {
                name: "file.txt".to_string(),
                is_dir: false,
                is_hidden: false,
                file_permissions: mode_to_rwx(file_mode),
                created_at: system_time_to_local_date(SystemTime::now()),
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
                file_permissions: mode_to_rwx(dir_mode),
                created_at: system_time_to_local_date(SystemTime::now()),
            },
        ];

        assert_eq!(items, expected);
    }

    #[test]
    fn test_find_directory_items_with_hidden_file() {
        let temp = TempDir::new().unwrap();
        temp.child(".file.txt").touch().unwrap();

        let path_as_str = &temp.path().to_str().unwrap().to_owned();
        let mut items = find_directory_items(path_as_str);

        let file_metadata = fs::metadata(temp.child(".file.txt").path()).unwrap();

        let file_permissions = file_metadata.permissions();

        let file_mode = file_permissions.mode();

        items.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(items.len(), 1);

        let expected: Vec<DirectoryItem> = vec![DirectoryItem {
            name: ".file.txt".to_string(),
            is_dir: false,
            is_hidden: true,
            file_permissions: mode_to_rwx(file_mode),
            created_at: system_time_to_local_date(SystemTime::now()),
        }];

        assert_eq!(items, expected);
    }
}
