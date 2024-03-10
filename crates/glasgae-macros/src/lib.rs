use proc_macro::TokenStream;

#[proc_macro]
pub fn r#do(input: TokenStream) -> TokenStream {
    panic!("{input:#?}");
    input
}

