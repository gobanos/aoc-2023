use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Vec<Card>> {
    let (_, cards) = parser::cards(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(cards)
}

#[aoc(day4, part1)]
fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| card.winning_numbers.intersection(&card.my_numbers).count())
        .filter_map(|matching_numbers| match matching_numbers {
            0 => None,
            matches => Some(1 << (matches - 1)),
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[Card]) -> u32 {
    let (_, score) = cards
        .iter()
        .map(|card| card.winning_numbers.intersection(&card.my_numbers).count())
        .fold(
            (VecDeque::new(), 0),
            |(mut stack, score), matching_numbers| {
                let cards = stack.pop_front().unwrap_or(0) + 1;
                for i in 0..matching_numbers {
                    if let Some(c) = stack.get_mut(i) {
                        *c += cards;
                    } else {
                        stack.push_back(cards);
                    }
                }
                (stack, score + cards)
            },
        );
    score
}

mod parser {
    use crate::day4::Card;
    use crate::nom_parser::number;
    use nom::bytes::complete::tag;
    use nom::combinator::opt;
    use nom::multi::{fold_many0, separated_list1};
    use nom::sequence::preceded;
    use nom::IResult;

    fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(tag(" "), preceded(opt(tag(" ")), number))(input)
    }

    fn card(input: &str) -> IResult<&str, Card> {
        let (input, _) = tag("Card ")(input)?;
        let (input, _) = fold_many0(tag(" "), || (), |acc, _| acc)(input)?;
        let (input, _) = number(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, winning_numbers) = numbers(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, my_numbers) = numbers(input)?;
        Ok((
            input,
            Card {
                winning_numbers: winning_numbers.into_iter().collect(),
                my_numbers: my_numbers.into_iter().collect(),
            },
        ))
    }

    pub fn cards(input: &str) -> IResult<&str, Vec<Card>> {
        separated_list1(tag("\n"), card)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 30);
    }
}
