use std::{error::Error, path::Path};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Fixity {
    Left,
    Right,
    None,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Operator {
    op: String,
    func: String,
    fixity: Fixity,
    prec: isize,
    flip: bool,
}

impl Operator {
    pub fn new(assoc: Fixity, prec: isize, op: impl ToString, func: impl ToString, flip: bool) -> Self {
        let op = op.to_string();
        let func = func.to_string();
        Operator {
            op,
            func,
            fixity: assoc,
            prec,
            flip
        }
    }

    pub fn op(&self) -> &str {
        &self.op
    }

    pub fn fixity(&self) -> Fixity {
        self.fixity
    }

    pub fn prec(&self) -> isize {
        self.prec
    }

    pub fn func(&self) -> &str {
        &self.func
    }

    pub fn flip(&self) -> bool {
        self.flip
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Operators(pub Vec<(String, Operator)>);

pub fn register_operators(
    out_dir: impl AsRef<Path>,
    input: impl IntoIterator<Item = Operator>,
) -> Result<(), Box<dyn Error>> {
    let out_dir = out_dir.as_ref();
    let input: Vec<_> = input.into_iter().collect();

    let path = out_dir.join("operators.ron");

    let mut ops = read_operators(out_dir)?;
    for op in input.iter() {
        let o = ops.0;
        ops = Operators(
            o.into_iter()
                .filter(|(k, _)| {
                    if k == op.op() {
                        println!("Overriding operator {k} with new implementation");
                        false
                    } else {
                        true
                    }
                })
                .collect(),
        );
    }
    ops.0
        .extend(input.into_iter().map(|op| (op.op().to_string(), op)));
    std::fs::write(path, ron::to_string(&ops)?)?;

    Ok(())
}

pub fn read_operators(out_dir: impl AsRef<Path>) -> Result<Operators, Box<dyn Error>> {
    let path = out_dir.as_ref().join("operators.ron");

    if path.exists() {
        let str = std::fs::read_to_string(path)?;
        let ops = ron::from_str::<Operators>(&str)?;
        Ok(ops)
    } else {
        Ok(Operators(Default::default()))
    }
}
