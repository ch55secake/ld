use lx_lib::DirectoryItem;

const STYLE_BOLD: &str = "\x1b[1m";
const COLOUR_PINK: &str = "\x1b[95m";
const STYLE_RESET: &str = "\x1b[0m";
const COLOUR_RESET: &str = "\x1b[39m";

/// Output colourised output based on file type  
pub fn output<F>(items: &[DirectoryItem], filter: F) -> String
where
    F: Fn(&DirectoryItem) -> bool,
{
    let mut output = String::new();
    items.iter().filter(|item| filter(item)).for_each(|item| {
        if item.is_dir {
            create_dir_output(&mut output, &item);
        } else {
            output.push_str(&item.name);
            output.push(' ');
        }
    });
    output
}

pub fn output_with_permissions<F>(items: &[DirectoryItem], filter: F) -> String
where
    F: Fn(&DirectoryItem) -> bool,
{
    let mut output = String::new();
    items.iter().filter(|item| filter(item)).for_each(|item| {
        create_permissions_output(&mut output, &item);
    });
    output.trim_end().to_string()
}

/// Create default directory output
fn create_dir_output(output: &mut String, item: &&DirectoryItem) {
    let dir_output: String = format!(
        "{}{}{}{}{}",
        STYLE_BOLD, COLOUR_PINK, &item.name, STYLE_RESET, COLOUR_RESET
    );
    output.push_str(&dir_output);
    output.push(' ');
}

/// Create permissions and default dir output
fn create_permissions_output(output: &mut String, item: &&DirectoryItem) {
    if item.is_dir {
        let permissions_output: String = format!(
            "{} {} {}{}{}{}{}",
            &item.created_at,
            &item.file_permissions,
            STYLE_BOLD,
            COLOUR_PINK,
            &item.name,
            STYLE_RESET,
            COLOUR_RESET
        );
        output.push_str(&permissions_output);
        output.push('\n');
    } else {
        let permissions_output: String = format!(
            "{} {} {}",
            &item.created_at, &item.file_permissions, &item.name
        );
        output.push_str(&permissions_output);
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use crate::output::{
        COLOUR_PINK, COLOUR_RESET, STYLE_BOLD, STYLE_RESET, output, output_with_permissions,
    };
    use lx_lib::{DirectoryItem, system_time_to_local_date};
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
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
                file_permissions: String::from("rwxrwxrwx"),
                created_at: system_time_to_local_date(SystemTime::now()),
            },
        ];

        let expected = format!(
            "testfile.txt {}{}subdir{}{} ",
            STYLE_BOLD, COLOUR_PINK, STYLE_RESET, COLOUR_RESET
        );

        assert_eq!(output(&items, |_| true), expected);
    }

    #[test]
    fn test_output_without_hidden_items() {
        let items = vec![DirectoryItem {
            name: ".DS_Store".to_string(),
            is_dir: false,
            is_hidden: true,
            file_permissions: String::from("rwxrwxrwx"),
            created_at: system_time_to_local_date(SystemTime::now()),
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
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
                file_permissions: String::from("rwxrwxrwx"),
                created_at: system_time_to_local_date(SystemTime::now()),
            },
        ];

        let expected = format!(
            "{} rwxrwxrwx .DS_Store\n{} rwxrwxrwx {}{}subdir{}{}",
            system_time_to_local_date(SystemTime::now()),
            system_time_to_local_date(SystemTime::now()),
            STYLE_BOLD,
            COLOUR_PINK,
            STYLE_RESET,
            COLOUR_RESET
        );

        assert_eq!(output_with_permissions(&items, |_| true), expected);
    }
}
