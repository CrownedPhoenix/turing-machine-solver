use crate::{
    guess_sequence::{Guess, solver::shrink},
    scenario::code::Code,
    solver::Solution,
};
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Decision<'a> {
    pub guesses: Vec<Guess<'a>>,
    pub current_guess: Guess<'a>,
    pub solution: Option<Code>,
}

impl<'a> Decision<'a> {
    pub fn prepare(
        prior_guesses: Vec<Guess<'a>>,
        current_guess: Guess<'a>,
        available_solutions: &Vec<&Solution>,
    ) -> Decision<'a> {
        let guesses = prior_guesses
            .iter()
            .cloned()
            .chain(std::iter::once(current_guess))
            .collect_vec();
        let solutions = shrink(&available_solutions, &guesses);
        Decision {
            guesses: guesses,
            current_guess: current_guess,
            solution: if solutions.len() == 1 {
                solutions.first().map(|s| s.code)
            } else {
                None
            },
        }
    }
}

pub struct Branch<'a> {
    pub verified: Option<Decision<'a>>,
    pub unverified: Option<Decision<'a>>,
}
