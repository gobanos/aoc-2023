use crate::nom_parser::to_result;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Oasis {
    sensors: Vec<History>,
}

#[derive(Debug, Clone)]
struct History {
    values: Vec<i32>,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Oasis> {
    to_result(parser::oasis(input))
}

const ZEROS: [i32; 21] = [0; 21];

#[derive(Debug)]
struct PartialSolution {
    past: i32,
    future: i32,
}

fn solve(oasis: Oasis) -> impl Iterator<Item = PartialSolution> {
    oasis.sensors.into_iter().map(|mut sensor| {
        let mut values = sensor.values.as_mut_slice();
        let mut future = 0;
        let mut past = [0; 2];
        for i in 0.. {
            if values == &ZEROS[..values.len()] {
                let sol = PartialSolution {
                    future,
                    past: past[0] - past[1],
                };

                return sol;
            }
            future += *values.last().unwrap();
            past[i % 2] += values[0];
            for i in 1..values.len() {
                let ([.., a], [b, ..]) = values.split_at_mut(i) else {
                    panic!("Check your maths");
                };
                *a = *b - *a;
            }
            let new_len = values.len() - 1;
            values = &mut values[..new_len];
        }
        unreachable!()
    })
}

#[aoc(day9, part1)]
fn part1(oasis: &Oasis) -> i32 {
    solve(oasis.clone()).map(|sol| sol.future).sum()
}

#[aoc(day9, part2)]
fn part2(oasis: &Oasis) -> i32 {
    solve(oasis.clone()).map(|sol| sol.past).sum()
}

mod parser {
    use crate::day9::{History, Oasis};
    use nom::character::complete::{i32, newline, space1};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::IResult;

    pub fn oasis(input: &str) -> IResult<&str, Oasis> {
        map(
            separated_list1(
                newline,
                map(separated_list1(space1, i32), |values| History { values }),
            ),
            |sensors| Oasis { sensors },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 2);
    }

    const CORRECT_LINE: &str = "-7 -6 -4 -9 -35 -97 -191 -246 -18 1130 4645 13492 33206 73178 147656 275084 472783 741142 1026794 1146833 644835";
    const BUGGY_LINE: &str = "9 26 43 55 64 84 151 349 869 2131 5030 11432 25171 54036 113717 235730 483765 988508 2023553 4171877 8686902";
    #[test]
    fn part2_buggy_line() {
        assert_eq!(part2(&parse(CORRECT_LINE).unwrap()), -4);
        assert_eq!(part2(&parse(BUGGY_LINE).unwrap()), -4);
    }
}
