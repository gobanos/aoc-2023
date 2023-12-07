use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::mem;

struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    N(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn into_usize(self) -> usize {
        match self {
            Card::N(n) => (n - 2).into(),
            Card::T => 8,
            Card::J => 9,
            Card::Q => 10,
            Card::K => 11,
            Card::A => 12,
        }
    }

    fn cmp_with_joker(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::J, Card::J) => Ordering::Equal,
            (Card::J, _) => Ordering::Less,
            (_, Card::J) => Ordering::Greater,
            _ => self.cmp(other),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut cards_count = [0u8; 13];
        for &card in &self.cards {
            cards_count[card.into_usize()] += 1;
        }
        cards_count.sort_by(|a, b| a.cmp(b).reverse());
        match cards_count.as_slice() {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            [1, ..] => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn hand_type_with_jokers(&self) -> HandType {
        let mut cards_count = [0u8; 13];
        for &card in &self.cards {
            cards_count[card.into_usize()] += 1;
        }
        let jokers = mem::replace(&mut cards_count[Card::J.into_usize()], 0);
        cards_count.sort_by(|a, b| a.cmp(b).reverse());
        cards_count[0] += jokers;
        match cards_count.as_slice() {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            [1, ..] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<(Hand, u32)>> {
    let (_, hands_and_bids) = parser::hands_and_bids(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(hands_and_bids)
}

#[aoc(day7, part1)]
fn part1(hands_and_bids: &[(Hand, u32)]) -> u32 {
    hands_and_bids
        .iter()
        .map(|(hand, bid)| (hand, bid, hand.hand_type()))
        .sorted_by(|(hand_a, _, hand_a_type), (hand_b, _, hand_b_type)| {
            hand_a_type
                .cmp(hand_b_type)
                .then_with(|| hand_a.cards.cmp(&hand_b.cards))
        })
        .enumerate()
        .map(|(i, (_, &bid, _))| (i as u32 + 1) * bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(hands_and_bids: &[(Hand, u32)]) -> u32 {
    hands_and_bids
        .iter()
        .map(|(hand, bid)| (hand, bid, hand.hand_type_with_jokers()))
        .sorted_by(|(hand_a, _, hand_a_type), (hand_b, _, hand_b_type)| {
            hand_a_type.cmp(hand_b_type).then_with(|| {
                hand_a
                    .cards
                    .iter()
                    .zip(&hand_b.cards)
                    .map(|(a, b)| a.cmp_with_joker(b))
                    .find(|ord| ord.is_ne())
                    .unwrap_or(Ordering::Equal)
            })
        })
        .enumerate()
        .map(|(i, (_, &bid, _))| (i as u32 + 1) * bid)
        .sum()
}

mod parser {
    use crate::day7::{Card, Hand};
    use crate::nom_parser::number;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take};
    use nom::character::complete::{line_ending, u8};
    use nom::combinator::{map, map_parser};
    use nom::multi::{many_m_n, separated_list1};
    use nom::sequence::{pair, preceded};
    use nom::IResult;

    fn card(input: &str) -> IResult<&str, Card> {
        alt((
            map(tag("A"), |_| Card::A),
            map(tag("K"), |_| Card::K),
            map(tag("Q"), |_| Card::Q),
            map(tag("J"), |_| Card::J),
            map(tag("T"), |_| Card::T),
            map(map_parser(take(1usize), u8), Card::N),
        ))(input)
    }

    pub fn hands_and_bids(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
        separated_list1(
            line_ending,
            pair(
                map(many_m_n(5, 5, card), |cards| Hand { cards }),
                preceded(tag(" "), number),
            ),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 5905);
    }
}
