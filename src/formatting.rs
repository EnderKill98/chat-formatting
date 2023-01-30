use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::error::{ChatColorParseError, ChatFormatParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr)]
pub enum ChatColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Reset,
    /// TODO: Impl serde properly for it
    Hex([u8; 3]),
}

impl ChatColor {
    pub fn from_hex_str(hex_str: &str) -> Result<Self, ChatColorParseError> {
        if !hex_str.starts_with('#') || hex_str.len() != 7 {
            return Err(ChatColorParseError::InvalidHexFormat {
                found: hex_str.to_owned(),
            });
        };
        let r = u8::from_str_radix(&hex_str[1..3], 16)
            .map_err(|err| ChatColorParseError::HexUnparsableInt(err))?;
        let g = u8::from_str_radix(&hex_str[3..5], 16)
            .map_err(|err| ChatColorParseError::HexUnparsableInt(err))?;
        let b = u8::from_str_radix(&hex_str[5..7], 16)
            .map_err(|err| ChatColorParseError::HexUnparsableInt(err))?;
        Ok(ChatColor::Hex([r, g, b]))
    }

    pub fn from_color_code_char(color_code_char: char) -> Result<Self, ChatColorParseError> {
        Ok(match color_code_char {
            '0' => ChatColor::Black,
            '1' => ChatColor::DarkBlue,
            '2' => ChatColor::DarkGreen,
            '3' => ChatColor::DarkAqua,
            '4' => ChatColor::DarkRed,
            '5' => ChatColor::DarkPurple,
            '6' => ChatColor::Gold,
            '7' => ChatColor::Gray,
            '8' => ChatColor::DarkGray,
            '9' => ChatColor::Blue,
            'a' => ChatColor::Green,
            'b' => ChatColor::Aqua,
            'c' => ChatColor::Red,
            'd' => ChatColor::LightPurple,
            'e' => ChatColor::Yellow,
            'f' => ChatColor::White,
            'r' => ChatColor::Reset,
            _ => return Err(ChatColorParseError::InvalidColorCodeChar { color_code_char }),
        })
    }

    pub fn from_color_code(color_code: &str) -> Result<Self, ChatColorParseError> {
        // len() would be 3 as § is 2 bytes in UTF-8!
        if color_code.chars().count() == 2 {
            Self::from_color_code_char(color_code.chars().nth(1).unwrap())
        } else {
            Err(ChatColorParseError::InvalidColorCodeFormat {
                found: color_code.to_owned(),
                length: color_code.len(),
            })
        }
    }

    pub fn from_color_name(color_name: &str) -> Result<Self, ChatColorParseError> {
        Ok(match color_name {
            "black" => ChatColor::Black,
            "dark_blue" => ChatColor::DarkBlue,
            "dark_green" => ChatColor::DarkGreen,
            "dark_aqua" => ChatColor::DarkAqua,
            "dark_red" => ChatColor::DarkRed,
            "dark_purple" => ChatColor::DarkPurple,
            "gold" => ChatColor::Gold,
            "gray" => ChatColor::Gray,
            "dark_gray" => ChatColor::DarkGray,
            "blue" => ChatColor::Blue,
            "green" => ChatColor::Green,
            "aqua" => ChatColor::Aqua,
            "red" => ChatColor::Red,
            "light_purple" => ChatColor::LightPurple,
            "yellow" => ChatColor::Yellow,
            "white" => ChatColor::White,
            "reset" => ChatColor::Reset,
            _ => {
                return Err(ChatColorParseError::InvalidColorName {
                    color_name: color_name.to_owned(),
                })
            }
        })
    }

    pub fn into_color_code(self) -> Option<char> {
        Some(match self {
            ChatColor::Black => '0',
            ChatColor::DarkBlue => '1',
            ChatColor::DarkGreen => '2',
            ChatColor::DarkAqua => '3',
            ChatColor::DarkRed => '4',
            ChatColor::DarkPurple => '5',
            ChatColor::Gold => '6',
            ChatColor::Gray => '7',
            ChatColor::DarkGray => '8',
            ChatColor::Blue => '9',
            ChatColor::Green => 'a',
            ChatColor::Aqua => 'b',
            ChatColor::Red => 'c',
            ChatColor::LightPurple => 'd',
            ChatColor::Yellow => 'e',
            ChatColor::White => 'f',
            ChatColor::Reset => 'r',
            ChatColor::Hex(_) => return None,
        })
    }

    pub fn into_ansi_escape_code(self, reset_formatting: bool) -> String {
        let simple_color =
            |reset, color| format!("\x1B[{}{}m", if reset { "0;" } else { "" }, color);
        match self {
            ChatColor::Black => simple_color(reset_formatting, 30),
            ChatColor::DarkBlue => simple_color(reset_formatting, 34),
            ChatColor::DarkGreen => simple_color(reset_formatting, 32),
            ChatColor::DarkAqua => simple_color(reset_formatting, 36),
            ChatColor::DarkRed => simple_color(reset_formatting, 31),
            ChatColor::DarkPurple => simple_color(reset_formatting, 35),
            ChatColor::Gold => simple_color(reset_formatting, 33),
            ChatColor::Gray => simple_color(reset_formatting, 37),
            ChatColor::DarkGray => simple_color(reset_formatting, 90),
            ChatColor::Blue => simple_color(reset_formatting, 94),
            ChatColor::Green => simple_color(reset_formatting, 92),
            ChatColor::Aqua => simple_color(reset_formatting, 96),
            ChatColor::Red => simple_color(reset_formatting, 91),
            ChatColor::LightPurple => simple_color(reset_formatting, 95),
            ChatColor::Yellow => simple_color(reset_formatting, 93),
            ChatColor::White => simple_color(reset_formatting, 97),
            ChatColor::Reset => format!("\x1B[0m"),
            ChatColor::Hex(rgb) => {
                return format!(
                    "\x1B[{}38;2;{};{};{}m",
                    if reset_formatting { "0;" } else { "" },
                    rgb[0],
                    rgb[1],
                    rgb[2]
                )
            }
        }
    }
}

impl std::fmt::Display for ChatColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatColor::Black => write!(f, "black"),
            ChatColor::DarkBlue => write!(f, "dark_blue"),
            ChatColor::DarkGreen => write!(f, "dark_green"),
            ChatColor::DarkAqua => write!(f, "dark_aqua"),
            ChatColor::DarkRed => write!(f, "dark_red"),
            ChatColor::DarkPurple => write!(f, "dark_purple"),
            ChatColor::Gold => write!(f, "gold"),
            ChatColor::Gray => write!(f, "gray"),
            ChatColor::DarkGray => write!(f, "dark_gray"),
            ChatColor::Blue => write!(f, "blue"),
            ChatColor::Green => write!(f, "green"),
            ChatColor::Aqua => write!(f, "aqua"),
            ChatColor::Red => write!(f, "red"),
            ChatColor::LightPurple => write!(f, "light_purple"),
            ChatColor::Yellow => write!(f, "yellow"),
            ChatColor::White => write!(f, "white"),
            ChatColor::Reset => write!(f, "reset"),
            ChatColor::Hex(rgb) => write!(f, "#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]),
        }
    }
}

impl std::str::FromStr for ChatColor {
    type Err = ChatColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") {
            Self::from_hex_str(s)
        } else if s.starts_with("§") {
            Self::from_color_code(s)
        } else {
            Self::from_color_name(s).map_err(|_| ChatColorParseError::UnknownChatColorFormat)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChatFormat {
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
    // (Reset is recognized as color value)
}

impl ChatFormat {
    pub fn from_format_code_char(format_code_char: char) -> Result<Self, ChatFormatParseError> {
        Ok(match format_code_char {
            'l' => ChatFormat::Bold,
            'm' => ChatFormat::Strikethrough,
            'n' => ChatFormat::Underlined,
            'o' => ChatFormat::Italic,
            'k' => ChatFormat::Obfuscated,
            _ => return Err(ChatFormatParseError::InvalidFormatCodeChar { format_code_char }),
        })
    }

    pub fn from_format_code(format_code: &str) -> Result<Self, ChatFormatParseError> {
        // len() would be 3 as § is 2 bytes in UTF-8!
        if format_code.chars().count() == 2 {
            Self::from_format_code_char(format_code.chars().nth(1).unwrap())
        } else {
            Err(ChatFormatParseError::InvalidFormatCodeFormat {
                found: format_code.to_owned(),
                length: format_code.len(),
            })
        }
    }

    pub fn into_format_code(self) -> char {
        match self {
            ChatFormat::Bold => 'l',
            ChatFormat::Italic => 'o',
            ChatFormat::Underlined => 'n',
            ChatFormat::Strikethrough => 'm',
            ChatFormat::Obfuscated => 'k',
        }
    }

    pub fn into_ansi_display_attribute(self) -> u8 {
        match self {
            ChatFormat::Bold => 1,
            ChatFormat::Italic => 3,
            ChatFormat::Underlined => 4,
            ChatFormat::Strikethrough => 9,
            ChatFormat::Obfuscated => 8,
        }
    }

    pub fn into_ansi_escape_code(self) -> String {
        format!("\x1B[{}m", self.into_ansi_display_attribute())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        assert_eq!(ChatColor::DarkGray.to_string(), "dark_gray");
        assert_eq!(ChatColor::Hex([0xF0, 0x04, 0x20]).to_string(), "#F00420");
        assert_eq!(
            ChatColor::from_hex_str("#F00420"),
            Ok(ChatColor::Hex([0xF0, 0x04, 0x20]))
        );
        assert_eq!("§1".parse(), Ok(ChatColor::DarkBlue));
        assert_eq!("dark_blue".parse(), Ok(ChatColor::DarkBlue));
        assert_eq!("#F00420".parse(), Ok(ChatColor::Hex([0xF0, 0x04, 0x20])));
        assert_eq!(
            "foobar".parse::<ChatColor>(),
            Err(ChatColorParseError::UnknownChatColorFormat)
        );
        "#-azxxxx".parse::<ChatColor>().unwrap_err();
    }
    #[test]

    fn test_formats() {
        assert_eq!(
            ChatFormat::from_format_code_char('k'),
            Ok(ChatFormat::Obfuscated)
        );
        assert_eq!(
            ChatFormat::from_format_code("§m"),
            Ok(ChatFormat::Strikethrough)
        )
    }
}
