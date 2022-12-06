use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    // let args = dbg!(parse_macro_input!(args as syn::Meta));
    let ty = dbg!(parse_macro_input!(input as syn::ItemEnum));
    let ts = quote! {#ty};
    ts.into()
}
