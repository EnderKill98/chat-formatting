use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    formatting::{ChatColor, ChatFormat},
    translator::Translator,
};

pub trait TextFormatter {
    /// Convert self into a legacy formatted string (using formatting codes prefixed by a paragraph "§")
    fn to_legacy_string(&self, translator: &Translator) -> String;

    /// Similar to legacy string, but uses common ansi escape codes to render with colors in most terminals
    fn to_ansi_string(&self, translator: &Translator) -> String {
        crate::legacy_to_ansi(&self.to_legacy_string(translator))
    }

    /// Get string without any formatting
    fn to_plain_string(&self, translator: &Translator) -> String {
        crate::legacy_to_plain(&self.to_legacy_string(translator))
    }
}

fn is_false(b: &bool) -> bool {
    !b
}

fn default_separator() -> ChatComponent {
    ChatComponent {
        color: Some(ChatColor::Gray),
        content: TextContent::new_literal(", "),
        ..Default::default()
    }
}

fn is_default_separator(component: &Option<Box<ChatComponent>>) -> bool {
    match component {
        Some(component) => component.as_ref() == &default_separator(),
        None => true,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatComponent {
    #[serde(flatten)]
    pub content: TextContent,

    #[serde(default, skip_serializing_if = "is_false")]
    pub bold: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub underlined: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub strikethrough: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub obfuscated: bool,

    pub color: Option<ChatColor>,

    /// Inserts in chat promt, if shift clicked
    pub insertion: Option<String>,
    /// Not used here, but might be useful to have (custom font path)
    pub font: Option<String>,

    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<ChatComponent>,
}

impl TextFormatter for ChatComponent {
    fn to_legacy_string(&self, translator: &Translator) -> String {
        let mut output = String::new();
        let mut component_formatting = String::new();
        if let Some(color) = self.color {
            if let Some(color_code) = color.into_color_code() {
                component_formatting.push('§');
                component_formatting.push(color_code);
            }
        }
        if self.bold {
            component_formatting.push_str("§l");
        }
        if self.italic {
            component_formatting.push_str("§o");
        }
        if self.strikethrough {
            component_formatting.push_str("§m");
        }
        if self.underlined {
            component_formatting.push_str("§n");
        }
        if self.obfuscated {
            component_formatting.push_str("§k");
        }
        output.push_str(&component_formatting);
        output.push_str(&match &self.content {
            TextContent::Literal { text } => text.to_owned(),
            TextContent::Keybind { keybind } => format!("<keybind:{:?}>", keybind),
            TextContent::Nbt { .. } => "<nbt>".to_owned(),
            TextContent::ScoreboardValue { score } => format!("<sbvalue:{:?}>", score.name),
            TextContent::EntityNamesSelector { selector, .. } => {
                format!("<selector:{:?}>", selector)
            }
            TextContent::Translatable {
                translate,
                with,
                fallback,
            } => {
                let resolved_args = with
                    .as_ref()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|arg| {
                        // Seems mc expects the old formatting to be restored (not trusting args)
                        format!(
                            "{}{}",
                            arg.to_legacy_string(translator),
                            &component_formatting
                        )
                    })
                    .collect::<Vec<_>>();
                translator.translate(
                    &translate,
                    &resolved_args
                        .iter()
                        .map(|arg| arg.as_str())
                        .collect::<Vec<_>>(),
                    fallback.as_ref().map(|s| s.as_str()),
                )
            }
        });
        output.push_str("§r");

        for extra in &self.extra {
            output.push_str(&extra.to_legacy_string(translator));
        }
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextContent {
    Translatable {
        translate: String,
        with: Option<Vec<Chat>>,
        fallback: Option<String>,
    },
    Keybind {
        keybind: String,
    },
    Nbt {
        nbt: String,

        #[serde(default, skip_serializing_if = "is_false")]
        interpret: bool,
        #[serde(default, skip_serializing_if = "is_default_separator")]
        separator: Option<Box<ChatComponent>>,

        block: Option<String>,
        entity: Option<String>,
        storage: Option<String>,
    },
    EntityNamesSelector {
        selector: String,
        #[serde(default, skip_serializing_if = "is_default_separator")]
        separator: Option<Box<ChatComponent>>,
    },
    ScoreboardValue {
        score: Score,
    },
    Literal {
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Score {
    name: String,
    objective: String,
    value: Option<i32>,
}

impl Default for TextContent {
    fn default() -> Self {
        TextContent::Literal {
            text: String::new(),
        }
    }
}

impl TextContent {
    pub fn new_literal(literal: &str) -> Self {
        TextContent::Literal {
            text: literal.to_owned(),
        }
    }

    pub fn new_translatable(translate: &str, with: &[&str]) -> Self {
        if with.is_empty() {
            TextContent::Translatable {
                translate: translate.to_owned(),
                with: None,
                fallback: None,
            }
        } else {
            TextContent::Translatable {
                translate: translate.to_owned(),
                with: Some(Vec::from_iter(
                    with.iter().map(|arg| Chat::Legacy(arg.to_string())),
                )),
                fallback: None,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClickEvent {
    action: ClickAction,
    value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverEvent {
    action: HoverAction,
    #[serde(flatten, alias = "value")]
    contents: HoverContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContent {
    /// Depending on action either legacy text or SNBT
    Text(String),
    /// Might be something unimplemented here, or also a ChatComponent
    Json(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Chat {
    Legacy(String),
    Component(ChatComponent),
    Components(Vec<ChatComponent>),
}

impl TextFormatter for Chat {
    fn to_legacy_string(&self, translator: &Translator) -> String {
        match self {
            Chat::Legacy(text) => text.to_owned(),
            Chat::Component(component) => component.to_legacy_string(translator),
            Chat::Components(components) => {
                let mut builder = String::new();
                for component in components {
                    builder.push_str(&component.to_legacy_string(translator));
                }
                builder
            }
        }
    }
}

impl Chat {
    pub fn new(legacy_text: &str) -> Self {
        Self::from_legacy(legacy_text)
    }

    pub fn from_legacy(legacy_text: &str) -> Self {
        let mut components: Vec<ChatComponent> = vec![];

        let mut cur_text = String::new();
        let mut cur_color: Option<ChatColor> = None;
        let mut cur_formattings: HashSet<ChatFormat> = HashSet::default();

        fn to_component(
            text: &str,
            color: &Option<ChatColor>,
            formattings: &HashSet<ChatFormat>,
        ) -> ChatComponent {
            ChatComponent {
                content: TextContent::Literal {
                    text: text.to_owned(),
                },
                color: color.clone(),
                bold: formattings.contains(&ChatFormat::Bold),
                italic: formattings.contains(&ChatFormat::Italic),
                obfuscated: formattings.contains(&ChatFormat::Obfuscated),
                strikethrough: formattings.contains(&ChatFormat::Strikethrough),
                underlined: formattings.contains(&ChatFormat::Underlined),
                ..Default::default()
            }
        }

        let mut previous_was_paragraph = false;
        for char in legacy_text.chars() {
            if char == '§' {
                previous_was_paragraph = true;
                continue;
            }
            if previous_was_paragraph {
                previous_was_paragraph = false;

                if let Ok(format) = ChatFormat::from_format_code_char(char) {
                    if !cur_text.is_empty() {
                        components.push(to_component(&cur_text, &cur_color, &cur_formattings));
                        cur_text.clear();
                    }
                    cur_formattings.insert(format);
                }
                if let Ok(color) = ChatColor::from_color_code_char(char) {
                    if !cur_text.is_empty() {
                        components.push(to_component(&cur_text, &cur_color, &cur_formattings));
                        cur_text.clear();
                    }
                    cur_color = Some(color);
                    cur_formattings.clear();
                }
                continue;
            }

            cur_text.push(char);
        }

        if !cur_text.is_empty() {
            components.push(to_component(&cur_text, &cur_color, &cur_formattings));
        }

        let mut root_component = Default::default();
        for (i, component) in components.into_iter().enumerate() {
            if i == 0 {
                root_component = component;
            } else {
                root_component.extra.push(component);
            }
        }

        return Chat::Component(root_component);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parsing() {
        // Simple
        assert_eq!(
            ChatComponent {
                content: TextContent::new_literal("Test"),
                color: Some(ChatColor::Red),
                ..Default::default()
            },
            serde_json::from_value(serde_json::json!({
                "text": "Test",
                "color": "red"
            }))
            .unwrap()
        );
        // Complex
        // Note quite working yet. Need to improve nesting ChatComponent
        // inside of HoverEvent. Raw Json Value seems to cause issues here.
        /*
        assert_eq!(
            ChatComponent {
                content: TextContent::new_literal("Hello "),
                color: Some(ChatColor::Red),
                italic: true,
                bold: true,
                extra: vec![
                    ChatComponent {
                        content: TextContent::new_translatable(
                            "test.translate.me",
                            &["Arg1", "Arg2"]
                        ),
                        hover_event: Some(HoverEvent {
                            action: HoverAction::ShowText,
                            contents: HoverContent::Json(
                                serde_json::to_value(ChatComponent {
                                    color: Some(ChatColor::DarkGreen),
                                    content: TextContent::new_literal("beautiful"),
                                    ..Default::default()
                                })
                                .unwrap()
                            )
                        }),
                        ..Default::default()
                    },
                    ChatComponent {
                        content: TextContent::new_literal(" World",),
                        hover_event: Some(HoverEvent {
                            action: HoverAction::ShowText,
                            contents: HoverContent::Text("§aAnother hover text!".to_owned())
                        }),
                        click_event: Some(ClickEvent {
                            action: ClickAction::CopyToClipboard,
                            value: "You clicked the world!".to_owned()
                        }),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            serde_json::from_value(serde_json::json!({
                "text": "Hello ",
                "color": "red",
                "italic": true,
                "bold": true,
                "extra": [
                    {
                        "translate": "test.translate.me",
                        "with": ["Arg1", "Arg2"],
                        "hoverEvent": {
                            "action": "show_text",
                            "contents": {
                                "text": "beautiful",
                                "color": "dark_green"
                            }
                        }
                    },
                    {
                        "text": " World",
                        "hoverEvent": {
                            "action": "show_text",
                            "value": "§aAnother hover text!"
                        },
                        "clickEvent": {
                            "action": "copy_to_clipboard",
                            "value": "You clicked the world!"
                        }
                    }
                ]
            }))
            .unwrap()
        );*/
    }
}
