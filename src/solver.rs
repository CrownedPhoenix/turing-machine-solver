use super::scenario::constraint::Constraint;
use crate::scenario::{card::Card, code::Code};
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
    let cards: Vec<Card> = [a, b, c, d, e, f]
        .iter()
        .filter_map(|&x| x)
        .filter_map(|x| Card::try_from(x).ok())
        .collect();
    cards
        .iter()
        .map(|card| card.constraints.iter())
        .multi_cartesian_product()
        .filter_map(|constraint_combo| {
            let possible_codes: Vec<Code> = (1u8..=5)
                .cartesian_product(1u8..=5)
                .cartesian_product(1u8..=5)
                .filter_map(|((blue, yellow), purple)| {
                    let code = Code {
                        code: [blue, yellow, purple],
                    };
                    if constraint_combo.iter().all(|c| (c.verifier)(&code)) {
                        Some(code)
                    } else {
                        None
                    }
                })
                .collect();
            if possible_codes.len() > 1 {
                return None;
            }
            if let Some(code) = possible_codes.first() {
                return Some(Solution {
                    code: *code,
                    constraints: constraint_combo.into_iter().copied().collect(),
                });
            }
            return None;
        })
        .collect()
}
