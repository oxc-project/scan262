use std::fs;

use project_root::get_project_root;
use quote::{format_ident, quote};

use codegen::Entry;

fn main() {
    let data = fs::read_to_string("./data.json").unwrap();
    let entries = serde_json::from_str::<Vec<Entry>>(&data).unwrap();

    // let struct_names = data.iter().map(|e| e.struct_name()).collect::<Vec<_>>();
    // let names = data.iter().map(|e| e.name.clone()).collect::<Vec<_>>();
    for entry in &entries {
        generate_implementation(&entry)
    }
    generate_module(&entries);
}

fn generate_implementation(entry: &Entry) {
    // let token_stream = quote! {
    // use crate::feature::Feature;

    // pub const RULES: &[&dyn Feature] = &[
    // #(&#struct_names,)*
    // ];

    // #(pub struct #struct_names;)*

    // #(impl Feature for #struct_names {
    // fn name(&self) -> &'static str {
    // #names
    // }

    // fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
    // None
    // }
    // })*

    // };

    let token_stream = quote! {};
    let filename = format!("implementation/{}.rs", entry.file_name());
    generate_file(&filename, token_stream);
}

fn generate_module(entries: &Vec<Entry>) {
    let mods = entries.iter().map(|e| format_ident!("{}", e.file_name().to_string()));
    let token_stream = quote! {
        #(mod #mods;)*
    };
    generate_file("implementation.rs", token_stream);
}

fn generate_file(file: &str, token_stream: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    let project_root = get_project_root().unwrap();
    fs::write(project_root.join("src").join(file), code).unwrap();
}
