use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn uow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let expanded = quote! {
        async fn #name(#inputs) #output {
            self.uow.run(async { #block }).await
        }
    };

    TokenStream::from(expanded)
}
