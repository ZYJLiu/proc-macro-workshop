use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Token};

#[derive(Debug)]
struct SeqMacroInput {/* ... */}

impl Parse for SeqMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let var = syn::Ident::parse(input)?;
        let _in = <Token![in]>::parse(input)?;
        let from = syn::Lit::parse(input)?;
        let _dots = <Token![..]>::parse(input)?;
        let to = syn::Lit::parse(input)?;
        let content;
        let braces = syn::braced!(content in input);

        // eprintln!("{:?} {:?} {:?} {:?} {:?}", var, _in, from, _dots, braces);
        let tt = proc_macro2::TokenStream::parse(&content)?;
        eprintln!("{:?}", var);
        eprintln!("{:?}", _in);
        eprintln!("{:?}", from);
        eprintln!("{:?}", _dots);
        eprintln!("{:?}", to);
        eprintln!("{:?}", braces);
        eprintln!("{:?}", tt);

        Ok(SeqMacroInput {})
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqMacroInput);
    println!("{:?}", input);
    TokenStream::new()
}
