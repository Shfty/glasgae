use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub fn r#impl(input: TokenStream) -> TokenStream {
    let s: ItemStruct = parse_macro_input!(input);

    let mut args: Vec<_> = s.generics.params.into_iter().collect();
    let free = args.pop().expect("Type has no free generic parameter");
    let ty = s.ident;

    let out = quote! {
        impl<#(#args,)* #free, _NewPointed> glasgae::prelude::Functor<_NewPointed> for #ty<#(#args,)* #free>
        where
            #(
                #args: Term,
            )*
            #free: glasgae::prelude::Functor<_NewPointed>,
            _NewPointed: glasgae::prelude::Term,
            StateLoggingT_<LVL, MSG, S, MA>: Functor<_NewPointed>,
        {
            fn fmap(self, f: impl glasgae::prelude::FunctionT<Self::Pointed, _NewPointed>) -> Self::WithPointed {
                #ty(self.0.fmap(f.to_function()))
            }
        }
    };

    out.into()
}
