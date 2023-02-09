use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static ARG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("%(?:(\\d+)\\$)?([A-Za-z%]|$)").unwrap());

#[derive(Clone, Debug, Default)]
pub struct Translator {
    translations: HashMap<String, String>,
}

impl Translator {
    pub fn from_translation_content(
        translation_file_content: &str,
    ) -> Result<Self, serde_json::Error> {
        let translations = serde_json::from_str(translation_file_content)?;
        Ok(Self { translations })
    }

    pub fn translate(&self, translate: &str, args: &[&str], fallback: Option<&str>) -> String {
        let translate = translate.to_owned();
        let translate = self
            .translations
            .get(&translate)
            .map(|s| s.as_str())
            .unwrap_or(&fallback.unwrap_or_else(|| translate.as_str()));

        let mut i = 0;
        ARG_REGEX
            .replace_all(translate, |cap: &Captures| {
                //let entire_match = cap.get(0).map(|m| m.as_str()).unwrap_or("");
                let group_1 = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let group_2 = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                if group_2 == "%" {
                    "%%" // Literal percent sign
                } else if group_2 != "s" {
                    "" // Should always be %s
                } else {
                    let index = if let Ok(custom_arg_index) = group_1.parse::<usize>() {
                        custom_arg_index - 1
                    } else {
                        let index = i;
                        i += 1;
                        index
                    };
                    args.get(index).unwrap_or(&"")
                }
            })
            .to_string()
    }
}
