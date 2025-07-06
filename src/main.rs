use clap::Parser;
use itertools::Itertools;
use std::str::FromStr;
use turing_solve::{
    guess_sequence::solver::guess_sequences,
    solver::{CardOrConstraint, turing_solve},
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
            .get(0)
            .map(<&CardOrConstraintArg as Into<CardOrConstraint>>::into),
        args.constraints
            .get(1)
            .map(<&CardOrConstraintArg as Into<CardOrConstraint>>::into),
        args.constraints
            .get(2)
            .map(<&CardOrConstraintArg as Into<CardOrConstraint>>::into),
        args.constraints
            .get(3)
            .map(<&CardOrConstraintArg as Into<CardOrConstraint>>::into),
        args.constraints
            .get(4)
            .map(<&CardOrConstraintArg as Into<CardOrConstraint>>::into),
        args.constraints
            .get(5)
            .map(<&CardOrConstraintArg as Into<CardOrConstraint>>::into),
    );
    for solution in &solutions {
        println!("{}", solution)
    }

    println!();
    println!("-------------");
    println!();

    guess_sequences(&solutions);

    Ok(())
}

impl Into<CardOrConstraint> for &CardOrConstraintArg {
    fn into(self) -> CardOrConstraint {
        match self {
            CardOrConstraintArg::Card(num) => CardOrConstraint::Card(*num),
            CardOrConstraintArg::CardConstraint(card_num, constraint_num) => {
                CardOrConstraint::CardConstraint(*card_num, *constraint_num)
            }
        }
    }
}
