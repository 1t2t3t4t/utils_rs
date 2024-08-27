use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use with_chain::expand_with_chain;

mod with_chain;

#[proc_macro_derive(WithChain)]
pub fn derive_with_chain(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_with_chain(input)
}
