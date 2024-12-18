use std::fs;

use project_root::get_project_root;
use quote::{format_ident, quote};

use codegen::Entry;

fn main() {
    let data = fs::read_to_string("./data.json").unwrap();
    let entries = serde_json::from_str::<Vec<Entry>>(&data).unwrap();
    for entry in &entries {
        // generate_implementation(entry);
        generate_meta(entry);
    }
    generate_implementation_module(&entries);
    generate_meta_module(&entries);
    generate_features(&entries);
}

#[expect(dead_code)]
fn generate_implementation(entry: &Entry) {
    let struct_name = entry.struct_name();
    let token_stream = quote! {
        use oxc::semantic::{AstNode, Semantic};

        use crate::{feature::Feature, ctx::Ctx};
        use crate::features::#struct_name;

        impl Feature for #struct_name {
            fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {
            }
        }

    };
    let filename = format!("implementation/{}.rs", entry.file_name());
    generate_file(&filename, token_stream);
}

fn generate_meta(entry: &Entry) {
    let struct_name = entry.struct_name();
    let key = entry.file_name();
    let name = &entry.name;
    let target = &entry.target;
    let category = &entry.category;
    let spec = &entry.spec;
    let significance = &entry.significance;
    let mdn = &entry.mdn;
    let exec = &entry.exec;
    let subtests = entry
        .subtests
        .iter()
        .map(|t| {
            let name = &t.name;
            let exec = &t.exec;
            quote! {
                Subtest {
                    name: #name,
                    exec: #exec,
                }

            }
        })
        .collect::<Vec<_>>();

    let token_stream = quote! {
        use crate::feature::{Meta, Subtest};
        use crate::features::#struct_name;

        impl Meta for #struct_name {
            fn name(&self) -> &'static str {
                #name
            }
            fn key(&self) -> &'static str {
                #key
            }
            fn target(&self) -> &'static str {
                #target
            }
            fn category(&self) -> &'static str {
                #category
            }
            fn spec(&self) -> &'static str {
                #spec
            }
            fn significance(&self) -> &'static str{
                #significance
            }
            fn mdn(&self) -> &'static str{
                #mdn
            }
            fn exec(&self) -> &'static str{
                #exec
            }
            fn subtests(&self) -> Vec<Subtest> {
                vec![#(#subtests,)*]
            }
        }
    };
    let filename = format!("meta/{}.rs", entry.file_name());
    generate_file(&filename, token_stream);
}

fn generate_features(entries: &[Entry]) {
    let struct_names = entries.iter().map(|e| e.struct_name()).collect::<Vec<_>>();

    let token_stream = quote! {
        use crate::feature::Feature;

        pub const FEATURES: &[&dyn Feature] = &[
            #(&#struct_names,)*
        ];

        #(pub struct #struct_names;)*

    };
    generate_file("features.rs", token_stream);
}

fn generate_implementation_module(entries: &[Entry]) {
    let mods = entries.iter().map(|e| format_ident!("{}", e.file_name().to_string()));
    let token_stream = quote! {
        #(mod #mods;)*
    };
    generate_file("implementation.rs", token_stream);
}

fn generate_meta_module(entries: &[Entry]) {
    let mods = entries.iter().map(|e| format_ident!("{}", e.file_name().to_string()));
    let token_stream = quote! {
        #(mod #mods;)*
    };
    generate_file("meta.rs", token_stream);
}

fn generate_file(file: &str, token_stream: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    let project_root = get_project_root().unwrap();
    fs::write(project_root.join("src").join(file), code).unwrap();
}
