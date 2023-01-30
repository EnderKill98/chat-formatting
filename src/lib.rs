pub mod chat;
pub mod error;
pub mod formatting;
pub mod translator;

pub(crate) fn legacy_to_ansi(input: &str) -> String {
    let mut output = String::new();
    let mut was_paragraph = false;
    for chr in input.chars() {
        if was_paragraph {
            if let Ok(chat_color) = formatting::ChatColor::from_color_code_char(chr) {
                output.push_str(&chat_color.into_ansi_escape_code(true));
            } else if let Ok(chat_format) = formatting::ChatFormat::from_format_code_char(chr) {
                output.push_str(&chat_format.into_ansi_escape_code());
            } else {
                //output.push('§'); // Actually show paragraph? I don't believe so.
                output.push(chr);
            }
            was_paragraph = false;
        } else {
            if chr == '§' {
                was_paragraph = true;
            } else {
                output.push(chr);
            }
        }
    }
    output
}

pub(crate) fn legacy_to_plain(input: &str) -> String {
    let mut output = String::new();
    let mut was_paragraph = false;
    for chr in input.chars() {
        if was_paragraph {
            was_paragraph = false;
        } else {
            if chr == '§' {
                was_paragraph = true;
            } else {
                output.push(chr);
            }
        }
    }
    output
}
