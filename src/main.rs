use clap::Parser;
use turing_solve::solver::turing_solve;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// A command line utility for creating a decision
/// tree for identifying the solution to a game of Turing Machine.
struct Args {
    #[arg(short)]
    a: Option<u8>,

    #[arg(short)]
    b: Option<u8>,

    #[arg(short)]
    c: Option<u8>,

    #[arg(short)]
    d: Option<u8>,

    #[arg(short)]
    e: Option<u8>,

    #[arg(short)]
    f: Option<u8>,
}
fn main() {
    let args = Args::parse();
    for solution in turing_solve(args.a, args.b, args.c, args.d, args.e, args.f) {
        // TODO: Do the solve; print out viable possibilities
    }
}
