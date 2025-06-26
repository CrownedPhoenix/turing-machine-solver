use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::{scenario::constraint::Constraint, solver::Solution};

pub fn guess_sequences(solutions: &Vec<Solution>) -> Vec<Vec<Constraint>> {
    let relevant_constraints: HashSet<&Constraint> = solutions.iter().fold(
        HashSet::<&Constraint>::new(),
        |mut constraints, solution| {
            constraints.extend(solution.constraints.iter());
            constraints
        },
    );

    let constraints_by_card: HashMap<u8, Vec<&Constraint>> = relevant_constraints.iter().fold(
        HashMap::<u8, Vec<&Constraint>>::new(),
        |mut map, constraint| {
            map.entry(constraint.id.card)
                .or_insert(vec![])
                .push(constraint);
            map
        },
    );

    let guess_sequences: Vec<Vec<&Constraint>> = constraints_by_card
        .values()
        .permutations(constraints_by_card.len())
        .flat_map(|permutation| {
            permutation
                .into_iter()
                .multi_cartesian_product()
                .unique()
                .map(|v| v.into_iter().map(|e| *e).collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let converging_guess_sequences = guess_sequences
        .into_iter()
        .filter_map(|sequence| -> Option<Vec<&Constraint>> {
            shortest_converging_guess_sequence(&sequence, solutions)
        })
        .unique()
        .collect_vec();

    let min_sequence_length = converging_guess_sequences
        .iter()
        .min_by(|a, b| a.len().cmp(&b.len()))
        .map(|s| s.len());
    let shortest_resolvable_guess_sequences = converging_guess_sequences
        .into_iter()
        .filter(|a| a.len() == min_sequence_length.unwrap_or(1));

    println!("---- Best possible guess sequences ----\n");
    for guess_sequence in shortest_resolvable_guess_sequences {
        let guess_str = guess_sequence
            .iter()
            .map(|&e| format!("({},{})", e.id.card, e.id.idx))
            .join(" ");
        println!("{}", guess_str);
    }

    vec![]
}

fn shortest_converging_guess_sequence<'a>(
    sequence: &Vec<&'a Constraint>,
    solutions: &Vec<Solution>,
) -> Option<Vec<&'a Constraint>> {
    (1..=sequence.len())
        .into_iter()
        .find(|&sequence_end| {
            let subsequence: &[&Constraint] = &sequence[0..sequence_end];
            solutions
                .iter()
                .filter(|&solution| {
                    subsequence
                        .iter()
                        .all(|&constraint| solution.constraints.contains(constraint))
                })
                .count()
                == 1
                || solutions
                    .iter()
                    .filter(|&solution| {
                        subsequence
                            .iter()
                            .all(|&constraint| !solution.constraints.contains(constraint))
                    })
                    .count()
                    == 1
        })
        .map(|end| sequence[..end].to_vec())
}
