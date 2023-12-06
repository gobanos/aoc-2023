use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

struct Races {
    times: Vec<u32>,
    distances: Vec<u32>,
}

#[aoc_generator(day6, part1)]
fn parse(input: &str) -> Result<Races> {
    let (_, races) = parser::races(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(races)
}

#[aoc(day6, part1)]
fn part1(races: &Races) -> u32 {
    races
        .times
        .iter()
        .zip(&races.distances)
        .map(|(&time, &distance)| {
            let delta: f64 = (time * time - 4 * distance).into();
            let time_f64: f64 = time.into();
            let sqrt = delta.sqrt();
            let root1 = (time_f64 - sqrt) / 2.0;
            let root2 = (time_f64 + sqrt) / 2.0;
            root2.ceil() as u32 - root1.floor() as u32 - 1
        })
        .product()
}

#[aoc_generator(day6, part2)]
fn parse_with_bad_kerning(input: &str) -> Result<(u64, u64)> {
    let (_, race) = parser::race_with_bad_kerning(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(race)
}

#[aoc(day6, part2)]
fn part2(&(time, distance): &(u64, u64)) -> u64 {
    let delta: f64 = (time * time - 4 * distance) as f64;
    let time_f64 = time as f64;
    let sqrt = delta.sqrt();
    let root1 = (time_f64 - sqrt) / 2.0;
    let root2 = (time_f64 + sqrt) / 2.0;
    root2.ceil() as u64 - root1.floor() as u64 - 1
}

mod parser {
    use crate::day6::Races;
    use crate::nom_parser::number;
    use nom::bytes::complete::{tag, take_while1};
    use nom::combinator::opt;
    use nom::error::ErrorKind;
    use nom::multi::{fold_many1, many1, separated_list1};
    use nom::sequence::preceded;
    use nom::IResult;
    use std::str::FromStr;

    fn spaces(input: &str) -> IResult<&str, ()> {
        fold_many1(tag(" "), || (), |acc, _| acc)(input)
    }

    pub fn races(input: &str) -> IResult<&str, Races> {
        let (input, _) = preceded(tag("Time:"), spaces)(input)?;
        let (input, times) = separated_list1(spaces, number)(input)?;
        let (input, _) = preceded(tag("\nDistance:"), spaces)(input)?;
        let (input, distances) = separated_list1(spaces, number)(input)?;

        Ok((input, Races { times, distances }))
    }

    pub fn number_with_bad_kerning<T: FromStr>(input: &str) -> IResult<&str, T> {
        let (input, number_str) = many1(preceded(
            opt(spaces),
            take_while1(|c: char| c.is_ascii_digit()),
        ))(input)?;
        Ok((
            input,
            number_str
                .join("")
                .parse()
                .map_err(|_| nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Fail)))?,
        ))
    }

    pub fn race_with_bad_kerning(input: &str) -> IResult<&str, (u64, u64)> {
        let (input, _) = preceded(tag("Time:"), spaces)(input)?;
        let (input, time) = number_with_bad_kerning(input)?;
        let (input, _) = preceded(tag("\nDistance:"), spaces)(input)?;
        let (input, distance) = number_with_bad_kerning(input)?;

        Ok((input, (time, distance)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_with_bad_kerning(EXAMPLE).unwrap()), 71503);
    }
}
