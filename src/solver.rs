use super::scenario::constraint::Constraint;
use crate::scenario::code::Code;
use itertools::Itertools;

#[derive(Clone)]
pub struct Solution<'a> {
    code: Code,
    constraints: Vec<Constraint<'a>>,
}

impl<'a> std::fmt::Display for Solution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constraints = self.constraints.iter().map(ToString::to_string).join("; ");
        write!(f, "({}) with {constraints}", self.code)
    }
}

#[test]
fn solution_display() {
    let solution = Solution {
        code: Code { code: [1, 2, 3] },
        constraints: vec![
            Constraint {
                name: "foo",
                verifier: |_| true,
            },
            Constraint {
                name: "bar",
                verifier: |_| true,
            },
        ],
    };
    assert_eq!(solution.to_string(), "(123) with foo; bar")
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
