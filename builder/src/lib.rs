extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn ty_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty.value() {
                return Some(t);
            }
        }
    }
    None
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if ty_inner_type(ty).is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: std::option::Option<#ty> }
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if let Some(inner_ty) = ty_inner_type(&ty) {
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let extend_methods = fields.iter().filter_map(|f| {
        for attr in &f.attrs {
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
                if let Some(TokenTree::Group(g)) = attr.tts.clone().into_iter().next() {
                    let mut tokens = g.stream().into_iter();
                    match tokens.next().unwrap() {
                        TokenTree::Ident(ref i) => assert_eq!(i, "each"),
                        tt => panic!("expect `each`, found {}", tt),
                    }
                    match tokens.next().unwrap() {
                        TokenTree::Punct(ref p) => assert_eq!(p.as_char(), '='),
                        tt => panic!("expect `=`, found {}", tt),
                    }
                    let arg = match tokens.next().unwrap() {
                        TokenTree::Literal(l) => l,
                        tt => panic!("expected string, found{}", tt),
                    };
                    match syn::Lit::new(arg) {
                        syn::Lit::Str(s) => {
                            let arg = syn::Ident::new(&s.value(), s.span());
                            return Some(quote! {fn #arg() {}});
                        }
                        lit => panic!("expected string, found {:?}", lit),
                    }
                }
            }
        }
        None
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if ty_inner_type(&f.ty).is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                    #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let build_empty = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {#name: None}
    });

    let expanded = quote! {
        struct #bident {
            #(#optionized,)*
        }
        impl #bident {
            #(#methods)*
            #(#extend_methods)*
            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
        impl #name {
            fn builder() -> #bident {
                #bident {
                     #(#build_empty,)*
                }
            }
        }
    };

    expanded.into()
}
