use chrono::{DateTime, Local, NaiveDate};
use std::ffi::OsString;
use std::time::SystemTime;
use terminal_size::{Height, Width, terminal_size};

/// Covert bytes into kb, mb, or gb
pub fn byte_conv(bytes: u64) -> String {
    if bytes as f64 > 1024.0 {
        return format!("{:.1}kb", (bytes as f64 / 1024.0).round() * 10.0).to_string();
    }
    format!("{:}b", bytes.to_string())
}

/// Get terminal width and height
pub fn get_terminal_width() -> Option<usize> {
    if let Some((Width(w), Height(_h))) = terminal_size() {
        Some(w as usize)
    } else {
        None
    }
}

/// Check if a file or directory is hidden
pub fn is_file_hidden(file_name: OsString) -> bool {
    file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Convert unix mode bits into rwx, split into 3 categories of user, group and others
pub fn mode_to_rwx(mode: u32) -> String {
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
