//! Declaration of the ReaderT monad transformer, which adds a static environment to a given monad.
//!
//! If the computation is to modify the stored information, use Control.Monad.Trans.State instead.

use crate::{
    base::{control::monad::io::MonadIO, data::functor::identity::Identity},
    derive_pointed_via, derive_with_pointed_via,
    prelude::*,
};

use super::class::MonadTrans;

/// The parameterizable reader monad.
///
/// Computations are functions of a shared environment.
///
/// The return function ignores the environment, while >>= passes the inherited environment to both subcomputations.
pub type Reader<R, A> = ReaderT<R, Identity<A>>;

impl<R, A> Reader<R, A>
where
    R: Term,
    A: Term,
{
    /// Runs a Reader and extracts the final value from it. (The inverse of new.)
    ///
    /// Self: A Reader to run
    /// r: An initial environment
    pub fn run(self, r: R) -> A {
        self.run_t(r).run()
    }

    /// Transform the value returned by a Reader.
    ///
    /// runReader (mapReader f m) = f . runReader m
    pub fn map<B>(self, f: impl FunctionT<A, B>) -> Reader<R, B>
    where
        B: Term,
    {
        let f = f.to_function();
        self.map_t(|t| Identity(f(t.run())))
    }

    /// Execute a computation in a modified environment (a specialization of withReaderT).
    ///
    /// runReader (withReader f m) = runReader m . f
    ///
    /// Self: Computation to run in the modified environment
    /// f: The function to modify the environment
    pub fn with<B>(self, f: impl FunctionT<B, R>) -> Reader<B, A>
    where
        B: Term,
    {
        let f = f.to_function();
        self.with_t(f)
    }
}

/// The reader monad transformer, which adds a read-only environment to the given monad.
///
/// The return function ignores the environment, while >>= passes the inherited environment to both subcomputations.
#[derive(Clone)]
pub struct ReaderT<R, M>(Function<R, M>)
where
    R: Term,
    M: Term;

impl<R, MA> ReaderT<R, MA>
where
    R: Term,
    MA: Pointed,
{
    /// Constructor for computations in the reader monad (equivalent to asks).
    pub fn new(f: impl FunctionT<R, MA::Pointed>) -> Self
    where
        MA: ReturnM,
    {
        let f = f.to_function();
        ReaderT::new_t(|t| ReturnM::return_m(f(t)))
    }

    pub fn new_t(f: impl FunctionT<R, MA>) -> Self {
        ReaderT(f.boxed())
    }

    pub fn run_t(self, r: R) -> MA {
        self.0(r)
    }

    /// Transform the computation inside a ReaderT.
    ///
    /// runReaderT (mapReaderT f m) = f . runReaderT m
    pub fn map_t<M2>(self, f: impl FunctionT<MA, M2>) -> ReaderT<R, M2>
    where
        M2: Pointed,
    {
        let f = f.to_function();
        ReaderT::new_t(|t| f(self.run_t(t)))
    }

    /// Execute a computation in a modified environment (a more general version of local).
    ///
    /// runReaderT (withReaderT f m) = runReaderT m . f
    ///
    /// Self: Computation to run in the modified environment.
    /// f: The function to modify the environment.
    pub fn with_t<B>(self, f: impl FunctionT<B, R>) -> ReaderT<B, MA>
    where
        B: Term,
    {
        let f = f.to_function();
        ReaderT::new_t(move |t| self.run_t(f(t)))
    }

    pub fn lift_t(m: MA) -> Self {
        ReaderT::new_t(r#const(m))
    }

    /// Fetch the value of the environment.
    pub fn ask() -> ReaderT<R, MA>
    where
        MA: ReturnM<Pointed = R>,
    {
        ReaderT::new_t(ReturnM::return_m)
    }

    /// Retrieve a function of the current environment.
    ///
    /// asks f = liftM f ask
    ///
    /// f: The selector function to apply to the environment.
    pub fn asks<A>(f: impl FunctionT<R, A>) -> Self
    where
        MA: ReturnM<Pointed = A>,
        A: Term,
    {
        let f = f.to_function();
        ReaderT::new_t(|t| ReturnM::return_m(f(t)))
    }

    /// Execute a computation in a modified environment (a specialization of withReaderT).
    ///
    /// runReaderT (local f m) = runReaderT m . f
    ///
    /// Self: Computation to run in the modified environment.
    /// f: The function to modify the environment.
    pub fn local(self, f: impl FunctionT<R, R>) -> Self
    where
        R: Clone,
        MA: Clone,
    {
        self.with_t(f)
    }
}

derive_pointed_via!(ReaderT<R, (M)>);
derive_with_pointed_via!(ReaderT<R, (M)>);

impl<R, MA, B> Functor<B> for ReaderT<R, MA>
where
    B: Term,
    MA: Functor<B>,
    R: Term,
{
    type Mapped = ReaderT<R, MA::Mapped>;

    fn fmap(self, f: impl FunctionT<MA::Pointed, B>) -> ReaderT<R, MA::Mapped> {
        let f = f.to_function();
        self.map_t(|y| y.fmap(f))
    }
}

impl<R, M> PureA for ReaderT<R, M>
where
    R: Term,
    M: PureA,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Self::lift_t(PureA::pure_a(t))
    }
}

impl<R, MF, F, MA, A, MB, B> AppA<A, B> for ReaderT<R, MF>
where
    R: Term,
    MF: Pointed<Pointed = F>
        + WithPointed<A, WithPointed = MA>
        + WithPointed<B, WithPointed = MB>
        + Applicative<A, B, WithA = MA, WithB = MB>,
    MA: WithPointed<F, Pointed = A, WithPointed = MF>,
    MB: WithPointed<F, Pointed = B, WithPointed = MF>,
    F: Term,
    A: Term,
    B: Term,
{
    type WithA = ReaderT<R, MA>;
    type WithB = ReaderT<R, MB>;

    fn app_a(self, v: WithPointedT<Self, A>) -> WithPointedT<Self, B> {
        let f = self;
        ReaderT::new_t(|r: R| f.run_t(r.clone()).app_a(v.run_t(r)))
    }
}

impl<R, M> ReturnM for ReaderT<R, M>
where
    R: Term,
    M: PureA,
{
}

impl<R, MA, A, MB, B> ChainM<B> for ReaderT<R, MA>
where
    R: Term,
    MA: Monad<B, Pointed = A, Chained = MB>,
    MB: Monad<A, Pointed = B, Chained = MA>,
    A: Term,
    B: Term,
{
    type Chained = ReaderT<R, MB>;

    fn chain_m(self, k: impl FunctionT<Self::Pointed, ReaderT<R, MB>>) -> ReaderT<R, MB> {
        let m = self;
        let k = k.to_function();
        ReaderT::new_t(|r: R| m.run_t(r.clone()).chain_m(|a| k(a).run_t(r)))
    }
}

impl<MO, R> MonadTrans<MO> for ReaderT<R, MO>
where
    R: Term,
    MO: Pointed,
{
    fn lift(m: MO) -> ReaderT<R, MO> {
        ReaderT::lift_t(m)
    }
}

impl<MA, R, A> MonadIO<A> for ReaderT<R, MA>
where
    R: Term,
    MA: MonadIO<A, Pointed = A>,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::{ChainM, ReturnM};

    use super::Reader;

    type Email = String;
    type Html = String;

    fn div(children: Vec<Html>) -> Html {
        format!("<div>{}</div>", children.concat())
    }

    fn h1(children: Vec<Html>) -> Html {
        format!("<h1>{}</h1>", children.concat())
    }

    fn p(children: Vec<Html>) -> Html {
        format!("<p>{}</p>", children.concat())
    }

    fn view(email: Email) -> Html {
        div(vec![page(email)])
    }

    fn page(email: Email) -> Html {
        div(vec![top_nav(), content(email)])
    }

    fn top_nav() -> Html {
        div(vec![h1(vec![format!("OurSite.com")])])
    }

    fn content(email: Email) -> Html {
        div(vec![h1(vec![
            format!("Custom Content for {email}"),
            left(),
            right(email),
        ])])
    }

    fn left() -> Html {
        div(vec![p(vec![format!("this is the left side")])])
    }

    fn right(email: Email) -> Html {
        div(vec![article(email)])
    }

    fn article(email: Email) -> Html {
        div(vec![p(vec![format!("this is an article"), widget(email)])])
    }

    fn widget(email: Email) -> Html {
        div(vec![p(vec![format!(
            "Hey {email}, we've got a great offer for you!"
        )])])
    }

    #[test]
    fn test_html() {
        let email = "test@foobar.com";
        let out = view(email.to_string());
        println!("{out}");
    }

    fn view_r() -> Reader<Email, Html> {
        page_r().chain_m(|page_| ReturnM::return_m(div(vec![page_])))
    }

    fn page_r() -> Reader<Email, Html> {
        content_r().chain_m(|content_| ReturnM::return_m(div(vec![top_nav(), content_])))
    }

    fn content_r() -> Reader<Email, Html> {
        Reader::ask().chain_m(|email| {
            right_r().chain_m(move |right_| {
                ReturnM::return_m(div(vec![h1(vec![
                    format!("Custom content for {email}"),
                    left(),
                    right_,
                ])]))
            })
        })
    }

    fn right_r() -> Reader<Email, Html> {
        article_r().chain_m(|article_| ReturnM::return_m(div(vec![article_])))
    }

    fn article_r() -> Reader<Email, Html> {
        widget_r().chain_m(|widget_| {
            ReturnM::return_m(div(vec![p(vec![format!("this is an article")]), widget_]))
        })
    }

    fn widget_r() -> Reader<Email, Html> {
        Reader::ask().chain_m(|email| {
            ReturnM::return_m(div(vec![p(vec![format!(
                "Hey {email}, we've got a great offer for you!"
            )])]))
        })
    }

    #[test]
    fn test_html_reader() {
        let email = "test@foobar.com";
        let out = view_r().run(email.to_string());
        println!("{out:#?}");
    }
}
