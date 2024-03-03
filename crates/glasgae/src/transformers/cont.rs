//! # Continuation monads.
//!
//! Delimited continuation operators are taken from
//! Kenichi Asai and Oleg Kiselyov's tutorial at CW 2011,
//! "Introduction to programming with shift and reset"
//! (<http://okmij.org/ftp/continuations/#tutorial>).

use crate::{
    base::data::{function::bifunction::BifunT, functor::identity::Identity},
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

impl<R, A> Cont<R, A> {
    /// Construct a continuation-passing computation from a function. (The inverse of run)
    pub fn new(f: impl FunctionT<Function<A, R>, R> + Clone) -> Cont<R, A> {
        Cont::new_t(|c| Identity(f((|t| c(t).run()).boxed())))
    }

    /// The result of running a CPS computation with a given final continuation.
    /// (The inverse of cont)
    ///
    /// self: Continuation compuation
    ///
    /// f: The final continuation, which produces the final result (often identity)
    pub fn run(self, f: impl FunctionT<A, Identity<R>> + Clone) -> R {
        self.run_t(f).run()
    }

    /// Apply a function to transform the result of a continuation-passing computation.
    pub fn map(self, f: impl FunctionT<R, R> + Clone) -> Self
    where
        R: Clone,
        A: Clone,
    {
        self.map_t(|t| Identity(f(t.run())))
    }

    /// Apply a function to transform the continuation passed to a CPS computation.
    pub fn with<B>(self, f: impl FunctionT<Function<B, R>, Function<A, R>> + Clone) -> Cont<R, B>
    where
        R: Clone,
        A: Clone,
    {
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

impl<R> Cont<R, R> {
    /// The result of running a CPS computation with the identity as the final continuation.
    pub fn eval(self) -> R {
        self.eval_t().run()
    }

    /// reset m delimits the continuation of any shift inside m.
    pub fn reset(self) -> Cont<R, R>
    where
        R: Clone,
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
    MR: 'static,
    MA: 'static + Pointed;

impl<MR, MA> ContT<MR, MA>
where
    MA: Pointed,
{
    pub fn new_t(t: impl FunctionT<Function<MA::Pointed, MR>, MR> + Clone) -> Self {
        ContT(t.boxed())
    }

    pub fn run_t(self, f: impl FunctionT<MA::Pointed, MR> + Clone) -> MR {
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
    pub fn map_t(self, f: impl FunctionT<MR, MR> + Clone) -> Self
    where
        MR: Clone,
        MA: Clone,
        MA::Pointed: Clone,
    {
        ContT::new_t(|t| f(self.run_t(t)))
    }
    ///
    /// Apply a function to transform the continuation passed to a CPS computation.
    ///
    /// runContT (withContT f m) = runContT m . f
    pub fn with_t<N>(
        self,
        f: impl FunctionT<Function<N::Pointed, MR>, Function<MA::Pointed, MR>> + Clone,
    ) -> ContT<MR, N>
    where
        MR: Clone,
        MA: Clone,
        MA::Pointed: Clone,
        N: Pointed,
    {
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
    pub fn call_cc<MB>(
        f: impl FunctionT<Function<MA::Pointed, ContT<MR, MB>>, Self> + Clone,
    ) -> Self
    where
        MA: Clone + Pointed,
        MA::Pointed: Clone,
        MB: 'static + Clone + Pointed,
    {
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
    pub fn shift_t(f: impl FunctionT<Function<MA::Pointed, MR>, ContT<MR, MR>> + Clone) -> Self
    where
        MA: Pointed,
        MR: 'static + ReturnM,
    {
        ContT::new_t(|t| f(t).eval_t())
    }

    /// resetT m delimits the continuation of any shiftT inside m.
    ///
    /// resetT (lift m) = lift m
    pub fn reset_t<MR_>(self) -> ContT<MR_, MA>
    where
        MR: Clone + ChainM<MR_> + ReturnM,
        MA: Pointed<Pointed = MR::Pointed>,
    {
        ContT::lift(self.eval_t())
    }

    /// liftLocal ask local yields a local function for ContT r m.
    pub fn lift_local<MR_>(
        self,
        ask: MR_,
        local: impl BifunT<MR, Function<MR_::Pointed, MR_::Pointed>, MR> + Clone,
        f: impl FunctionT<MR_::Pointed, MR_::Pointed> + Clone,
    ) -> Self
    where
        MR: Clone,
        MA: Clone,
        MA::Pointed: Clone,
        MR_: 'static + Clone + ChainM<MR>,
        MR_::Pointed: 'static + Clone,
    {
        ContT::new_t(|c| {
            ask.chain_m(|r| {
                local.clone()(self.run_t(|x| local(c(x), r#const(r).boxed())), f.boxed())
            })
        })
    }
}

impl<MR, MA> Pointed for ContT<MR, MA>
where
    MA: Pointed,
{
    type Pointed = MA::Pointed;
}

impl<MR, MA, T> WithPointed<T> for ContT<MR, MA>
where
    MA: WithPointed<T>,
    MA::WithPointed: 'static,
{
    type WithPointed = ContT<MR, MA::WithPointed>;
}

impl<MR, MA, A> Functor<A> for ContT<MR, MA>
where
    MA: Clone + Pointed<Pointed = A> + WithPointed<A>,
    MA::Pointed: 'static + Clone,
    MR: Clone,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, A> + Clone) -> Self::WithPointed {
        let m = self;
        ContT::new_t(|c| m.run_t(|t| c(f(t))))
    }
}

impl<MR, MA> PureA for ContT<MR, MA>
where
    MA: Pointed,
    MA::Pointed: Clone,
{
    fn pure_a(t: Self::Pointed) -> Self {
        ContT::new_t(|f| f(t))
    }
}

impl<MR, MF, MA, MB> AppA<ContT<MR, MA>, ContT<MR, MB>> for ContT<MR, MF>
where
    MF: Clone + Pointed,
    MF::Pointed: Clone + FunctionT<MA::Pointed, MB::Pointed>,
    MA: Clone + Pointed,
    MA::Pointed: Clone,
    MB: Pointed,
    MR: Clone,
{
    fn app_a(self, v: ContT<MR, MA>) -> ContT<MR, MB> {
        let f = self;
        ContT::new_t(|c| f.run_t(|g| v.run_t(|t| c(g(t)))))
    }
}

impl<MR, MA> ReturnM for ContT<MR, MA>
where
    MA: Pointed,
    MA::Pointed: Clone,
{
}

impl<MR, MA, MN> ChainM<ContT<MR, MN>> for ContT<MR, MA>
where
    MA: Clone + Pointed,
    MA::Pointed: Clone,
    MN: Pointed,
    MN::Pointed: Clone,
    MR: Clone,
{
    fn chain_m(self, k: impl FunctionT<Self::Pointed, ContT<MR, MN>> + Clone) -> ContT<MR, MN> {
        let m = self;
        ContT::new_t(|c| m.run_t(|x| k(x).run_t(c)))
    }
}

impl<MI, MO, MR> MonadTrans<MI> for ContT<MR, MO>
where
    MO: 'static + Pointed<Pointed = MI::Pointed>,
    MI: 'static + Clone + ChainM<MR>,
{
    fn lift(m: MI) -> ContT<MR, MO> {
        ContT::new_t(|n| m.chain_m(n))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        transformers::cont::Cont,
        prelude::{r#const, Boxed, ChainM, Function, ReturnM},
    };

    pub fn cont_add<R, T>(u: T) -> Function<T, Cont<R, T>>
    where
        T: Clone + std::ops::Add<T, Output = T> + 'static,
    {
        (|t| ReturnM::return_m(t + u)).boxed()
    }

    pub fn cont_mul<R, T>(u: T) -> Function<T, Cont<R, T>>
    where
        T: Clone + std::ops::Mul<T, Output = T> + 'static,
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
