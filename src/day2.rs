use crate::day2::parser::games;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Game {
    id: u32,
    set_of_cubes: Vec<SetOfCubes>,
}

#[derive(Default)]
struct SetOfCubes(HashMap<Color, u32>);

impl SetOfCubes {
    fn contains(&self, other: &Self) -> bool {
        other
            .0
            .iter()
            .all(|(color, &count)| self.0.get(color).copied().unwrap_or_default() >= count)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Game>> {
    let (_, games) = games(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(games)
}

#[aoc(day2, part1)]
fn part1(games: &[Game]) -> u32 {
    let bag = SetOfCubes(
        [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]
            .into_iter()
            .collect(),
    );
    games
        .iter()
        .filter(|game| {
            game.set_of_cubes
                .iter()
                .all(|set_of_cubes| bag.contains(set_of_cubes))
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.set_of_cubes
                .iter()
                .fold(SetOfCubes::default(), |mut minimal_set, set_of_cubes| {
                    for (&color, &count) in &set_of_cubes.0 {
                        let entry = minimal_set.0.entry(color).or_insert(0);
                        *entry = u32::max(*entry, count);
                    }
                    minimal_set
                })
        })
        .map(|minimal_set| minimal_set.0.values().product::<u32>())
        .sum()
}

mod parser {
    use crate::day2::{Color, Game, SetOfCubes};
    use crate::nom_parser::number;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::multi::{separated_list0, separated_list1};
    use nom::IResult;
    use nom::Parser;

    fn color(input: &str) -> IResult<&str, (Color, u32)> {
        let (input, count) = number(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, color) = alt((
            tag("red").map(|_| Color::Red),
            tag("green").map(|_| Color::Green),
            tag("blue").map(|_| Color::Blue),
        ))(input)?;
        Ok((input, (color, count)))
    }

    fn set_of_cubes(input: &str) -> IResult<&str, SetOfCubes> {
        let (input, colors) = separated_list1(tag(", "), color)(input)?;

        Ok((input, SetOfCubes(colors.into_iter().collect())))
    }

    fn game(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = number(input)?;
        let (input, _) = tag(": ")(input)?;

        let (input, set_of_cubes) = separated_list1(tag("; "), set_of_cubes)(input)?;

        Ok((input, Game { id, set_of_cubes }))
    }

    pub fn games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list0(tag("\n"), game)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 2286);
    }
}
