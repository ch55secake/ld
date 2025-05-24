use ld::DirectoryItem;

const STYLE_BOLD: &str = "\x1b[1m";
const COLOUR_PINK: &str = "\x1b[95m";
const STYLE_RESET: &str = "\x1b[0m";
const COLOUR_RESET: &str = "\x1b[39m";

/// Output colourised output based on file type  
pub fn output(items: &Vec<DirectoryItem>) -> String {
    let mut output = String::new();
    items.into_iter().for_each(|item| {
        if item.is_dir {
            let dir_output: String = format!(
                "{}{}{}{}{}",
                STYLE_BOLD, COLOUR_PINK, &item.name, STYLE_RESET, COLOUR_RESET
            );
            output.push_str(&dir_output);
            output.push_str(" ");
        } else {
            output.push_str(&item.name);
            output.push_str(" ");
        }
    });
    output
}
