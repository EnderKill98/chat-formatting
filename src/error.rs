use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ChatColorParseError {
    #[error("Invalid hex format (expected format like #RRGGBB in hex, found {found:?})")]
    InvalidHexFormat { found: String },
    #[error("Couldn't parse int (a component was not valid hex)")]
    HexUnparsableInt(#[from] std::num::ParseIntError),
    #[error("Expected one of §<code>, #RRGGBB or color_name")]
    UnknownChatColorFormat,
    #[error("{color_code_char:?} is not a valid color code")]
    InvalidColorCodeChar { color_code_char: char },
    #[error("Invalid color code format (expected format like §X of lenght 2, found {found:?} of lenght {length})")]
    InvalidColorCodeFormat { found: String, length: usize },
    #[error("{color_name:?} is not a valid color name")]
    InvalidColorName { color_name: String },
}

#[derive(Error, Debug, PartialEq)]
pub enum ChatFormatParseError {
    #[error("{format_code_char:?} is not a valid format code")]
    InvalidFormatCodeChar { format_code_char: char },
    #[error("Invalid format code format (expected format like §X of lenght 2, found {found:?} of lenght {length})")]
    InvalidFormatCodeFormat { found: String, length: usize },
}
