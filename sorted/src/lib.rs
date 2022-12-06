use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input as syn::Item);
    assert!(args.is_empty());
    sorted_impl(ty).into()
}

fn sorted_impl(input: syn::Item) -> proc_macro2::TokenStream {
    if let syn::Item::Enum(e) = input {
        quote! {#e}
    } else {
        quote! {compile_error!("expected enum or match expression");}
    }
}
