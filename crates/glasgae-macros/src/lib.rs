use proc_macro::TokenStream;
mod punct;

mod r#do;
mod infix;

/// Basic implementation of Haskell `do` sugar.
#[proc_macro]
pub fn r#do(input: TokenStream) -> TokenStream {
    r#do::r#impl(input)
}

#[proc_macro]
pub fn infix(input: TokenStream) -> TokenStream {
    infix::r#impl(input)
}
