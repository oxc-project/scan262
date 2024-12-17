use convert_case::{Case, Casing};
use serde::Deserialize;
use syn::Ident;

#[derive(Debug, Default, Deserialize)]
pub struct Entry {
    pub name: String,
    pub category: Option<String>,
    pub spec: Option<String>,
    pub significance: Option<String>,
    pub mdn: Option<String>,
    pub exec: Option<String>,
    #[serde(default)]
    pub subtests: Vec<Entry>,
}

impl Entry {
    fn name(&self) -> String {
        self.name.chars().map(|c| if c.is_ascii_alphabetic() { c } else { '_' }).collect::<String>()
    }

    pub fn struct_name(&self) -> Ident {
        let name = self.name().to_case(Case::Pascal);
        quote::format_ident!("{name}")
    }

    pub fn file_name(&self) -> String {
        self.name().to_case(Case::Snake)
    }
}
