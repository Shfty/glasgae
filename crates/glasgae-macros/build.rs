use glasgae_kiss::*;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Export OUT_DIR to env!() macro
    println!("cargo:rustc-env=PROC_ARTIFACT_DIR={}", out_dir);

    register_operators(
        out_dir,
        [
            // 9
            Operator::new(
                Fixity::Right,
                9,
                "(.)",
                "todo!(\"Function Composition\")",
                false,
            ),
            // 6
            Operator::new(
                Fixity::Right,
                6,
                "<>",
                "glasgae::prelude::Semigroup::assoc_s",
                false,
            ),
            // 4
            Operator::new(
                Fixity::Left,
                4,
                "<$>",
                "glasgae::prelude::Functor::fmap",
                false,
            ),
            Operator::new(
                Fixity::Left,
                4,
                "<$",
                "glasgae::prelude::Functor::replace",
                false,
            ),
            Operator::new(
                Fixity::Left,
                4,
                "$>",
                "glasgae::prelude::Functor::replace",
                true,
            ),
            Operator::new(
                Fixity::Left,
                4,
                "<*>",
                "glasgae::prelude::AppA::app_a",
                false,
            ),
            Operator::new(
                Fixity::Left,
                4,
                "*>",
                "todo!(\"Assoc Discard Left\")",
                false,
            ),
            Operator::new(
                Fixity::Left,
                4,
                "<*",
                "todo!(\"Assoc Discard Right\")",
                false,
            ),
            // 1
            Operator::new(
                Fixity::Left,
                1,
                "<&>",
                "glasgae::prelude::Functor::fmap",
                false,
            ),
            Operator::new(
                Fixity::Left,
                1,
                ">>=",
                "glasgae::prelude::ChainM::chain_m",
                false,
            ),
            Operator::new(
                Fixity::Left,
                1,
                ">>",
                "glasgae::prelude::ThenM::then_m",
                false,
            ),
            Operator::new(
                Fixity::Right,
                1,
                "=<<",
                "glasgae::prelude::ChainM::chain_m",
                true,
            ),
            Operator::new(
                Fixity::Right,
                1,
                "<<",
                "glasgae::prelude::ThenM::then_m",
                true,
            ),
            // 0
            Operator::new(
                Fixity::Right,
                0,
                "$",
                "todo!(\"TODO: Infix Application\")",
                false,
            ),
        ],
    )
    .expect("Operator registration failed");
}
