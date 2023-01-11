use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort_call_site, emit_error};
use quote::quote;
use syn::{parse::Parser, spanned::Spanned, AttributeArgs, Data, DeriveInput, Lit, NestedMeta};

#[allow(unused_variables)]
fn get_base_path(ast_span: Span) -> PathBuf {
    cfg_if::cfg_if! {
        if #[cfg(feature = "nightly")]
        {
            let mut source_path = ast_span.source_file().path();
            source_path.pop();

            source_path
        } else {
            std::env::current_dir().unwrap()
        }
    }
}

fn create_field(env_name: &str) -> syn::Field {
    let named_field: TokenStream = env_name.parse().unwrap();

    syn::Field::parse_named
        .parse2(quote! { pub #named_field: &'static str })
        .unwrap()
}

fn parse_file(path: &Path) -> io::Result<Vec<(String, String)>> {
    let mut file = File::open(path)?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let assignments = file_contents
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.starts_with('#') || line.is_empty() {
                return None;
            }

            let mut parts = line.splitn(2, '=');

            let name = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap_or_default().trim().to_string();

            Some((name, value))
        })
        .collect();

    Ok(assignments)
}

pub(crate) fn dotenv(input_args: AttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let mut path = get_base_path(ast.span());

    let mut suffix_path: String = ".env".to_string();

    for arg in input_args {
        if let NestedMeta::Lit(Lit::Str(literal)) = arg {
            suffix_path = literal.value();
        } else {
            emit_error!(arg.span(), "expected string literal for path");
        }
    }

    path.push(suffix_path);

    let assignments = match parse_file(&path) {
        Ok(assignments) => assignments,
        Err(_) => abort_call_site!("failed to parse file at path: {}", path.display()),
    };

    match &mut ast.data {
        Data::Struct(ref mut struct_data) => {
            if let syn::Fields::Named(fields) = &mut struct_data.fields {
                for (name, _) in &assignments {
                    fields.named.push(create_field(name));
                }
            } else {
                let ident = ast.ident;
                abort_call_site!(
                    "expected named struct fields. declare struct as \"struct {} {{}}\"",
                    ident
                )
            }
        }
        _ => abort_call_site!("expected struct"),
    };

    quote! {
        #ast
    }
}
