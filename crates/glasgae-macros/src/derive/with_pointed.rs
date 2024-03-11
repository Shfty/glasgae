use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub fn r#impl(input: TokenStream) -> TokenStream {
    let s: ItemStruct = parse_macro_input!(input);

    let mut args: Vec<_> = s.generics.params.into_iter().collect();
    let free = args.pop().expect("Type has no free generic parameter");
    let ty = s.ident;

    let out = quote! {
        impl<#(#args,)* #free, _NewPointed> glasgae::prelude::WithPointed<_NewPointed> for #ty<#(#args,)* #free>
        where
            #(#args: glasgae::prelude::Term,)*
            #free: glasgae::prelude::Term,
            _NewPointed: glasgae::prelude::Term,
        {
            type WithPointed = #ty<#(#args,)* _NewPointed>;
        }
    };

    out.into()
}
