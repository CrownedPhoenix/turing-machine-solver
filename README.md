# Turing Machine Solver
A command line utility for creating a decision tree for identifying
the solution to a game of [Turing Machine](https://www.turingmachine.info/).

## Quick Start
Provided the card numbers associated with each module, `turing-solve`
will list the constraints that, if valid, would permit only
a single solution.

Then it will display a decision tree that can be used to
identify which of those possible solutions is correct.

For example:
```sh
> turing-solve 3 10 14 17
------ Possible solutions -----

(122) with (3.0) 🟨 < 3; (10.0) No 4s; (14.0) 🔷 smallest; (16.0) Even > Odd
(212) with (3.0) 🟨 < 3; (10.0) No 4s; (14.1) 🟨 smallest; (16.0) Even > Odd
...

------ Decision tree -----

(3.0) 🟨 < 3 
        (10.0) No 4s 
                (14.0) 🔷 smallest --- 122
                !(14.0) 🔷 smallest 
                        (14.2) 🟣 smallest --- 221
                        !(14.2) 🟣 smallest --- 212
        !(10.0) No 4s 
                ...
!(3.0) 🟨 < 3 
        (16.1) Odd > Even 
        ...
```

The output format is:
```sh
(122) with (3.0) 🟨 < 3; (10.0) No 4s; (14.0) 🔷 smallest; (16.0) Even > Odd
  |          |
  |          |- The constraint id (<card #>.<constraint #>)
  |- The solution code
```
```sh
(3.0) 🟨 < 3 
        (10.0) No 4s 
                (14.0) 🔷 smallest --- 122 # Solution code for this path
```
