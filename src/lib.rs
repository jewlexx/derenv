use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};

mod inner;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn dotenv(args: TokenStream, input: TokenStream) -> TokenStream {
    inner::dotenv(
        parse_macro_input!(args as AttributeArgs),
        parse_macro_input!(input as DeriveInput),
    )
    .into()
}
