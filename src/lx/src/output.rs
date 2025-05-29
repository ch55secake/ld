use lx_lib::utils::byte_conv;
use lx_lib::{DirectoryItem, utils};
use regex::Regex;

const STYLE_BOLD: &str = "\x1b[1m";
const COLOUR_PINK: &str = "\x1b[95m";
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
    // let terminal_width = 142;
    let max_entry_len = output.iter().map(|s| s.len()).max().unwrap_or(0);
    let col_width = max_entry_len + 2;
    let cols = terminal_width / col_width;

    // println!("{}", output.join(""));
    //
    // println!(
    //     "terminal width: {}\nmax_entry_len: {}\ncol_width: {}\ncols {}",
    //     terminal_width, max_entry_len, col_width, cols
    // );

    output.sort_by(|right, left| left.contains(STYLE_BOLD).cmp(&right.contains(STYLE_BOLD)));

    for (i, item) in output.iter().enumerate() {
        str_out.push_str(
            format!("{:<width$}", item, width = col_width)
                .to_string()
                .as_str(),
        );
        if (i + 1) % cols == 0 {
            // println!("{}", (i + 1) % cols == 0);
            str_out.push('\n');
        }
    }
    if output.len() % cols != 0 {
        // println!("{}", output.len() % cols != 0);
        str_out.push('\n');
    }
    let pattern = format!(r"\s{{{},}}", max_entry_len - cols - 2);
    let re = Regex::new(&pattern).unwrap();

    // println!("max_entry_len - cols: {}", max_entry_len - cols);
    // println!("matching: {}", re.is_match(&str_out));
    str_out = re
        .replace_all(&str_out, format!("{:width$}", "", width = cols))
        .to_string();
    str_out.trim_end().to_string()
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
            "{:<10} {:<10} {:>8} {}{}{:<30}{}{}",
            &item.created_at,
            &item.file_permissions,
            byte_conv(item.size),
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
            "{:<10} {:<10} {:>8} {:<30}",
            &item.created_at,
            &item.file_permissions,
            byte_conv(item.size),
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
