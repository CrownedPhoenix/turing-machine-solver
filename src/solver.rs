use std::collections::HashSet;

use super::scenario::constraint::Constraint;
use crate::scenario::{card::Card, code::Code};
use itertools::Itertools;
use log::debug;

#[derive(Clone)]
pub struct Solution {
    code: Code,
    constraints: HashSet<Constraint>,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constraints = self.constraints.iter().map(ToString::to_string).join("; ");
        write!(f, "({}) with {constraints}", self.code)
    }
}

#[test]
fn solution_display() {
    let solution = Solution {
        code: Code { code: [1, 2, 3] },
        constraints: [
            Constraint {
                id: ConstraintID { card: 1, idx: 3 },
                name: "foo",
                verifier: |_| true,
            },
            Constraint {
                id: ConstraintID { card: 2, idx: 1 },
                name: "bar",
                verifier: |_| true,
            },
        ]
        .into(),
    };
    assert_eq!(solution.to_string(), "(123) with (1.3) foo; (2.1) bar")
}

#[derive(Clone, Copy)]
pub enum CardOrConstraint {
    Card(u8),
    CardConstraint(u8, u8),
}

#[derive(Clone)]
struct ConstraintGroup {
    constraints: Vec<Constraint>,
}

impl Into<ConstraintGroup> for Card {
    fn into(self) -> ConstraintGroup {
        ConstraintGroup {
            constraints: self.constraints,
        }
    }
}
impl Into<ConstraintGroup> for Constraint {
    fn into(self) -> ConstraintGroup {
        ConstraintGroup {
            constraints: vec![self],
        }
    }
}

pub fn turing_solve(
    a: Option<CardOrConstraint>,
    b: Option<CardOrConstraint>,
    c: Option<CardOrConstraint>,
    d: Option<CardOrConstraint>,
    e: Option<CardOrConstraint>,
    f: Option<CardOrConstraint>,
) -> Vec<Solution> {
    let constraint_groups: Vec<ConstraintGroup> = [a, b, c, d, e, f]
        .iter()
        .filter_map(|x| -> Option<&CardOrConstraint> { x.as_ref() })
        .filter_map(|x| -> Option<ConstraintGroup> {
            match *x {
                CardOrConstraint::Card(num) => Card::try_from(num).map(Card::into).ok(),
                CardOrConstraint::CardConstraint(card_num, constraint_num) => {
                    Card::try_from(card_num)
                        .ok()
                        .and_then(|card| card.constraints.get(constraint_num as usize).cloned())
                        .map(Constraint::into)
                }
            }
        })
        .collect();
    constraint_groups
        .iter()
        .map(|group| group.constraints.iter())
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
                        debug!(
                            "{} passed {}",
                            code,
                            constraint_combo.iter().map(ToString::to_string).join("; ")
                        );
                        Some(code)
                    } else {
                        debug!(
                            "{} failed {}",
                            code,
                            constraint_combo.iter().map(ToString::to_string).join("; ")
                        );
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
