use std::collections::HashMap;

use inquire::{autocompletion::Replacement, Autocomplete};

use crate::utils;

const MDN_DATATYPE_BASE_URL: &str =
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/";

// All data types fall under the MDN_DATATYPE_BASE_URL except null
const NULL_URL: &str =
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/null";

#[derive(Clone)]
pub struct Map(pub HashMap<&'static str, &'static str>);

lazy_static! {
    pub static ref DATA_TYPES_MAP: Map = {
        Map(HashMap::from([
            ("array []", "Array"),
            ("object {}", "Object"),
            ("boolean =", "Boolean"),
            ("string 💬", "String"),
            ("number 🔢", "Number"),
            ("undefined ⁉️", "undefined"),
            ("null 🚫", "null"),
            ("map 🗺", "Map"),
            ("set 🧊", "Set"),
            ("date 📅", "Date"),
            ("promise 🤝", "Promise"),
            ("regexp 📕", "RegExp"),
        ]))
    };
}

impl Map {
    fn iter_matching_keys<F: FnMut(&str) -> ()>(&self, input: &str, mut cb: F) {
        let keys = self.0.keys();
        for key in keys {
            if key.starts_with(&input.to_lowercase()) {
                cb(key.clone());
            }
        }
    }
}

impl Autocomplete for Map {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let mut matching_keys: Vec<String> = vec![format!(
            "Search for {}",
            if input.len() >= 2 { input } else { "..." }
        )];

        self.iter_matching_keys(input, |key| {
            matching_keys.push(String::from(key));
        });

        return Ok(matching_keys);
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        let mut matching_keys: Vec<String> = vec![];

        self.iter_matching_keys(input, |key| {
            matching_keys.push(String::from(key));
        });

        Ok(match highlighted_suggestion {
            Some(highlighted) => Replacement::Some(highlighted),
            None => match matching_keys.len() {
                1 => Replacement::Some(matching_keys[0].clone()),
                _ => Replacement::None,
            },
        })
    }
}

pub fn mdn_data_types(selected_data_type: &str) -> Option<()> {
    let selected_data_type_result = DATA_TYPES_MAP.0.get(selected_data_type);

    let selected_data_type = match selected_data_type_result {
        Some(res) => res.to_owned(),
        None => {
            return None;
        }
    };

    let data_type_url = match selected_data_type {
        "null" => NULL_URL.to_owned(),
        _ => format!("{}{}", MDN_DATATYPE_BASE_URL, selected_data_type),
    };

    utils::open_and_output_url(&data_type_url[..]);
    Some(())
}
