/// Manual impl of common Haskell infix operators.
#[macro_export]
macro_rules ! infix {
    ($expr:expr ;. $($next:tt)*) => {
        $crate::prelude::Compose::compose_clone($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr ;<S> $($next:tt)*) => {
        $crate::prelude::Functor::fmap($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr ;<S $($next:tt)*) => {
        $crate::prelude::Functor::replace($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr ;S> $($next:tt)*) => {
        $crate::prelude::Functor::replace($crate::infix!{
            $($next)*
        }, $expr)
    };
    ($expr:expr ;<&> $($next:tt)*) => {
        $crate::prelude::Functor::fmap($crate::infix!{
            $($next)*
        }, $expr)
    };
    ($expr:expr ;<*> $($next:tt)*) => {
        $crate::prelude::AppA::app_a($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr ;<< $($next:tt)*) => {
        $crate::prelude::ThenM::then_m($crate::infix!{
            $($next)*
        }, $expr)
    };
    ($expr:expr ;>> $($next:tt)*) => {
        $crate::prelude::ThenM::then_m($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr ;=<< $($next:tt)*) => {
        $crate::prelude::ChainM::chain_m($crate::infix!{
            $($next)*
        }, $expr)
    };
    ($expr:expr ;>>= $($next:tt)*) => {
        $crate::prelude::ChainM::chain_m($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr ;<> $($next:tt)*) => {
        $crate::prelude::Semigroup::assoc_s($expr, $crate::infix!{
            $($next)*
        })
    };
    ($expr:expr $(;)?) => {
        $expr
    };
}
