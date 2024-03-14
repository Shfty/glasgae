use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Local, Pat, PatType, Stmt, Token};

use crate::punct;

struct TypedPat(Pat);

impl Parse for TypedPat {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if PatType::parse(&input.fork()).is_ok() {
            return Ok(TypedPat(Pat::Type(PatType::parse(input).unwrap())));
        };

        match Pat::parse_single(&input.fork()) {
            Ok(_) => Ok(TypedPat(Pat::parse_single(input).unwrap())),
            Err(e) => Err(e),
        }
    }
}

impl ToTokens for TypedPat {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

struct DoChain {
    pat: Pat,
    _sep: punct::DoBind,
    expr: Expr,
    _semi: Token![;],
}

impl Parse for DoChain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _input = input.fork();
        let _pat: Pat = TypedPat::parse(&_input)?.0;
        let __sep: punct::DoBind = _input.parse()?;
        let _expr: Expr = _input.parse()?;
        let __semi: Token![;] = _input.parse()?;

        let pat: Pat = TypedPat::parse(input).unwrap().0;
        let _sep: punct::DoBind = input.parse().unwrap();
        let expr: Expr = input.parse().unwrap();
        let _semi: Token![;] = input.parse().unwrap();

        Ok(DoChain {
            pat,
            _sep,
            expr,
            _semi,
        })
    }
}

enum DoTerm {
    Chain(DoChain),
    Let(Local),
    Then(Expr),
    Return(Expr),
}

impl Parse for DoTerm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.fork().parse::<DoChain>() {
            Ok(_) => Ok(DoTerm::Chain(input.parse().unwrap())),
            _ => match input.fork().parse::<Stmt>() {
                Ok(Stmt::Local(local)) => {
                    let _stmt: Stmt = input.parse().unwrap();
                    Ok(DoTerm::Let(local))
                }
                _ => match input.fork().parse::<Expr>() {
                    Ok(expr) => {
                        let _expr: Expr = input.parse().unwrap();
                        match input.fork().parse::<Token![;]>() {
                            Ok(_) => {
                                input.parse::<Token![;]>().unwrap();
                                Ok(DoTerm::Then(expr))
                            }
                            Err(_) => Ok(DoTerm::Return(expr)),
                        }
                    }
                    Err(e) => Err(e),
                },
            },
        }
    }
}

struct DoBlock(Vec<DoTerm>);

impl Parse for DoBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut out = vec![];
        while let Ok(term) = DoTerm::parse(input) {
            out.push(term);
        }

        Ok(DoBlock(out))
    }
}

impl ToTokens for DoBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(
            self.0.iter().rfold(
                TokenStream2::default(),
                |acc: TokenStream2, next| match next {
                    DoTerm::Chain(DoChain { pat, expr, .. }) => quote! {
                        glasgae::prelude::ChainM::chain_m(
                            #expr,
                            |#pat| #acc
                        )
                    },
                    DoTerm::Let(local) => {
                        quote!({#local #acc})
                    }
                    DoTerm::Then(expr) => {
                        quote! {
                            glasgae::prelude::ThenM::then_m(
                                #expr, #acc
                            )
                        }
                    }
                    DoTerm::Return(expr) => {
                        assert!(acc.is_empty(), "Trailing input:\n{acc}");
                        quote!(#expr)
                    }
                },
            ),
        );
    }
}

pub fn r#impl(input: TokenStream) -> TokenStream {
    let block: DoBlock = parse_macro_input!(input);
    block.to_token_stream().into()
}
