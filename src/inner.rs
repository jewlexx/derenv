use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, abort_call_site, emit_call_site_error, emit_error};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, AttributeArgs, Data, DeriveInput, Lit, Meta, NestedMeta};

pub(crate) fn dotenv(input_args: AttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let mut path: String = ".env".to_string();
    let mut is_public: bool = false;

    for arg in input_args {
        if let NestedMeta::Meta(Meta::NameValue(assignment)) = arg {
            if assignment.path.is_ident("path") {
                if let Lit::Str(literal) = assignment.lit {
                    path = literal.value();
                } else {
                    emit_error!(assignment.lit.span(), "expected string literal");
                }
            } else if assignment.path.is_ident("is_public") {
                if let Lit::Bool(literal) = assignment.lit {
                    is_public = literal.value();
                } else {
                    emit_error!(assignment.lit.span(), "expected boolean literal");
                }
            }
        }
    }

    match ast.data {
        Data::Struct(ref mut struct_data) => {}
        _ => abort_call_site!("expected struct"),
    };

    quote! {
        // const STRUCT_TOKENS: &str = #tokens_str;
    }
}
