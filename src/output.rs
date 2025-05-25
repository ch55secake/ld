use ld::DirectoryItem;

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
            output.push_str(" ");
        }
    });
    output
}

/// Create default directory output
fn create_dir_output(output: &mut String, item: &&DirectoryItem) {
    let dir_output: String = format!(
        "{}{}{}{}{}",
        STYLE_BOLD, COLOUR_PINK, &item.name, STYLE_RESET, COLOUR_RESET
    );
    output.push_str(&dir_output);
    output.push_str(" ");
}

#[cfg(test)]
mod tests {
    use crate::output::output;
    use ld::DirectoryItem;

    #[test]
    fn test_output() {
        let items = vec![
            DirectoryItem {
                name: "testfile.txt".to_string(),
                is_dir: false,
                is_hidden: false,
            },
            DirectoryItem {
                name: "subdir".to_string(),
                is_dir: true,
                is_hidden: false,
            },
        ];

        let expected = "testfile.txt \u{1b}[1m\u{1b}[95msubdir\u{1b}[0m\u{1b}[39m ";

        assert_eq!(output(&items, |_| true), expected);
    }

    #[test]
    fn test_output_without_hidden_items() {
        let items = vec![DirectoryItem {
            name: ".DS_Store".to_string(),
            is_dir: false,
            is_hidden: true,
        }];

        let expected = "";

        assert_eq!(output(&items, |item| item.is_hidden), expected);
    }
}
