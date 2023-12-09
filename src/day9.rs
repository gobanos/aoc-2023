use crate::nom_parser::to_result;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

struct Oasis {
    sensors: Vec<History>,
}

struct History {
    values: Vec<i32>,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Oasis> {
    to_result(parser::oasis(input))
}

const ZEROS: [i32; 21] = [0; 21];

#[aoc(day9, part1)]
fn part1(oasis: &Oasis) -> i32 {
    oasis
        .sensors
        .iter()
        .map(|sensor| {
            let mut values = sensor.values.clone();
            let mut values = values.as_mut_slice();
            let mut prediction = 0;
            loop {
                if values == &ZEROS[..values.len()] {
                    return prediction;
                }
                prediction += *values.last().unwrap();
                for i in 1..values.len() {
                    let ([.., a], [b, ..]) = values.split_at_mut(i) else { panic!("Check your maths"); };
                    *a = *b - *a;
                }
                let new_len = values.len() - 1;
                values = &mut values[..new_len];

            }
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(oasis: &Oasis) -> String {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), "<RESULT>");
    }
}
