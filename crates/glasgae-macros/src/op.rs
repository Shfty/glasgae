use glasgae_kiss::{read_operators, Fixity, Operator};
use proc_macro::TokenStream;
use proc_macro2::{Spacing, TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::quote;
use syn::Path;

#[derive(Debug, Clone)]
enum Atom {
    TokenTree(TokenTree2),
    TokenStream(TokenStream2),
    Operator(Operator),
}

fn token_stream_to_atoms(input: TokenStream2) -> Vec<Atom> {
    input.into_iter().map(Atom::TokenTree).collect()
}

fn split_ops(input: Vec<Atom>) -> Vec<Atom> {
    // Join into a list of TokenStream and Operator
    input.into_iter().fold(vec![], |mut acc, next| {
        let last = acc.pop();
        match (last, next) {
            (None, Atom::TokenTree(tt)) => acc.push(Atom::TokenStream(tt.into())),
            (None, ts @ Atom::TokenStream(_)) => acc.push(ts),
            (Some(Atom::TokenStream(l)), Atom::TokenTree(r)) => {
                acc.push(Atom::TokenStream(l.into_iter().chain([r]).collect()))
            }
            (Some(Atom::TokenStream(l)), Atom::TokenStream(r)) => {
                acc.push(Atom::TokenStream(l.into_iter().chain(r).collect()))
            }
            (Some(ts @ Atom::TokenStream(_)), op @ Atom::Operator(_)) => {
                acc.push(ts);
                acc.push(op);
                acc.push(Atom::TokenStream(Default::default()))
            }
            _ => unimplemented!(),
        };
        acc
    })
}

fn parse(op1: Operator, e1: TokenStream2, mut rest: Vec<Atom>) -> (TokenStream2, Vec<Atom>) {
    if rest.is_empty() {
        return (e1, vec![]);
    }

    let Atom::Operator(op2) = rest.remove(0) else {
        panic!("Atom is not an Operator");
    };

    // Illegal expressions
    if op1.prec() == op2.prec() && (op1.fixity() != op2.fixity() || op1.fixity() == Fixity::None) {
        panic!("Illegal expression");
    }

    // Left associative
    if op1.prec() > op2.prec() || (op1.prec() == op2.prec() && op1.fixity() == Fixity::Left) {
        return (e1, [Atom::Operator(op2)].into_iter().chain(rest).collect());
    }

    // Right associative
    let Atom::TokenStream(next) = rest.remove(0) else {
        panic!("Atom is not a TokenStream");
    };

    let (r, rest_) = parse(op2.clone(), next, rest);

    let path: Path = syn::parse_str(op2.func()).expect("Invalid function path");
    let flip = op1.flip();
    parse(
        op1,
        if flip {
            quote!(#path(#r, #e1))
        } else {
            quote!(#path(#e1, #r))
        },
        rest_,
    )
}

fn resolve(mut input: Vec<Atom>) -> TokenStream2 {
    let Atom::TokenStream(e1) = input.remove(0) else {
        panic!("Atom is not a TokenStream")
    };

    let (rest, _) = parse(Operator::new(Fixity::None, -1, "", "", false), e1, input);

    rest
}

pub fn r#impl(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let input: Vec<_> = token_stream_to_atoms(input);
    let ops = split_ops(input);
    let out = resolve(ops);
    out.into()
}
