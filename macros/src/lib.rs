#![doc(html_root_url = "https://docs.rs/qjack_macros/0.1.0")]

use proc_macro::TokenStream;

mod model;
#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    model::derive_model(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
