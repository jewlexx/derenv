use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{AttributeArgs, DeriveInput};

pub(crate) fn dotenv(args: AttributeArgs, ast: DeriveInput) -> TokenStream {
    quote! {
        // const STRUCT_TOKENS: &str = #tokens_str;
    }
}
