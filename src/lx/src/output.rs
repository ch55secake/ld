use lx_lib::utils::byte_conv;
use lx_lib::{DirectoryItem, utils};

const STYLE_BOLD: &str = "\x1b[1m";
const COLOUR_PINK: &str = "\x1b[95m";
const COLOUR_CYAN: &str = "\x1b[36m";
const STYLE_RESET: &str = "\x1b[0m";
const COLOUR_RESET: &str = "\x1b[39m";

/// Output colourised output based on file type  
pub fn output<F>(items: &[DirectoryItem], filter: F) -> String
where
    F: Fn(&DirectoryItem) -> bool,
{
    let mut output = vec![];
    items.iter().filter(|item| filter(item)).for_each(|item| {
        if item.is_dir {
            create_dir_output(&mut output, &item);
        } else {
            output.push(format!("{} ", item.name.to_string()));
        }
    });

    let mut str_out = String::new();
    let terminal_width = utils::get_terminal_width().unwrap_or(80);
    let col_width = output
        .iter()
        .map(|s| clean_styling(s).len())
        .max()
        .unwrap_or(1);
    let cols = terminal_width / col_width;

    output.sort_by(|left, right| right.contains(STYLE_BOLD).cmp(&left.contains(STYLE_BOLD)));

    for (i, (styled_item, clean_item)) in output
        .iter()
        .map(|s| (s.clone(), clean_styling(s)))
        .enumerate()
    {
        let clean_len = clean_item.len();
        let padding_needed = if clean_len < col_width {
            col_width - clean_len
        } else {
            0
        };

        str_out.push_str(&styled_item);
        str_out.push_str(&" ".repeat(padding_needed));

        if (i + 1) % cols == 0 {
            str_out.push('\n');
        }
    }

    str_out
}

pub fn output_with_permissions<F>(items: &mut Vec<DirectoryItem>, filter: F) -> String
where
    F: Fn(&DirectoryItem) -> bool,
{
    let mut output = String::new();
    items.sort_by(|left, right| right.is_dir.cmp(&left.is_dir));
    items.iter().filter(|item| filter(item)).for_each(|item| {
        create_permissions_output(&mut output, &item);
    });
    output.trim_end().to_string()
}

/// Create default directory output
fn create_dir_output(output: &mut Vec<String>, item: &&DirectoryItem) {
    let dir_output: String = format!(
        "{}{}{}{}{}",
        STYLE_BOLD, COLOUR_PINK, &item.name, STYLE_RESET, COLOUR_RESET
    );
    output.push(format!("{} ", dir_output));
}

/// Create permissions and default dir output
fn create_permissions_output(output: &mut String, item: &&DirectoryItem) {
    if item.is_dir {
        let permissions_output: String = format!(
            "{}{}{:<10}{} {:<10} {:>8} {}{:<30}{}{}",
            STYLE_BOLD,
            COLOUR_CYAN,
            &item.created_at,
            COLOUR_RESET,
            &item.file_permissions,
            byte_conv(item.size),
            COLOUR_PINK,
            &item.name,
            STYLE_RESET,
            COLOUR_RESET
        );
        output.push_str(&permissions_output);
        output.push('\n');
    } else {
        let permissions_output: String = format!(
            "{}{}{:<10} {}{:<10} {}{:>8}{} {:<30}",
            STYLE_BOLD,
            COLOUR_CYAN,
            &item.created_at,
            COLOUR_RESET,
            &item.file_permissions,
            STYLE_BOLD,
            byte_conv(item.size),
            STYLE_RESET,
            &item.name
        );
        output.push_str(&permissions_output);
        output.push('\n');
    }
}

/// Clean the styling off a string
fn clean_styling(s: &String) -> String {
    s.replace(STYLE_BOLD, "")
        .replace(STYLE_RESET, "")
        .replace(COLOUR_PINK, "")
        .replace(COLOUR_RESET, "")
}

#[cfg(test)]
mod tests {
    use crate::output::output;
    use lx_lib::{DirectoryItem, system_time_to_local_date};
    use regex::Regex;
    use std::time::SystemTime;

    #[test]
    fn test_output() {
        let items = vec![
            DirectoryItem {
                name: "testfile.txt".to_string(),
                is_dir: false,
                is_hidden: false,
                file_permissions: String::from("rwxrwxrwx"),
                created_at: system_time_to_local_date(SystemTime::now()),
                size: 0,
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
                file_permissions: String::from("rwxrwxrwx"),
                created_at: system_time_to_local_date(SystemTime::now()),
                size: 0,
            },
        ];
        let expected_pattern = r"\S\s*\S";

        let re = Regex::new(expected_pattern).unwrap();

        assert!(re.is_match(&output(&items, |_| true)));
    }

    #[test]
    fn test_output_without_hidden_items() {
        let items = vec![DirectoryItem {
            name: ".DS_Store".to_string(),
            is_dir: false,
            is_hidden: true,
            file_permissions: String::from("rwxrwxrwx"),
            created_at: system_time_to_local_date(SystemTime::now()),
            size: 0,
        }];

        let expected = "";

        assert_eq!(output(&items, |item| !item.is_hidden), expected);
    }

    #[test]
    fn test_output_with_permissions() {
        let items = vec![
            DirectoryItem {
                name: ".DS_Store".to_string(),
                is_dir: false,
                is_hidden: true,
                file_permissions: String::from("rwxrwxrwx"),
                created_at: system_time_to_local_date(SystemTime::now()),
                size: 0,
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
                file_permissions: String::from("rwxrwxrwx"),
                created_at: system_time_to_local_date(SystemTime::now()),
                size: 0,
            },
        ];

        let expected_pattern = r"\S\s*\S";

        let re = Regex::new(expected_pattern).unwrap();

        assert!(re.is_match(&output(&items, |_| true)));
    }
}
