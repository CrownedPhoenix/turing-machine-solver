use itertools::Itertools;
use log::{Level, debug, error, log_enabled};
use std::collections::{HashMap, HashSet};

use crate::{
    guess_sequence::{Branch, Decision, DecisionTree, Guess},
    scenario::constraint::Constraint,
    solver::Solution,
};

/// Determine the best possible guess sequences that converge to a solution.
pub fn guess_sequence(solutions: &Vec<Solution>) -> DecisionTree {
    let relevant_constraints: HashSet<&Constraint> = solutions.iter().fold(
        HashSet::<&Constraint>::new(),
        |mut constraints, solution| {
            constraints.extend(solution.constraints.iter());
            constraints
        },
    );

    let mut decisions: HashMap<Decision, Branch> = HashMap::new();
    let first_best_guess: &Constraint = best_guess(
        &relevant_constraints,
        &solutions.iter().map(|s| s).collect_vec(),
    )
    .unwrap();

    let available_solutions: Vec<&Solution> = solutions.iter().map(|s| s).collect_vec();

    let roots = vec![
        Decision::prepare(
            vec![],
            Guess {
                constraint: first_best_guess,
                verified: true,
            },
            &available_solutions,
        ),
        Decision::prepare(
            vec![],
            Guess {
                constraint: first_best_guess,
                verified: false,
            },
            &available_solutions,
        ),
    ];
    let mut unbranched_decisions: Vec<Decision> = roots.clone();

    while let Some(decision) = unbranched_decisions.pop() {
        if decision.solution != None {
            continue;
        }
        let remaining_solutions = &shrink(&available_solutions, &decision.guesses);
        debug!("For guess sequence: {:?}", decision.guesses);
        if log_enabled!(Level::Debug) {
            debug!(
                "Remaining possible solutions: {}",
                remaining_solutions.len()
            );
            for solution in remaining_solutions.iter() {
                debug!("\t{}", solution)
            }
        }
        let remaining_constraints: HashSet<&Constraint> = relevant_constraints
            .iter()
            .filter(|&&c| c != decision.current_guess.constraint)
            .map(|c| *c)
            .collect();
        if let Some(best_guess) = best_guess(&remaining_constraints, remaining_solutions) {
            let verified_decision = Decision::prepare(
                decision.guesses.iter().copied().collect(),
                Guess {
                    constraint: best_guess,
                    verified: true,
                },
                &available_solutions,
            );

            let unverified_decision = Decision::prepare(
                decision.guesses.iter().copied().collect(),
                Guess {
                    constraint: best_guess,
                    verified: false,
                },
                &available_solutions,
            );
            decisions.insert(
                decision,
                Branch {
                    verified: Some(verified_decision.clone()),
                    unverified: Some(unverified_decision.clone()),
                },
            );
            unbranched_decisions.push(verified_decision);
            unbranched_decisions.push(unverified_decision);
        } else {
            error!("Beep")
        }
    }

    return DecisionTree {
        roots: roots,
        decisions: decisions,
    };
}

/// The difference between the number of solutions with
/// the provided constraint verified vs the number of
/// solutions with the provided constraint unverified
fn tf_difference(constraint: &Constraint, solutions: &Vec<&Solution>) -> u32 {
    let tf_set = solutions
        .iter()
        .fold(HashMap::<bool, u32>::new(), |mut tf_set, solution| {
            *tf_set
                .entry(solution.constraints.contains(constraint))
                .or_insert(0) += 1;
            tf_set
        });

    u32::abs_diff(
        *tf_set.get(&true).unwrap_or(&0),
        *tf_set.get(&false).unwrap_or(&0),
    )
}

/// Determine the ideal constraint
/// to guess using an
/// implementation-specific heuristic
fn best_guess<'a>(
    available_constraints: &HashSet<&'a Constraint>,
    possible_solutions: &Vec<&Solution>,
) -> Option<&'a Constraint> {
    let best_guess = available_constraints
        .iter()
        .min_by(|&&c1, &&c2| {
            tf_difference(c1, possible_solutions).cmp(&tf_difference(c2, possible_solutions))
        })
        .map(|g| *g);
    if let Some(best_guess) = best_guess {
        debug!(
            "Next best guess {} ({})",
            best_guess,
            tf_difference(best_guess, possible_solutions)
        );
    }
    best_guess
}

/// Filter the provided solutions
/// to include only solutions
/// that contain only solutions possible
/// with the provided guess sequence
pub(super) fn shrink<'a>(
    available_solutions: &Vec<&'a Solution>,
    guess_sequence: &Vec<Guess<'a>>,
) -> Vec<&'a Solution> {
    available_solutions
        .iter()
        .filter(|&&solution| {
            guess_sequence.iter().all(|&guess| {
                if guess.verified {
                    solution.constraints.contains(guess.constraint)
                } else {
                    !solution.constraints.contains(guess.constraint)
                }
            })
        })
        .map(|s| *s)
        .collect_vec()
}
