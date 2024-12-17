use convert_case::{Case, Casing};
use serde::Deserialize;
use syn::Ident;

#[derive(Debug, Default, Deserialize)]
pub struct Entry {
    pub name: String,

    #[serde(default)]
    pub target: String,

    #[serde(default)]
    pub category: String,

    #[serde(default)]
    pub spec: String,

    #[serde(default)]
    pub significance: String,

    #[serde(default)]
    pub mdn: String,

    #[serde(default)]
    pub exec: String,

    #[serde(default)]
    pub subtests: Vec<Entry>,
}

impl Entry {
    fn prefix(&self) -> String {
        if self.category.starts_with("20") {
            format!("es{}", self.category.chars().take(4).collect::<String>())
        } else {
            self.target.clone()
        }
    }

    fn name(&self) -> String {
        self.name.chars().map(|c| if c.is_ascii_alphabetic() { c } else { '_' }).collect::<String>()
    }

    pub fn struct_name(&self) -> Ident {
        let prefix = self.prefix().to_case(Case::Pascal);
        let name = self.name().to_case(Case::Pascal);
        let name = format!("{}{}", prefix, name);
        quote::format_ident!("{name}")
    }

    pub fn file_name(&self) -> String {
        let prefix = self.prefix();
        let name = self.name().to_case(Case::Snake);
        let mut name = format!("{prefix}_{name}");
        if matches!(name.as_str(), "const" | "let" | "super") {
            name = format!("feature_{name}");
        }
        name
    }
}
