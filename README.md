# Turing Machine Solver
A command line utility for creating a decision tree for identifying
the solution to a game of [Turing Machine](https://www.turingmachine.info/).

## Quick Start
Provided the card numbers associated with each module, `turing-solve`
will list the constraints that, if valid, would permit only
a single solution.

For example:
```sh
> turing-solve -a 3 -b 10 -c 14 -d 17
A: 🟨 < 🔷; B: 🟣 smallest; C: 🔷 = 3; D: 🟨 + 🟣 + 🔷 < 6 ==> 312
```

Using this information, it is left to the user to determine how to
most efficiently determine which constraints are valid.

In the future, this tool may be updated with further functionality such as:
- Identifying the sequence of checks that will progressively eliminate
    the most number of possibilities.
- Determining the best numeric guess to apply across a set of modules.