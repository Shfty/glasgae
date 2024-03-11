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

fn split_op(toks: &[Atom], op: &Operator) -> Option<(Vec<Atom>, Vec<Atom>)> {
    let enumerated = toks.iter().enumerate().collect::<Vec<_>>();

    let mut windows = enumerated.windows(op.op().len()).collect::<Vec<_>>();

    if op.fixity() == Fixity::Right {
        windows.reverse();
    }

    windows
        .into_iter()
        .find_map(|t| {
            let Ok(t) = t.iter().try_fold(vec![], |mut acc, (i, next)| {
                let Atom::TokenTree(TokenTree2::Punct(p)) = next else {
                    return Err(());
                };

                acc.push((i, p));

                Ok(acc)
            }) else {
                return None;
            };

            if t.iter()
                .take(t.len() - 1)
                .any(|(_, t)| t.spacing() != Spacing::Joint)
            {
                return None;
            }

            if t.last().map(|(_, t)| t.spacing()) != Some(Spacing::Alone) {
                return None;
            }

            if t.iter()
                .zip(op.op().chars())
                .any(|((_, p), c)| p.as_char() != c)
            {
                return None;
            }

            Some((t.first().unwrap().0, t.last().unwrap().0))
        })
        .map(|(s, e)| (toks[..*s].to_vec(), toks[*e + 1..].to_vec()))
}

fn split_ops_impl(input: Vec<Atom>) -> Vec<Atom> {
    let out_dir = std::env!("PROC_ARTIFACT_DIR");
    let ops = read_operators(out_dir).expect("Failed to read operators");

    match ops
        .0
        .iter()
        .find_map(|(_, op)| split_op(&input, op).map(|(lhs, rhs)| (lhs, op, rhs)))
    {
        Some((lhs, op, rhs)) => {
            let lhs = split_ops_impl(lhs);
            let rhs = split_ops_impl(rhs);
            lhs.into_iter()
                .chain([Atom::Operator(op.clone())])
                .chain(rhs)
                .collect()
        }
        None => input,
    }
}

fn token_stream_to_atoms(input: TokenStream2) -> Vec<Atom> {
    input.into_iter().map(Atom::TokenTree).collect()
}

fn split_ops(input: Vec<Atom>) -> Vec<Atom> {
    // Split into a list of TokenTree and Operator
    let ops = split_ops_impl(input);

    // Join into a list of TokenStream and Operator
    ops.into_iter().fold(vec![], |mut acc, next| {
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

fn parse_neg(op1: Operator, mut rest: Vec<Atom>) -> (TokenStream2, Vec<Atom>) {
    let Atom::TokenStream(e1) = rest.remove(0) else {
        panic!("Atom is not a TokenStream")
    };

    parse(op1, e1, rest)
}

fn resolve(input: Vec<Atom>) -> TokenStream2 {
    let (rest, _) = parse_neg(Operator::new(Fixity::None, -1, "", "", false), input);

    rest
}

pub fn r#impl(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let input: Vec<_> = token_stream_to_atoms(input);
    let ops = split_ops(input);
    let out = resolve(ops);
    out.into()
}
