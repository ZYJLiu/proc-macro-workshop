use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input as syn::Item);
    assert!(args.is_empty());
    sorted_impl(ty)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn sorted_impl(input: syn::Item) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let syn::Item::Enum(e) = input {
        let mut names = Vec::new();
        for variant in e.variants.iter() {
            let name = variant.ident.to_string();
            if names.last().map(|last| &name < last).unwrap_or(false) {
                let next_lex_i = names.binary_search(&name).unwrap_err();
                return Err(syn::Error::new(
                    variant.span(),
                    format!("{} should sort before {}", name, names[next_lex_i]),
                ));
            }
            names.push(name);
        }
        Ok(quote! {#e})
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        ))
    }
}
