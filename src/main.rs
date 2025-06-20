use clap::Parser;
use itertools::Itertools;
use std::str::FromStr;
use turing_solve::solver::{CardOrConstraint, turing_solve};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// A command line utility for creating a decision
/// tree for identifying the solution to a game of Turing Machine.
struct Args {
    #[arg(short)]
    a: Option<CardOrConstraintArg>,

    #[arg(short)]
    b: Option<CardOrConstraintArg>,

    #[arg(short)]
    c: Option<CardOrConstraintArg>,

    #[arg(short)]
    d: Option<CardOrConstraintArg>,

    #[arg(short)]
    e: Option<CardOrConstraintArg>,

    #[arg(short)]
    f: Option<CardOrConstraintArg>,
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

fn main() {
    env_logger::init();
    let args = Args::parse();
    // TODO: Do the solve
    let solutions = turing_solve(
        args.a.map(CardOrConstraintArg::into),
        args.b.map(CardOrConstraintArg::into),
        args.c.map(CardOrConstraintArg::into),
        args.d.map(CardOrConstraintArg::into),
        args.e.map(CardOrConstraintArg::into),
        args.f.map(CardOrConstraintArg::into),
    );
    for solution in solutions {
        println!("{}", solution)
    }
}

impl Into<CardOrConstraint> for CardOrConstraintArg {
    fn into(self) -> CardOrConstraint {
        match self {
            CardOrConstraintArg::Card(num) => CardOrConstraint::Card(num),
            CardOrConstraintArg::CardConstraint(card_num, constraint_num) => {
                CardOrConstraint::CardConstraint(card_num, constraint_num)
            }
        }
    }
}
