use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Pat, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

// [x * 2 for x <- xs]
// expression 'for' pattern '<-' expression
struct Comp {
    mapping: Expr,
    pattern: Pat,
    sequence: Expr,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> Result<Self> {
        let mapping = input.parse::<Expr>()?;
        _ = input.parse::<Token![for]>()?;
        let pattern = Pat::parse_single(input)?;
        _ = input.parse::<Token![<-]>()?;
        let sequence = input.parse::<Expr>()?;
        Ok(Comp {
            mapping,
            pattern,
            sequence,
        })
    }
}

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    // Custom Syntax Parsing
    let code = parse_macro_input!(input as Comp);
    let Comp {
        mapping,
        pattern,
        sequence,
    } = code;
    // Rust Code Generation
    quote! {
        core::iter::IntoIterator::into_iter(#sequence).map(|#pattern|{
            #mapping
        })
    }
    .into()
}
