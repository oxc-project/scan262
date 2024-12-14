use serde::Deserialize;

use convert_case::{Case, Casing};

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub name: String,
}

impl Entry {
    pub fn struct_name(&self) -> String {
        self.name
            .chars()
            .map(|c| if c.is_ascii_alphabetic() { c } else { '_' })
            .collect::<String>()
            .to_case(Case::Pascal)
    }
}
