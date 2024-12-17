use std::fs;

use project_root::get_project_root;
use quote::quote;

use codegen::Entry;

fn main() {
    let data = fs::read_to_string("./data.json").unwrap();
    let data = serde_json::from_str::<Vec<Entry>>(&data).unwrap();

    let struct_names = data.iter().map(|e| e.struct_name()).collect::<Vec<_>>();
    let names = data.iter().map(|e| e.name.clone()).collect::<Vec<_>>();

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

    // let syntax_tree = syn::parse2(token_stream).unwrap();
    // let code = prettyplease::unparse(&syntax_tree);
    // let project_root = get_project_root().unwrap();
    // fs::write(project_root.join("src/features.rs"), code).unwrap();
}
