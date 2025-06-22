use itertools::Itertools;

use crate::scenario::{
    code::Code,
    constraint::{Constraint, ConstraintID},
};

#[derive(Clone, Debug)]
pub(crate) struct Card {
    pub(crate) constraints: Vec<Constraint>,
}

impl Card {
    /// Convenience initializer.
    fn new<const N: usize>(num: u8, constraints: [(&'static str, fn(&Code) -> bool); N]) -> Self {
        Card {
            constraints: constraints
                .iter()
                .enumerate()
                .map(|(idx, (name, verifier))| Constraint {
                    id: ConstraintID {
                        card: num,
                        idx: idx as u8,
                    },
                    name: *name,
                    verifier: *verifier,
                })
                .collect_vec(),
        }
    }
}

impl TryFrom<u8> for Card {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Card::new(
                1,
                [
                    ("Blue = 1", |code| code.blue() == 1),
                    ("Blue > 1", |code| code.blue() > 1),
                ],
            )),
            2 => Ok(Card::new(
                2,
                [
                    ("Blue < 3", |code| code.blue() < 3),
                    ("Blue = 3", |code| code.blue() == 3),
                    ("Blue > 3", |code| code.blue() > 3),
                ],
            )),
            3 => Ok(Card::new(
                3,
                [
                    ("Yellow < 3", |code| code.yellow() < 3),
                    ("Yellow = 3", |code| code.yellow() == 3),
                    ("Yellow > 3", |code| code.yellow() > 3),
                ],
            )),
            4 => Ok(Card::new(
                4,
                [
                    ("Yellow < 4", |code| code.yellow() < 4),
                    ("Yellow = 4", |code| code.yellow() == 4),
                    ("Yellow > 4", |code| code.yellow() > 4),
                ],
            )),
            5 => Ok(Card::new(
                5,
                [
                    ("Blue is even", |code| code.blue() % 2 == 0),
                    ("Blue is odd", |code| code.blue() % 2 != 0),
                ],
            )),
            6 => Ok(Card::new(
                6,
                [
                    ("Yellow is even", |code| code.yellow() % 2 == 0),
                    ("Yellow is odd", |code| code.yellow() % 2 != 0),
                ],
            )),
            7 => Ok(Card::new(
                7,
                [
                    ("Purple is even", |code| code.purple() % 2 == 0),
                    ("Purple is odd", |code| code.purple() % 2 != 0),
                ],
            )),
            8 => Ok(Card::new(
                8,
                [
                    ("No 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 0
                    }),
                    ("One 1", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 1
                    }),
                    ("Two 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 2
                    }),
                    ("Three 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 3
                    }),
                ],
            )),
            9 => Ok(Card::new(
                9,
                [
                    ("No 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 0
                    }),
                    ("One 3", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 1
                    }),
                    ("Two 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 2
                    }),
                    ("Three 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 3
                    }),
                ],
            )),
            10 => Ok(Card::new(
                10,
                [
                    ("No 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 0
                    }),
                    ("One 4", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 1
                    }),
                    ("Two 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 2
                    }),
                    ("Three 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 3
                    }),
                ],
            )),
            11 => Ok(Card::new(
                11,
                [
                    ("Blue < Yellow", |code| code.blue() < code.yellow()),
                    ("Blue = Yellow", |code| code.blue() == code.yellow()),
                    ("Blue > Yellow", |code| code.blue() > code.yellow()),
                ],
            )),
            12 => Ok(Card::new(
                12,
                [
                    ("Blue < Purple", |code| code.blue() < code.purple()),
                    ("Blue = Purple", |code| code.blue() == code.purple()),
                    ("Blue > Purple", |code: &Code| code.blue() > code.purple()),
                ],
            )),
            13 => Ok(Card::new(
                13,
                [
                    ("Yellow < Purple", |code| code.yellow() < code.purple()),
                    ("Yellow = Purple", |code| code.yellow() == code.purple()),
                    ("Yellow > Purple", |code| code.yellow() > code.purple()),
                ],
            )),
            14 => Ok(Card::new(
                14,
                [
                    ("Blue smallest", |code| {
                        code.blue() < code.yellow() && code.blue() < code.purple()
                    }),
                    ("Yellow smallest", |code| {
                        code.yellow() < code.blue() && code.yellow() < code.purple()
                    }),
                    ("Purple smallest", |code| {
                        code.purple() < code.yellow() && code.purple() < code.blue()
                    }),
                ],
            )),
            15 => Ok(Card::new(
                15,
                [
                    ("Blue is largest", |code| {
                        code.blue() > code.yellow() && code.blue() > code.purple()
                    }),
                    ("Yellow is largest", |code| {
                        code.yellow() > code.blue() && code.yellow() > code.purple()
                    }),
                    ("Purple is largest", |code| {
                        code.purple() > code.yellow() && code.purple() > code.blue()
                    }),
                ],
            )),
            16 => Ok(Card::new(
                16,
                [
                    ("Even > Odd", |code| {
                        code.iter().filter(|n: &u8| *n % 2 == 0).count() == 2
                    }),
                    ("Odd > Even", |code| {
                        code.iter().filter(|n: &u8| *n % 2 != 0).count() == 2
                    }),
                ],
            )),
            17 => Ok(Card::new(
                17,
                [
                    ("No even #", |code| {
                        code.iter().filter(|n: &u8| *n % 2 == 0).count() == 0
                    }),
                    ("1 even #", |code| {
                        code.iter().filter(|n: &u8| *n % 2 == 0).count() == 1
                    }),
                    ("2 even #s", |code| {
                        code.iter().filter(|n: &u8| *n % 2 == 0).count() == 2
                    }),
                    ("3 even #s", |code| {
                        code.iter().filter(|n: &u8| *n % 2 == 0).count() == 3
                    }),
                ],
            )),
            18 => Ok(Card::new(
                18,
                [
                    ("Sum even", |code| {
                        code.iter().map(|n: u8| n).sum::<u8>() % 2 == 0
                    }),
                    ("Sum odd", |code| {
                        code.iter().map(|n: u8| n).sum::<u8>() % 2 != 0
                    }),
                ],
            )),
            19 => Ok(Card::new(
                19,
                [
                    ("Blue + Yellow < 6", |code| code.blue() + code.yellow() < 6),
                    ("Blue + Yellow = 6", |code| code.blue() + code.yellow() == 6),
                    ("Blue + Yellow > 6", |code| code.blue() + code.yellow() > 6),
                ],
            )),
            20 => {
                Ok(Card::new(
                    20,
                    [
                        ("Triple #", |code| {
                            code.blue() == code.yellow() && code.yellow() == code.purple()
                        }),
                        ("Double #", |code| {
                            (code.blue() == code.yellow() && code.yellow() != code.purple()) // Only blue and yellow
                        || (code.yellow() == code.purple() && code.blue() != code.yellow()) // Only yellow and purple
                        || (code.blue() == code.purple() && code.blue() != code.yellow()) // Only blue and purple
                        }),
                        ("No repetition", |code| {
                            code.blue() != code.yellow()
                                && code.yellow() != code.purple()
                                && code.blue() != code.purple()
                        }),
                    ],
                ))
            }
            21 => Ok(Card::new(
                21,
                [
                    ("A pairs", |code| {
                        code.iter()
                            .tuple_combinations()
                            .filter(|(a, b)| a == b)
                            .count()
                            == 1
                    }),
                    ("No pairs", |code| {
                        code.iter().tuple_combinations().all(|(a, b)| a != b)
                    }),
                ],
            )),
            22 => Ok(Card::new(
                22,
                [
                    ("Ascending", |code| {
                        code.iter().tuple_windows().all(|(a, b)| a < b)
                    }),
                    ("Descending", |code| {
                        code.iter().tuple_windows().all(|(a, b)| a > b)
                    }),
                    ("No order", |code| {
                        !code.iter().tuple_windows().all(|(a, b)| a < b)
                            && !code.iter().tuple_windows().all(|(a, b)| a > b)
                    }),
                ],
            )),
            23 => Ok(Card::new(
                23,
                [
                    ("Sum < 6", |code| code.iter().sum::<u8>() < 6),
                    ("Sum = 6", |code| code.iter().sum::<u8>() == 6),
                    ("Sum > 6", |code| code.iter().sum::<u8>() > 6),
                ],
            )),
            24 => Ok(Card::new(
                24,
                [
                    ("Ascending", |code| {
                        code.iter().tuple_windows().all(|(a, b)| a < b)
                    }),
                    ("2 ascending", |code| {
                        code.iter().tuple_windows().filter(|(a, b)| a < b).count() == 1
                    }),
                    ("None ascending", |code| {
                        code.iter().tuple_windows().all(|(a, b)| !(a < b))
                    }),
                ],
            )),
            25 => Ok(Card::new(
                25,
                [
                    ("No sequence asc/dsc", |code| {
                        code.iter().tuple_windows().all(|(a, b)| !(a < b || a > b))
                    }),
                    ("2 asc/dsc", |code| {
                        code.iter().tuple_windows().any(|(a, b)| a < b || a > b)
                    }),
                    ("3 asc/dsc", |code| {
                        code.iter().is_sorted() || code.iter().rev().is_sorted()
                    }),
                ],
            )),
            26 => Ok(Card::new(
                26,
                [
                    ("Blue < 3", |code| code.blue() < 3),
                    ("Yellow < 3", |code| code.yellow() < 3),
                    ("Purple < 3", |code| code.purple() < 3),
                ],
            )),
            27 => Ok(Card::new(
                27,
                [
                    ("Blue < 4", |code| code.blue() < 4),
                    ("Yellow < 4", |code| code.yellow() < 4),
                    ("Purple < 4", |code| code.purple() < 4),
                ],
            )),
            28 => Ok(Card::new(
                28,
                [
                    ("Blue = 1", |code| code.blue() == 1),
                    ("Yellow = 1", |code| code.yellow() == 1),
                    ("Purple = 1", |code| code.purple() == 1),
                ],
            )),
            29 => Ok(Card::new(
                29,
                [
                    ("Blue = 1", |code| code.blue() == 3),
                    ("Yellow = 1", |code| code.yellow() == 3),
                    ("Purple = 1", |code| code.purple() == 3),
                ],
            )),
            30 => Ok(Card::new(
                30,
                [
                    ("Blue = 4", |code| code.blue() == 4),
                    ("Yellow = 4", |code| code.yellow() == 4),
                    ("Purple = 4", |code| code.purple() == 4),
                ],
            )),
            31 => Ok(Card::new(
                31,
                [
                    ("Blue > 1", |code| code.blue() > 1),
                    ("Yellow > 1", |code| code.yellow() > 1),
                    ("Purple > 1", |code| code.purple() > 1),
                ],
            )),
            33 => Ok(Card::new(
                33,
                [
                    ("Blue is even", |code| code.blue() % 2 == 0),
                    ("Blue is odd", |code| code.blue() % 2 != 0),
                    ("Yellow is even", |code| code.blue() % 2 == 0),
                    ("Yellow is odd", |code| code.blue() % 2 != 0),
                    ("Purple is even", |code| code.blue() % 2 == 0),
                    ("Purple is odd", |code| code.blue() % 2 != 0),
                ],
            )),
            34 => Ok(Card::new(
                34,
                [
                    ("Blue <= Yellow & Purple", |code| {
                        code.blue() <= code.yellow() && code.blue() <= code.purple()
                    }),
                    ("Purple <= Blue &Yellow", |code| {
                        code.yellow() <= code.blue() && code.yellow() <= code.purple()
                    }),
                    ("Yellow <= Blue & Purple", |code| {
                        code.purple() <= code.blue() && code.purple() <= code.yellow()
                    }),
                ],
            )),
            35 => Ok(Card::new(
                35,
                [
                    ("Blue >= Yellow & Purple", |code| {
                        code.blue() >= code.yellow() && code.blue() >= code.purple()
                    }),
                    ("Purple >= Blue &Yellow", |code| {
                        code.yellow() >= code.blue() && code.yellow() >= code.purple()
                    }),
                    ("Yellow >= Blue & Purple", |code| {
                        code.purple() >= code.blue() && code.purple() >= code.yellow()
                    }),
                ],
            )),
            36 => Ok(Card::new(
                36,
                [
                    ("Sum mutiple of 3", |code| code.iter().sum::<u8>() % 3 == 0),
                    ("Sum mutiple of 4", |code| code.iter().sum::<u8>() % 4 == 0),
                    ("Sum mutiple of 5", |code| code.iter().sum::<u8>() % 5 == 0),
                ],
            )),
            37 => Ok(Card::new(
                37,
                [
                    ("Blue + Yellow = 4", |code| code.blue() + code.yellow() == 4),
                    ("Blue + Purple = 4", |code| code.blue() + code.purple() == 4),
                    ("Yellow + Purple = 4", |code| {
                        code.yellow() + code.purple() == 4
                    }),
                ],
            )),
            38 => Ok(Card::new(
                38,
                [
                    ("Blue + Yellow = 6", |code| code.blue() + code.yellow() == 6),
                    ("Blue + Purple = 6", |code| code.blue() + code.purple() == 6),
                    ("Yellow + Purple = 6", |code| {
                        code.yellow() + code.purple() == 6
                    }),
                ],
            )),
            39 => Ok(Card::new(
                39,
                [
                    ("Blue = 1", |code| code.blue() == 1),
                    ("Blue > 1", |code| code.blue() > 1),
                    ("Yellow = 1", |code| code.yellow() == 1),
                    ("Yellow > 1", |code| code.yellow() > 1),
                    ("Purple = 1", |code| code.purple() == 1),
                    ("Purple > 1", |code| code.purple() > 1),
                ],
            )),
            40 => Ok(Card::new(
                40,
                [
                    ("Blue < 3", |code| code.blue() < 3),
                    ("Blue = 3", |code| code.blue() == 3),
                    ("Blue > 3", |code| code.blue() > 3),
                    ("Yellow < 3", |code| code.yellow() < 3),
                    ("Yellow = 3", |code| code.yellow() == 3),
                    ("Yellow > 3", |code| code.yellow() > 3),
                    ("Purple < 3", |code| code.purple() < 3),
                    ("Purple = 3", |code| code.purple() == 3),
                    ("Purple > 3", |code| code.purple() > 3),
                ],
            )),
            41 => Ok(Card::new(
                41,
                [
                    ("Blue < 4", |code| code.blue() < 4),
                    ("Blue = 4", |code| code.blue() == 4),
                    ("Blue > 4", |code| code.blue() > 4),
                    ("Yellow < 4", |code| code.yellow() < 4),
                    ("Yellow = 4", |code| code.yellow() == 4),
                    ("Yellow > 4", |code| code.yellow() > 4),
                    ("Purple < 4", |code| code.purple() < 4),
                    ("Purple = 4", |code| code.purple() == 4),
                    ("Purple > 4", |code| code.purple() > 4),
                ],
            )),
            42 => Ok(Card::new(
                42,
                [
                    ("Blue smallest", |code| {
                        code.blue() < code.yellow() && code.blue() < code.purple()
                    }),
                    ("Yellow smallest", |code| {
                        code.yellow() < code.blue() && code.yellow() < code.purple()
                    }),
                    ("Purple smallest", |code| {
                        code.purple() < code.yellow() && code.purple() < code.blue()
                    }),
                    ("Blue largest", |code| {
                        code.blue() > code.yellow() && code.blue() > code.purple()
                    }),
                    ("Yellow largest", |code| {
                        code.yellow() > code.blue() && code.yellow() > code.purple()
                    }),
                    ("Purple largest", |code| {
                        code.purple() > code.yellow() && code.purple() > code.blue()
                    }),
                ],
            )),
            43 => Ok(Card::new(
                43,
                [
                    ("Blue < Yellow", |code| code.blue() < code.yellow()),
                    ("Blue = Yellow", |code| code.blue() == code.yellow()),
                    ("Blue > Yellow", |code| code.blue() > code.yellow()),
                    ("Blue < Purple", |code| code.blue() < code.purple()),
                    ("Blue = Purple", |code| code.blue() == code.purple()),
                    ("Blue > Purple", |code: &Code| code.blue() > code.purple()),
                ],
            )),
            44 => Ok(Card::new(
                44,
                [
                    ("Yellow < Blue", |code| code.yellow() < code.blue()),
                    ("Yellow = Blue", |code| code.yellow() == code.blue()),
                    ("Yellow > Blue", |code| code.yellow() > code.blue()),
                    ("Yellow < Purple", |code| code.yellow() < code.purple()),
                    ("Yellow = Purple", |code| code.yellow() == code.purple()),
                    ("Yellow > Purple", |code: &Code| {
                        code.yellow() > code.purple()
                    }),
                ],
            )),
            45 => Ok(Card::new(
                45,
                [
                    ("No 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 0
                    }),
                    ("One 1", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 1
                    }),
                    ("Two 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 2
                    }),
                    ("No 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 0
                    }),
                    ("One 3", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 1
                    }),
                    ("Two 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 2
                    }),
                ],
            )),
            46 => Ok(Card::new(
                46,
                [
                    ("No 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 0
                    }),
                    ("One 3", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 1
                    }),
                    ("Two 3s", |code| {
                        code.iter().filter(|n: &u8| *n == 3).count() == 2
                    }),
                    ("No 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 0
                    }),
                    ("One 4", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 1
                    }),
                    ("Two 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 2
                    }),
                ],
            )),
            47 => Ok(Card::new(
                47,
                [
                    ("No 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 0
                    }),
                    ("One 1", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 1
                    }),
                    ("Two 1s", |code| {
                        code.iter().filter(|n: &u8| *n == 1).count() == 2
                    }),
                    ("No 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 0
                    }),
                    ("One 4", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 1
                    }),
                    ("Two 4s", |code| {
                        code.iter().filter(|n: &u8| *n == 4).count() == 2
                    }),
                ],
            )),
            48 => Ok(Card::new(
                48,
                [
                    ("Blue < Yellow", |code| code.blue() < code.yellow()),
                    ("Blue = Yellow", |code| code.blue() == code.yellow()),
                    ("Blue > Yellow", |code| code.blue() > code.yellow()),
                    ("Blue < Purple", |code| code.blue() < code.purple()),
                    ("Blue = Purple", |code| code.blue() == code.purple()),
                    ("Blue > Purple", |code: &Code| code.blue() > code.purple()),
                    ("Yellow < Purple", |code| code.yellow() < code.purple()),
                    ("Yellow = Purple", |code| code.yellow() == code.purple()),
                    ("Yellow > Purple", |code| code.yellow() > code.purple()),
                ],
            )),
            _ => Err("No mapping for specified card number."),
        }
    }
}
