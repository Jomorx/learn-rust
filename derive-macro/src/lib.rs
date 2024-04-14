use proc_macro::{Ident, Span, TokenStream, TokenTree};
use syn::DeriveInput;
use quote::quote;
#[proc_macro_derive(MyDeriveExample)]
pub fn my_derive_example(input: TokenStream) -> TokenStream {
    // let tokens:Vec<_> = input.into_iter().map(|token| match &token {
    //     TokenTree::Ident(ident) => {
    //         if ident.to_string() == "Student" {
    //             return TokenTree::Ident(Ident::new("HelloClass", Span::call_site()));
    //         }
    //         return  token;
    //     }
    //     _ => token,
    // }).collect();

    // let input = TokenStream::from_iter(tokens.into_iter());
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    quote! (
        impl #name {
            fn say_hello(&self){
                println!("say hello")
            }
        }
    ).into()
}
