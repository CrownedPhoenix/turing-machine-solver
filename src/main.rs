use clap::Parser;
use itertools::Itertools;
use std::str::FromStr;
use turing_solve::{
    guess_sequence::solver::guess_sequence,
    solver::{Constraint, constraints_for_card, turing_solve},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// A command line utility for creating a decision
/// tree for identifying the solution to a game of Turing Machine.
struct Args {
    #[arg(help = "Set of constraints to apply (max 6)")]
    constraints: Vec<CardOrConstraintArg>,
}

#[derive(Clone)]
pub enum CardOrConstraintArg {
    Card(u8),
    CardConstraint(u8, u8),
}

impl FromStr for CardOrConstraintArg {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = u8::from_str(s) {
            Ok(CardOrConstraintArg::Card(num))
        } else {
            let Some(split): Option<(&str, &str)> = s.split(".").collect_tuple() else {
                return Err("");
            };
            let Ok(card_num) = u8::from_str(split.0) else {
                return Err("");
            };
            let Ok(constraint_num) = u8::from_str(split.1) else {
                return Err("");
            };
            Ok(CardOrConstraintArg::CardConstraint(
                card_num,
                constraint_num,
            ))
        }
    }
}

fn main() -> Result<(), &'static str> {
    env_logger::init();
    let args = Args::parse();
    if args.constraints.len() > 6 {
        return Err("Too many constraints provided");
    }
    // TODO: Do the solve
    let solutions = turing_solve(
        args.constraints
            .iter()
            .flat_map(<&CardOrConstraintArg as Into<Vec<Constraint>>>::into)
            .collect_vec(),
    );
    if !solutions.is_empty() {
        for solution in &solutions {
            println!("{}", solution)
        }
    } else {
        println!("----- No possible solutions -----");
        return Ok(());
    }

    let Some(decision_tree) = guess_sequence(&solutions) else {
        return Err("Could not construct decision tree");
    };
    println!("\n------ Decision tree -----\n");
    println!("{}", decision_tree);

    Ok(())
}

impl Into<Vec<Constraint>> for &CardOrConstraintArg {
    fn into(self) -> Vec<Constraint> {
        match self {
            CardOrConstraintArg::Card(num) => constraints_for_card(*num).unwrap(),
            CardOrConstraintArg::CardConstraint(card_num, constraint_num) => {
                constraints_for_card(*card_num)
                    .unwrap()
                    .get(*constraint_num as usize)
                    .copied()
                    .into_iter()
                    .collect()
            }
        }
    }
}
