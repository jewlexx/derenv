use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{AttributeArgs, Data, DeriveInput};

pub(crate) fn dotenv(args: AttributeArgs, mut ast: DeriveInput) -> TokenStream {
    match ast.data {
        Data::Struct(ref mut struct_data) => {}
        _ => abort_call_site!("expected struct"),
    };

    quote! {
        // const STRUCT_TOKENS: &str = #tokens_str;
    }
}
