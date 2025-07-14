use std::collections::{HashMap, HashSet};

pub use super::scenario::constraint::Constraint;
use crate::scenario::{card::Card, code::Code};
use itertools::Itertools;
use log::debug;

#[derive(Clone)]
pub struct Solution {
    pub(crate) code: Code,
    pub(crate) constraints: HashSet<Constraint>,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constraints = self
            .constraints
            .iter()
            .sorted_by(|&a, &b| a.id.cmp(&b.id))
            .map(ToString::to_string)
            .join("; ");
        write!(f, "({}) with {constraints}", self.code)
    }
}

#[cfg(test)]
mod test {
    use crate::scenario::code::Code;
    use crate::scenario::constraint::{Constraint, ConstraintID};
    use crate::solver::Solution;
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
}

#[derive(Clone)]
struct ConstraintGroup {
    constraints: Vec<Constraint>,
}

impl Into<Vec<Constraint>> for Card {
    fn into(self) -> Vec<Constraint> {
        self.constraints
    }
}
impl Into<Vec<Constraint>> for Constraint {
    fn into(self) -> Vec<Constraint> {
        vec![self]
    }
}

pub fn constraints_for_card(card_num: u8) -> Option<Vec<Constraint>> {
    Card::try_from(card_num).ok().map(|c| c.constraints)
}

pub fn turing_solve(constraints: Vec<Constraint>) -> Vec<Solution> {
    let constraint_groups: Vec<ConstraintGroup> = constraints
        .iter()
        .fold(HashMap::<u8, Vec<Constraint>>::new(), |mut map, c| {
            map.entry(c.id.card).or_insert(vec![]).push(*c);
            map
        })
        .into_values()
        .map(|constraints| ConstraintGroup {
            constraints: constraints,
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
