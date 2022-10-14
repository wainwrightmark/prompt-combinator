use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::collections::BTreeMap;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct SavedPromptsState {
    pub prompts: BTreeMap<String, String>,
}

pub const EXAMPLES: [(&str, &str); 3] = [
    ("Disjunction Example", "a {black|brown} {cat|dog}"),
    ("Range Example", "a (red:{0.0;1.0;0.1}) cat"),
    (
        "Variables Example",
        "{<animal>:cat|dog}! a {<animal>} with another {<animal>}",
    ),
];

impl Default for SavedPromptsState {
    fn default() -> Self {
        let mut prompts = BTreeMap::new();

        for (name, text) in EXAMPLES {
            prompts.insert(name.to_string(), text.to_string());
        }

        Self { prompts }
    }
}

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct InputState {
    pub name: String,
    pub text: String,
    pub output: Rc<Vec<String>>,
    pub error: Option<String>,
}

impl Default for InputState {
    fn default() -> Self {
        let example = EXAMPLES[0];

        let mut result = Self {
            name: example.0.to_string(),
            text: Default::default(),
            error: None,
            output: Default::default(),
        };

        result.update_text(example.1.to_string());

        result
    }
}

impl InputState {
    pub fn save(&self) {
        Dispatch::<SavedPromptsState>::new()
            .reduce_mut(|s| s.prompts.insert(self.name.clone(), self.text.clone()));
    }

    pub fn load_saved(&mut self, name: String) {
        let saved = Dispatch::<SavedPromptsState>::new().get();
        let s = saved.prompts.get(&name);

        if let Some(text) = s {
            self.name = name;
            self.update_text(text.clone());
        }
    }

    pub fn update_text(&mut self, new_text: String) {
        if self.text != new_text {
            self.text = new_text.clone();
            //Do not change output - it's annoying
            //self.output = Default::default();
            let statement_result = parse_prompt(new_text.as_str());

            match statement_result {
                Ok(statement) => match statement.fully_expand() {
                    Ok(output) => {
                        self.output = output.into();
                        self.error = None;
                    }
                    Err(error) => self.error = Some(error.to_string()),
                },
                Err(error) => self.error = Some(error.to_string()),
            }
        }
    }
}
