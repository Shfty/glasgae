//! # Continuation monads.
//!
//! Delimited continuation operators are taken from
//! Kenichi Asai and Oleg Kiselyov's tutorial at CW 2011,
//! "Introduction to programming with shift and reset"
//! (<http://okmij.org/ftp/continuations/#tutorial>).

use std::panic::UnwindSafe;

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::{function::bifunction::BifunT, functor::identity::Identity, term::Term},
    },
    prelude::*,
};

use super::class::MonadTrans;

/// Continuation monad. Cont r a is a CPS ("continuation-passing style") computation that produces an intermediate result of type a within a CPS computation whose final result type is r.
///
/// The return function simply creates a continuation which passes the value on.
///
/// The >>= operator adds the bound function into the continuation chain.
pub type Cont<R, A = R> = ContT<Identity<R>, Identity<A>>;

pub type CallCC<MA, MB, A> = Function<Function<Function<A, MB>, MA>, MA>;

impl<R, A> Cont<R, A>
where
    R: Term,
    A: Term,
    Function<A, R>: Term,
{
    /// Construct a continuation-passing computation from a function. (The inverse of run)
    pub fn new(f: impl FunctionT<Function<A, R>, R>) -> Cont<R, A> {
        let f = f.to_function();
        Cont::new_t(|c| Identity(f((|t| c(t).run()).boxed())))
    }

    /// The result of running a CPS computation with a given final continuation.
    /// (The inverse of cont)
    ///
    /// self: Continuation compuation
    ///
    /// f: The final continuation, which produces the final result (often identity)
    pub fn run(self, f: impl FunctionT<A, Identity<R>>) -> R {
        self.run_t(f).run()
    }

    /// Apply a function to transform the result of a continuation-passing computation.
    pub fn map(self, f: impl FunctionT<R, R>) -> Self {
        let f = f.to_function();
        self.map_t(|t| Identity(f(t.run())))
    }

    /// Apply a function to transform the continuation passed to a CPS computation.
    pub fn with<B>(self, f: impl FunctionT<Function<B, R>, Function<A, R>>) -> Cont<R, B>
    where
        B: Term,
    {
        let f = f.to_function();
        self.with_t(|x| {
            f(x.compose_clone(Identity::run).boxed())
                .compose_clone(Identity)
                .boxed()
        })
    }

    /// shift f captures the continuation up to the nearest enclosing reset and passes it to f:
    ///
    /// reset (shift f >>= k) = reset (f (evalCont . k))
    pub fn shift(f: Function<Function<A, R>, Cont<R, R>>) -> Self {
        Self::shift_t(|t| f(t.compose_clone(Identity::run).boxed()))
    }
}

impl<R> Cont<R, R>
where
    R: Term,
{
    /// The result of running a CPS computation with the identity as the final continuation.
    pub fn eval(self) -> R {
        self.eval_t().run()
    }

    /// reset m delimits the continuation of any shift inside m.
    pub fn reset(self) -> Cont<R, R>
    where
        R: Clone + UnwindSafe,
    {
        self.reset_t()
    }
}

/// The continuation monad transformer. Can be used to add continuation handling to any type constructor: the Monad instance and most of the operations do not require m to be a monad.
///
/// ContT is not a functor on the category of monads, and many operations cannot be lifted through it.
#[derive(Clone)]
pub struct ContT<MR, MA>(Function<Function<MA::Pointed, MR>, MR>)
where
    MR: Pointed,
    MA: Pointed;

impl<MR, MA> ContT<MR, MA>
where
    MR: Pointed,
    MA: Pointed,
{
    pub fn new_t(t: impl FunctionT<Function<MA::Pointed, MR>, MR>) -> Self {
        ContT(t.boxed())
    }

    pub fn run_t(self, f: impl FunctionT<MA::Pointed, MR>) -> MR {
        self.0(f.boxed())
    }

    /// The result of running a CPS computation with return as the final continuation.
    ///
    /// evalContT (lift m) = m
    pub fn eval_t(self) -> MR
    where
        MR: ReturnM<Pointed = MA::Pointed>,
    {
        self.run_t(ReturnM::return_m)
    }

    /// Apply a function to transform the result of a continuation-passing computation.
    /// This has a more restricted type than the map operations for other monad transformers,
    /// because ContT does not define a functor in the category of monads.
    ///
    /// runContT (mapContT f m) = f . runContT m
    pub fn map_t(self, f: impl FunctionT<MR, MR>) -> Self {
        let f = f.to_function();
        ContT::new_t(|t| f(self.run_t(t)))
    }
    ///
    /// Apply a function to transform the continuation passed to a CPS computation.
    ///
    /// runContT (withContT f m) = runContT m . f
    pub fn with_t<N>(
        self,
        f: impl FunctionT<Function<N::Pointed, MR>, Function<MA::Pointed, MR>>,
    ) -> ContT<MR, N>
    where
        N: Pointed,
    {
        let f = f.to_function();
        ContT::new_t(|t| self.run_t(f(t)))
    }

    /// callCC (call-with-current-continuation) calls its argument function, passing it the current continuation.
    /// It provides an escape continuation mechanism for use with continuation monads.
    /// Escape continuations one allow to abort the current computation and return a value immediately.
    /// They achieve a similar effect to throwE and catchE within an ExceptT monad.
    /// The advantage of this function over calling return is that it makes the continuation explicit, allowing more flexibility and better control.
    ///
    /// The standard idiom used with callCC is to provide a lambda-expression to name the continuation.
    /// Then calling the named continuation anywhere within its scope will escape from the computation,
    /// even if it is many layers deep within nested computations.
    pub fn call_cc<MB>(f: impl FunctionT<Function<MA::Pointed, ContT<MR, MB>>, Self>) -> Self
    where
        MR: Pointed,
        MB: Pointed,
    {
        let f = f.to_function();
        ContT::new_t(|c| {
            f({
                let c = c.clone();
                move |x| ContT::new_t(move |_| c(x))
            }
            .boxed())
            .run_t(c)
        })
    }

    /// shiftT f captures the continuation up to the nearest enclosing resetT and passes it to f:
    ///
    /// resetT (shiftT f >>= k) = resetT (f (evalContT . k))
    pub fn shift_t(f: impl FunctionT<Function<MA::Pointed, MR>, ContT<MR, MR>>) -> Self
    where
        MA: Pointed,
        MR: 'static + ReturnM,
    {
        let f = f.to_function();
        ContT::new_t(|t| f(t).eval_t())
    }

    /// resetT m delimits the continuation of any shiftT inside m.
    ///
    /// resetT (lift m) = lift m
    pub fn reset_t<MR_>(self) -> ContT<MR_, MA>
    where
        MR_: Pointed,
        MR: ChainM<MR_> + ReturnM,
        MA: Pointed<Pointed = MR::Pointed>,
    {
        ContT::lift(self.eval_t())
    }

    /// liftLocal ask local yields a local function for ContT r m.
    pub fn lift_local<MR_>(
        self,
        ask: MR_,
        local: impl BifunT<MR, Function<MR_::Pointed, MR_::Pointed>, MR>,
        f: impl FunctionT<MR_::Pointed, MR_::Pointed>,
    ) -> Self
    where
        MR: Pointed,
        MA: Term,
        MR_: ChainM<MR>,
    {
        let local = local.to_bifun();
        let f = f.to_function();
        ContT::new_t(|c| {
            ask.chain_m(|r| local.clone()(self.run_t(|x| local(c(x), r#const(r).boxed())), f))
        })
    }
}

impl<MR, MA> Pointed for ContT<MR, MA>
where
    MR: Pointed,
    MA: Pointed,
{
    type Pointed = MA::Pointed;
}

impl<MR, MA, T> WithPointed<T> for ContT<MR, MA>
where
    MR: Pointed,
    MA: Pointed + WithPointed<T>,
    T: Term,
{
    type WithPointed = ContT<MR, MA::WithPointed>;
}

impl<MR, MA, A> Functor<A> for ContT<MR, MA>
where
    MA: Pointed<Pointed = A> + WithPointed<A>,
    MR: Pointed,
    A: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, A>) -> Self::WithPointed {
        let m = self;
        let f = f.to_function();
        ContT::new_t(|c| m.run_t(|t| c(f(t))))
    }
}

impl<MR, MA> PureA for ContT<MR, MA>
where
    MR: Pointed,
    MA: Pointed,
{
    fn pure_a(t: Self::Pointed) -> Self {
        ContT::new_t(|f| f(t))
    }
}

impl<MR, MF, MA, MB> AppA<ContT<MR, MA>, ContT<MR, MB>> for ContT<MR, MF>
where
    MF: Pointed,
    MF::Pointed: FunctionT<MA::Pointed, MB::Pointed>,
    MA: Pointed,
    MB: Pointed,
    MR: Pointed,
{
    fn app_a(self, v: ContT<MR, MA>) -> ContT<MR, MB> {
        let f = self;
        ContT::new_t(|c| f.run_t(|g| v.run_t(|t| c(g(t)))))
    }
}

impl<MR, MA> ReturnM for ContT<MR, MA>
where
    MR: Pointed,
    MA: Pointed,
{
}

impl<MR, MA, MN> ChainM<ContT<MR, MN>> for ContT<MR, MA>
where
    MA: Pointed,
    MN: Pointed,
    MR: Pointed,
{
    fn chain_m(self, k: impl FunctionT<Self::Pointed, ContT<MR, MN>>) -> ContT<MR, MN> {
        let m = self;
        let k = k.to_function();
        ContT::new_t(|c| m.run_t(|x| k(x).run_t(c)))
    }
}

impl<MI, MO, MR> MonadTrans<MI> for ContT<MR, MO>
where
    MO: Pointed<Pointed = MI::Pointed>,
    MI: ChainM<MR>,
    MR: Pointed,
{
    fn lift(m: MI) -> ContT<MR, MO> {
        ContT::new_t(|n| m.chain_m(n))
    }
}

impl<MR, MA, A> MonadIO<A> for ContT<MR, MA>
where
    Self: MonadTrans<IO<A>>,
    MR: Pointed,
    MA: Pointed<Pointed = A>,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        base::data::term::Term,
        prelude::{r#const, Boxed, ChainM, Function, ReturnM},
        transformers::cont::Cont,
    };

    pub fn cont_add<R, T>(u: T) -> Function<T, Cont<R, T>>
    where
        R: Term,
        T: Term + std::ops::Add<T, Output = T>,
    {
        (|t| ReturnM::return_m(t + u)).boxed()
    }

    pub fn cont_mul<R, T>(u: T) -> Function<T, Cont<R, T>>
    where
        R: Term,
        T: Term + std::ops::Mul<T, Output = T>,
    {
        (|t| ReturnM::return_m(t * u)).boxed()
    }

    #[test]
    fn test_cont_1() {
        println!("Building continuation...");

        let ex1 =
            Cont::shift((|k: Function<usize, usize>| Cont::new(r#const(k.clone()(k(10))))).boxed())
                .chain_m(cont_mul(2))
                .reset()
                .chain_m(cont_add(1));

        println!("Running...");
        let res = ex1.eval_t();
        println!("Result: {res:#?}");
    }
}
