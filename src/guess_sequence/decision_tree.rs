use std::collections::HashMap;

use itertools::Itertools;
use log::debug;

use crate::guess_sequence::{Branch, Decision};

pub struct DecisionTree<'a> {
    pub roots: Vec<Decision<'a>>,
    pub decisions: HashMap<Decision<'a>, Branch<'a>>,
}

impl<'a> std::fmt::Display for DecisionTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let roots = self
            .roots
            .iter()
            .map(|d| convert_to_decision_tree_string(d, &self.decisions, 0))
            .join("");
        write!(f, "{}", roots)
    }
}

fn convert_to_decision_tree_string(
    decision: &Decision,
    map: &HashMap<Decision, Branch>,
    indent: usize,
) -> String {
    let children = map
        .get(decision)
        .map(|branch| {
            [&branch.verified, &branch.unverified]
                .into_iter()
                .flatten()
                .map(|d| convert_to_decision_tree_string(d, map, indent + 1))
                .join("")
        })
        .unwrap_or(String::new());
    if children.is_empty() && decision.solution == None {
        debug!("Problem w: {:?}", decision)
    }
    format!(
        "{}{} {}\n{}",
        "\t".repeat(indent),
        decision.current_guess,
        if let Some(code) = decision.solution {
            format!("--- {}", code)
        } else {
            String::new()
        },
        children,
    )
}
