use crate::{
    base::{
        control::zipper::{Travel, Zipper},
        data::function::bifunction::BifunT,
    },
    prelude::*,
    transformers::cont::Cont,
};

pub trait ZipMove<D, M> {
    fn zip_move(self, dir: D) -> M;
}

impl<M, T, D> ZipMove<D, M> for Zipper<T, D>
where
    M: ReturnM<Pointed = Zipper<T, D>>,
    T: std::fmt::Debug,
    D: std::fmt::Debug,
{
    fn zip_move(self, dir: D) -> M {
        match self {
            Zipper::Zipper(t, n) => {
                println!("Dir: {dir:#?}\nTerm: {t:#?}\n");
                ReturnM::return_m(n((None, dir)))
            }
            _ => unimplemented!(),
        }
    }
}

impl<M, T, D> ZipMove<D, M> for Cont<T>
where
    Self: ChainM<M, Pointed = T>,
    M: Clone + ReturnM<Pointed = T>,
    T: ZipMove<D, M>,
    D: 'static + Clone,
{
    fn zip_move(self, dir: D) -> M {
        self.chain_m(|t| t.zip_move(dir))
    }
}

pub trait ZipAllTheWay<M, T, D>: Sized {
    fn zip_all_the_way(self, dir: D, f: impl FunctionT<T, Option<T>> + Clone) -> M;
}

impl<M, T, D> ZipAllTheWay<M, T, D> for Zipper<T, D>
where
    M: ReturnM<Pointed = Zipper<T, D>>,
    D: Clone + std::fmt::Debug,
{
    fn zip_all_the_way(self, dir: D, f: impl FunctionT<T, Option<T>> + Clone) -> M {
        match self {
            Zipper::Zipper(t, k) => k((f.clone()(t), dir.clone())).zip_all_the_way(dir, f),
            Zipper::ZipDone(t) => ReturnM::return_m(Zipper::done(t)),
        }
    }
}

impl<M, T, D> ZipAllTheWay<M, T::Pointed, D> for Cont<T>
where
    Self: ChainM<M, Pointed = T>,
    M: Clone + ReturnM<Pointed = T>,
    T: Pointed + ZipAllTheWay<M, T::Pointed, D>,
    D: 'static + Clone,
{
    fn zip_all_the_way(
        self,
        dir: D,
        f: impl FunctionT<T::Pointed, Option<T::Pointed>> + Clone,
    ) -> M {
        self.chain_m(|t| t.zip_all_the_way(dir, f))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Var(String),
    L(String, Box<Self>),
    A(Box<Self>, Box<Self>),
    Free,
}

impl Pointed for Term {
    type Pointed = String;
}

impl WithPointed<String> for Term {
    type WithPointed = Term;
}

impl Functor<String> for Term {
    fn fmap(self, f: impl FunctionT<String, String> + Clone) -> Term {
        match self {
            Term::Var(t) => Term::var(f(t)),
            Term::L(s, n) => Term::l(f.clone()(s), n.fmap(f)),
            Term::A(l, r) => Term::a(l.fmap(f.clone()), r.fmap(f)),
            Term::Free => unimplemented!(),
        }
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(s) => f.write_str(s),
            Term::A(l, r) => f.write_fmt(format_args!("({l:?} {r:?})")),
            Term::L(t, n) => f.write_fmt(format_args!("L{t}. {n:?}")),
            Term::Free => f.write_str("()"),
        }
    }
}

impl Term {
    pub fn var(t: impl ToString) -> Self {
        Term::Var(t.to_string())
    }

    pub fn a(lhs: Self, rhs: Self) -> Self {
        Term::A(lhs.boxed(), rhs.boxed())
    }

    pub fn l(t: impl ToString, n: Self) -> Self {
        Term::L(t.to_string(), n.boxed())
    }

    pub fn a_free() -> Self {
        Self::a(Term::Free, Term::Free)
    }

    pub fn l_free(t: impl ToString) -> Self {
        Self::l(t, Term::Free)
    }

    pub fn left(self, t: Self) -> Self {
        let Term::A(_, r) = self else {
            panic!("Term is not an A")
        };
        Term::a(t, *r)
    }

    pub fn right(self, t: Self) -> Self {
        let Term::A(l, _) = self else {
            panic!("Term is not an A")
        };
        Term::a(*l, t)
    }

    pub fn next(self, t: Self) -> Self {
        let Term::L(s, _) = self else {
            panic!("Term is not an L");
        };
        Term::l(s, t)
    }
}

impl Foldable<String, String> for Term {
    fn foldr(self, f: impl BifunT<String, String, String> + Clone, init: String) -> String {
        match self {
            Term::Var(t) => f(t, init),
            Term::L(l, r) => f.clone()(l, r.foldr(f, init)),
            Term::A(l, r) => l.foldr(f.clone(), r.foldr(f, init)),
            Term::Free => unimplemented!(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    #[default]
    Up,
    Next,
    DownLeft,
    DownRight,
}

impl<M, N> Travel<Direction, M, N> for Term
where
    M: ChainM<N, Pointed = (Option<Term>, Direction)>,
    N: 'static + Clone + ChainM<N, Pointed = Term> + ReturnM<Pointed = Term>,
{
    fn travel(self, tf: impl FunctionT<Self, M> + Clone) -> N {
        tf.clone()(self.clone()).chain_m(|(term_, dir)| {
            let t = term_.unwrap_or(self);

            match (dir, t) {
                (Direction::Up, t) | (_, t @ Term::Var(_)) => ReturnM::return_m(t),
                (_, Term::L(v, t1)) => Travel::<Direction, M, N>::travel(*t1, tf)
                    .chain_m(|t1| ReturnM::return_m(Term::l(v, t1))),
                (Direction::Next, Term::A(l, r)) => {
                    Travel::<Direction, M, N>::travel(*l, tf.clone()).chain_m(|l| {
                        Travel::<Direction, M, N>::travel(*r, tf)
                            .chain_m(|r| ReturnM::return_m(Term::a(l, r)))
                    })
                }
                (Direction::DownLeft, Term::A(l, r)) => Travel::<Direction, M, N>::travel(*l, tf)
                    .chain_m(|l| ReturnM::return_m(Term::a(l, *r))),
                (Direction::DownRight, Term::A(l, r)) => Travel::<Direction, M, N>::travel(*r, tf)
                    .chain_m(|r| ReturnM::return_m(Term::a(*l, r))),
                _ => unimplemented!(),
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction1 {
    FirstKid,
    RightKid,
    Parent,
}

impl<M, N> Travel<Direction1, M, N> for Term
where
    M: ChainM<N, Pointed = (Option<Term>, Direction1)>,
    N: 'static + Clone + ChainM<N, Pointed = Term> + ReturnM<Pointed = Term>,
{
    fn travel(self, tf: impl FunctionT<Self, M> + Clone) -> N {
        tf.clone()(self.clone()).chain_m(|(term_, dir)| {
            let t = term_.unwrap_or(self);

            match (dir, t) {
                (Direction1::Parent, t) => ReturnM::return_m(t),
                (dir, Term::L(v, t1))
                    if dir == Direction1::FirstKid || dir == Direction1::RightKid =>
                {
                    t1.travel(tf.clone())
                        .chain_m(|t1| Term::l(v, t1).travel(tf))
                }
                (Direction1::RightKid, Term::A(l, r)) => {
                    r.travel(tf.clone()).chain_m(|r| Term::a(*l, r).travel(tf))
                }
                (Direction1::FirstKid, Term::A(l, r)) => {
                    l.travel(tf.clone()).chain_m(|l| Term::a(l, *r).travel(tf))
                }
                _ => unimplemented!(),
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::base::{control::zipper::ZipTravel, data::functor::identity::Identity};

    use super::*;

    fn term() -> Term {
        let x = Term::var("x");
        let f = Term::var("f");

        let term =
            Term::l_free("f").next(
                Term::l_free("x").next(
                    Term::a_free()
                        .left(Term::a_free().left(f.clone()).right(
                            Term::l_free("f").next(
                                Term::a_free().left(f.clone()).right(
                                    Term::l_free("f").next(Term::l_free("x").next(x.clone())),
                                ),
                            ),
                        ))
                        .right(
                            Term::a_free()
                                .left(Term::a_free().left(f.clone()).right(
                                    Term::l_free("f").next(Term::l_free("x").next(x.clone())),
                                ))
                                .right(x.clone()),
                        ),
                ),
            );

        println!("Term:\n{term:#?}");

        term
    }

    #[test]
    fn test_1() {
        let term = term()
            .travel(|_| Identity::return_m((None, Direction::Next)))
            .run();

        println!("Done:\n{term:#?}");
    }

    #[test]
    fn test_2() {
        let term = term();

        let term = term
            .travel(|term| {
                println!("Term: {term:#?}");
                Identity::return_m((None, Direction::Next))
            })
            .run();

        println!("Done:\n{term:#?}");
    }

    #[test]
    fn test_3() {
        let term = term();

        let term = term
            .travel(|term| {
                Identity::return_m(match term {
                    Term::A(ref l, _) if matches!(**l, Term::Var(ref s) if s == "f") => {
                        println!("Cutting {term:#?}");
                        (None, Direction::Up)
                    }
                    term => {
                        println!("Term: {term:#?}");
                        (None, Direction::DownLeft)
                    }
                })
            })
            .run();

        println!("Done:\n{term:#?}");
    }

    #[test]
    fn test_4() {
        let term = term();

        let term = term.travel(|t| {
            Identity::return_m(match t {
                Term::L(t, n) if t == "x" && matches!(&*n, Term::Var(s) if s == "x") => {
                    println!("Replacing...");
                    (Some(Term::l("y", Term::var("y"))), Direction::Next)
                }
                t => {
                    println!("Term: {t:#?}");
                    (None, Direction::Next)
                }
            })
        });

        println!("Done:\n{term:#?}");
    }

    #[test]
    fn test_zip_1() {
        let term: Cont<Zipper<Term, Direction>> =
            term().zip_travel().zip_all_the_way(Direction::Next, |t| {
                println!("Encountered: {t:#?}");
                None
            });

        let term = term.eval();
        println!("Term: {term:#?}");
    }

    #[test]
    fn test_zip_2() {
        let term: Cont<Zipper<Term, Direction>> = term()
            .zip_travel()
            .zip_move(Direction::Next)
            .zip_move(Direction::Next)
            .zip_move(Direction::Next)
            .zip_move(Direction::DownRight);

        let term = term.fmap(
            Functor::replace
                .flip_clone()
                .curry_clone(Term::a(Term::var("x"), Term::var("x"))),
        );

        let term: Cont<Zipper<Term, Direction>> = term.zip_all_the_way(Direction::Up, |t| {
            println!("Zipping Up: {t:#?}");
            None
        });

        let term = term.eval_t().run();

        println!("Done: {term:#?}");
    }

    #[test]
    fn test_zip_3() {
        let term: Cont<Zipper<Term, Direction1>> =
            Term::l("x", Term::a(Term::var("a"), Term::var("b")))
                .zip_travel()
                .zip_move(Direction1::FirstKid)
                .zip_move(Direction1::FirstKid)
                .zip_move(Direction1::Parent)
                .zip_move(Direction1::RightKid)
                .zip_move(Direction1::Parent)
                .zip_move(Direction1::Parent)
                .zip_move(Direction1::Parent);

        let term = term.eval_t().run();

        println!("Done:\n{term:#?}");
    }
}
