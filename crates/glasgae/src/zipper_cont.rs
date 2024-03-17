use crate::{
    base::{
        control::zipper::{Travel, Zipper},
        data::{foldr1_default, function::bifunction::BifunT, Foldable1},
    },
    prelude::*,
    transformers::cont::Cont,
};

pub trait ZipMove<D, M>: Term {
    fn zip_move(self, dir: D) -> M;
}

impl<M, T, D> ZipMove<D, M> for Zipper<T, D>
where
    M: ReturnM<Pointed = Zipper<T, D>>,
    T: Term + std::fmt::Debug,
    D: Term + std::fmt::Debug,
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
    M: ReturnM<Pointed = T>,
    T: ZipMove<D, M>,
    D: Term,
{
    fn zip_move(self, dir: D) -> M {
        self.chain_m(|t| t.zip_move(dir))
    }
}

pub trait ZipAllTheWay<M, T, D>: Term
where
    T: Term,
{
    fn zip_all_the_way(self, dir: D, f: impl FunctionT<T, Option<T>>) -> M;
}

impl<M, T, D> ZipAllTheWay<M, T, D> for Zipper<T, D>
where
    M: ReturnM<Pointed = Zipper<T, D>>,
    T: Term,
    D: Term,
{
    fn zip_all_the_way(self, dir: D, f: impl FunctionT<T, Option<T>>) -> M {
        let f = f.to_function();
        match self {
            Zipper::Zipper(t, k) => k((f.clone()(t), dir.clone())).zip_all_the_way(dir, f),
            Zipper::ZipDone(t) => ReturnM::return_m(Zipper::done(t)),
        }
    }
}

impl<M, T, D> ZipAllTheWay<M, T::Pointed, D> for Cont<T>
where
    Self: ChainM<M, Pointed = T>,
    M: ReturnM<Pointed = T>,
    T: Pointed + ZipAllTheWay<M, T::Pointed, D>,
    D: Term,
{
    fn zip_all_the_way(self, dir: D, f: impl FunctionT<T::Pointed, Option<T::Pointed>>) -> M {
        let f = f.to_function();
        self.chain_m(|t| t.zip_all_the_way(dir, f))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZipperTerm {
    Var(String),
    L(String, Box<Self>),
    A(Box<Self>, Box<Self>),
    Free,
}

impl Pointed for ZipperTerm {
    type Pointed = String;
}

impl WithPointed<String> for ZipperTerm {
    type WithPointed = ZipperTerm;
}

impl Functor<String> for ZipperTerm {
    fn fmap(self, f: impl FunctionT<String, String>) -> ZipperTerm {
        match self {
            ZipperTerm::Var(t) => ZipperTerm::var(f(t)),
            ZipperTerm::L(s, n) => ZipperTerm::l(f.to_function()(s), n.fmap(f)),
            ZipperTerm::A(l, r) => ZipperTerm::a(l.fmap(f.to_function()), r.fmap(f)),
            ZipperTerm::Free => unimplemented!(),
        }
    }
}

impl std::fmt::Debug for ZipperTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZipperTerm::Var(s) => f.write_str(s),
            ZipperTerm::A(l, r) => f.write_fmt(format_args!("({l:?} {r:?})")),
            ZipperTerm::L(t, n) => f.write_fmt(format_args!("L{t}. {n:?}")),
            ZipperTerm::Free => f.write_str("()"),
        }
    }
}

impl ZipperTerm {
    pub fn var(t: impl ToString) -> Self {
        ZipperTerm::Var(t.to_string())
    }

    pub fn a(lhs: Self, rhs: Self) -> Self {
        ZipperTerm::A(lhs.boxed(), rhs.boxed())
    }

    pub fn l(t: impl ToString, n: Self) -> Self {
        ZipperTerm::L(t.to_string(), n.boxed())
    }

    pub fn a_free() -> Self {
        Self::a(ZipperTerm::Free, ZipperTerm::Free)
    }

    pub fn l_free(t: impl ToString) -> Self {
        Self::l(t, ZipperTerm::Free)
    }

    pub fn left(self, t: Self) -> Self {
        let ZipperTerm::A(_, r) = self else {
            panic!("Term is not an A")
        };
        ZipperTerm::a(t, *r)
    }

    pub fn right(self, t: Self) -> Self {
        let ZipperTerm::A(l, _) = self else {
            panic!("Term is not an A")
        };
        ZipperTerm::a(*l, t)
    }

    pub fn next(self, t: Self) -> Self {
        let ZipperTerm::L(s, _) = self else {
            panic!("Term is not an L");
        };
        ZipperTerm::l(s, t)
    }
}

impl Foldable<String, String> for ZipperTerm {
    fn foldr(self, f: impl BifunT<String, String, String>, init: String) -> String {
        let f = f.to_bifun();
        match self {
            ZipperTerm::Var(t) => f(t, init),
            ZipperTerm::L(l, r) => f.clone()(l, r.foldr(f, init)),
            ZipperTerm::A(l, r) => l.foldr(f.clone(), r.foldr(f, init)),
            ZipperTerm::Free => unimplemented!(),
        }
    }

    fn foldl(self, f: impl BifunT<String, String, String>, init: String) -> String {
        let f = f.to_bifun();
        match self {
            ZipperTerm::Var(t) => f(init, t),
            ZipperTerm::L(l, r) => f.clone()(l, r.foldl(f, init)),
            ZipperTerm::A(l, r) => r.foldl(f.clone(), l.foldl(f, init)),
            ZipperTerm::Free => unimplemented!(),
        }
    }
}

impl Foldable1<String> for ZipperTerm {
    fn foldr1(self, f: impl BifunT<String, String, String>) -> String {
        todo!()
    }

    fn foldl1(self, f: impl BifunT<String, String, String>) -> String {
        todo!()
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

impl<M, N> Travel<Direction, M, N> for ZipperTerm
where
    M: ChainM<N, Pointed = (Option<ZipperTerm>, Direction)>,
    N: ChainM<N, Pointed = ZipperTerm> + ReturnM<Pointed = ZipperTerm>,
{
    fn travel(self, tf: impl FunctionT<Self, M>) -> N {
        let tf = tf.to_function();

        tf.clone()(self.clone()).chain_m(|(term_, dir)| {
            let t = term_.unwrap_or(self);

            match (dir, t) {
                (Direction::Up, t) | (_, t @ ZipperTerm::Var(_)) => ReturnM::return_m(t),
                (_, ZipperTerm::L(v, t1)) => Travel::<Direction, M, N>::travel(*t1, tf)
                    .chain_m(|t1| ReturnM::return_m(ZipperTerm::l(v, t1))),
                (Direction::Next, ZipperTerm::A(l, r)) => {
                    Travel::<Direction, M, N>::travel(*l, tf.clone()).chain_m(|l| {
                        Travel::<Direction, M, N>::travel(*r, tf)
                            .chain_m(|r| ReturnM::return_m(ZipperTerm::a(l, r)))
                    })
                }
                (Direction::DownLeft, ZipperTerm::A(l, r)) => {
                    Travel::<Direction, M, N>::travel(*l, tf)
                        .chain_m(|l| ReturnM::return_m(ZipperTerm::a(l, *r)))
                }
                (Direction::DownRight, ZipperTerm::A(l, r)) => {
                    Travel::<Direction, M, N>::travel(*r, tf)
                        .chain_m(|r| ReturnM::return_m(ZipperTerm::a(*l, r)))
                }
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

impl<M, N> Travel<Direction1, M, N> for ZipperTerm
where
    M: ChainM<N, Pointed = (Option<ZipperTerm>, Direction1)>,
    N: ChainM<N, Pointed = ZipperTerm> + ReturnM<Pointed = ZipperTerm>,
{
    fn travel(self, tf: impl FunctionT<Self, M>) -> N {
        let tf = tf.to_function();

        tf.clone()(self.clone()).chain_m(|(term_, dir)| {
            let t = term_.unwrap_or(self);

            match (dir, t) {
                (Direction1::Parent, t) => ReturnM::return_m(t),
                (dir, ZipperTerm::L(v, t1))
                    if dir == Direction1::FirstKid || dir == Direction1::RightKid =>
                {
                    t1.travel(tf.clone())
                        .chain_m(|t1| ZipperTerm::l(v, t1).travel(tf))
                }
                (Direction1::RightKid, ZipperTerm::A(l, r)) => r
                    .travel(tf.clone())
                    .chain_m(|r| ZipperTerm::a(*l, r).travel(tf)),
                (Direction1::FirstKid, ZipperTerm::A(l, r)) => l
                    .travel(tf.clone())
                    .chain_m(|l| ZipperTerm::a(l, *r).travel(tf)),
                _ => unimplemented!(),
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::base::{control::zipper::ZipTravel, data::functor::identity::Identity};

    use super::*;

    fn term() -> ZipperTerm {
        let x = ZipperTerm::var("x");
        let f = ZipperTerm::var("f");

        let term = ZipperTerm::l_free("f").next(
            ZipperTerm::l_free("x").next(
                ZipperTerm::a_free()
                    .left(
                        ZipperTerm::a_free().left(f.clone()).right(
                            ZipperTerm::l_free("f").next(
                                ZipperTerm::a_free().left(f.clone()).right(
                                    ZipperTerm::l_free("f")
                                        .next(ZipperTerm::l_free("x").next(x.clone())),
                                ),
                            ),
                        ),
                    )
                    .right(
                        ZipperTerm::a_free()
                            .left(
                                ZipperTerm::a_free().left(f.clone()).right(
                                    ZipperTerm::l_free("f")
                                        .next(ZipperTerm::l_free("x").next(x.clone())),
                                ),
                            )
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
                    ZipperTerm::A(ref l, _) if matches!(**l, ZipperTerm::Var(ref s) if s == "f") => {
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
                ZipperTerm::L(t, n)
                    if t == "x" && matches!(&*n, ZipperTerm::Var(s) if s == "x") =>
                {
                    println!("Replacing...");
                    (
                        Some(ZipperTerm::l("y", ZipperTerm::var("y"))),
                        Direction::Next,
                    )
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
        let term: Cont<Zipper<ZipperTerm, Direction>> =
            term().zip_travel().zip_all_the_way(Direction::Next, |t| {
                println!("Encountered: {t:#?}");
                None
            });

        let term = term.eval();
        println!("Term: {term:#?}");
    }

    #[test]
    fn test_zip_2() {
        let term: Cont<Zipper<ZipperTerm, Direction>> = term()
            .zip_travel()
            .zip_move(Direction::Next)
            .zip_move(Direction::Next)
            .zip_move(Direction::Next)
            .zip_move(Direction::DownRight);

        let term = term.fmap(
            Functor::replace
                .flip_clone()
                .curry_clone(ZipperTerm::a(ZipperTerm::var("x"), ZipperTerm::var("x"))),
        );

        let term: Cont<Zipper<ZipperTerm, Direction>> = term.zip_all_the_way(Direction::Up, |t| {
            println!("Zipping Up: {t:#?}");
            None
        });

        let term = term.eval_t().run();

        println!("Done: {term:#?}");
    }

    #[test]
    fn test_zip_3() {
        let term: Cont<Zipper<ZipperTerm, Direction1>> = ZipperTerm::l(
            "x",
            ZipperTerm::a(ZipperTerm::var("a"), ZipperTerm::var("b")),
        )
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
