use proc_macro::TokenStream;
mod punct;

mod _do;
mod op;

/// Basic implementation of Haskell `do` sugar.
#[proc_macro]
pub fn _do(input: TokenStream) -> TokenStream {
    _do::r#impl(input)
}

#[proc_macro]
pub fn op(input: TokenStream) -> TokenStream {
    op::r#impl(input)
}
