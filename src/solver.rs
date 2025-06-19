use super::scenario::constraint::Constraint;
use crate::scenario::code::Code;

#[derive(Clone)]
pub struct Solution<'a> {
    code: Code,
    constraints: Vec<Constraint<'a>>,
}

pub fn turing_solve<'a>(
    a: Option<u8>,
    b: Option<u8>,
    c: Option<u8>,
    d: Option<u8>,
    e: Option<u8>,
    f: Option<u8>,
) -> Vec<Solution<'a>> {
    vec![]
}
