use std::fmt::Display;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Indent<T>(T, usize);

impl<T> Display for Indent<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Indent(t, depth) = self;
        f.write_fmt(format_args!("{}{}", "  ".repeat(*depth), t))
    }
}

impl<T> Indent<T> {
    pub fn new(t: T, depth: usize) -> Self {
        Indent(t, depth)
    }
}
