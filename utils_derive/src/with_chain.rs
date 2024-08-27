use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn expand_with_chain(input: DeriveInput) -> TokenStream {
    let item_ident = input.ident;
    let with_fns = match input.data {
        syn::Data::Struct(str) => str.fields.into_iter().filter_map(|f| {
            if let Some(ident) = f.ident {
                let fn_name = format_ident!("with_{ident}");
                let ty = f.ty;
                Some(quote! {
                    pub fn #fn_name(mut self, val: #ty) -> Self {
                        self.#ident = val;
                        self
                    }
                })
            } else {
                None
            }
        }),
        _ => panic!("With chaining only support struct type"),
    };
    let expanded = quote! {
        impl #item_ident {
            #(#with_fns)*
        }
    };

    expanded.into()
}
