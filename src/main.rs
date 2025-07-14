use clap::Parser;
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
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
    CardConstraint {
        inverted: bool,
        card_num: u8,
        id: u8,
    },
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
            let inverted = split.0.starts_with('^');
            let Ok(card_num) = u8::from_str(if inverted { &split.0[1..] } else { split.0 }) else {
                return Err("");
            };
            let Ok(constraint_num) = u8::from_str(split.1) else {
                return Err("");
            };
            Ok(CardOrConstraintArg::CardConstraint {
                inverted: inverted,
                card_num: card_num,
                id: constraint_num,
            })
        }
    }
}

fn main() -> Result<(), &'static str> {
    env_logger::init();
    let args = Args::parse();
    if args.constraints.len() > 6 {
        return Err("Too many constraints provided");
    }
    let constraints_to_filter: HashSet<(u8, u8)> = args
        .constraints
        .iter()
        .filter_map(|cc| {
            let CardOrConstraintArg::CardConstraint {
                inverted: true,
                card_num,
                id,
            } = cc
            else {
                return None;
            };
            Some((*card_num, *id))
        })
        .collect();
    let solutions = turing_solve(
        args.constraints
            .iter()
            .filter_map(|cc| -> Option<Vec<Constraint>> {
                match *cc {
                    CardOrConstraintArg::Card(num) => constraints_for_card(num),
                    CardOrConstraintArg::CardConstraint {
                        inverted: false,
                        card_num,
                        id: constraint_num,
                    } => constraints_for_card(card_num)
                        .and_then(|constraints| constraints.get(constraint_num as usize).cloned())
                        .map(Constraint::into),
                    CardOrConstraintArg::CardConstraint {
                        inverted: true,
                        card_num,
                        ..
                    } => constraints_for_card(card_num),
                }
            })
            .flat_map(|vc| vc)
            .filter(|cc| !constraints_to_filter.contains(&(cc.id.card, cc.id.idx)))
            .dedup_by(|x, y| x.id.eq(&y.id))
            .collect(),
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
