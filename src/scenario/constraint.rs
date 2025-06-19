use crate::scenario::code::Code;

#[derive(Clone, Copy, Debug)]
pub struct Constraint<'a> {
    pub(crate) name: &'a str,
    pub(crate) verifier: fn(code: &Code) -> bool,
}
