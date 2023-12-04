use aho_corasick::AhoCorasick;
use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};

fn as_numeric(c: &u8) -> Option<u32> {
    if c.is_ascii_digit() {
        Some((c - b'0').into())
    } else {
        None
    }
}

#[aoc_generator(day1, part1)]
fn parse_part1(input: &[u8]) -> Vec<(u32, u32)> {
    input
        .split(|&c| c == b'\n')
        .map(|line| {
            (
                line.iter().find_map(as_numeric).unwrap(),
                line.iter().rev().find_map(as_numeric).unwrap(),
            )
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| a * 10 + b).sum()
}

const NUMBERS_IN_ENGLISH: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc_generator(day1, part2)]
fn parse_part2(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut findings = Vec::new();
            for (index, number_str) in NUMBERS_IN_ENGLISH.iter().enumerate() {
                if let Some(pos) = line.find(number_str) {
                    findings.push((pos, index as u32 + 1));
                }
            }
            for number in 0..10 {
                if let Some(pos) = line.find::<char>((b'0' + number).into()) {
                    findings.push((pos, number.into()));
                }
            }
            let &(_, first) = findings.iter().min_by_key(|(pos, _)| pos).unwrap();

            let mut findings = Vec::new();
            for (index, number_str) in NUMBERS_IN_ENGLISH.iter().enumerate() {
                if let Some(pos) = line.rfind(number_str) {
                    findings.push((pos, index as u32 + 1));
                }
            }
            for number in 0..10 {
                if let Some(pos) = line.rfind::<char>((b'0' + number).into()) {
                    findings.push((pos, number.into()));
                }
            }
            let &(_, last) = findings.iter().max_by_key(|(pos, _)| pos).unwrap();

            (first, last)
        })
        .collect()
}

#[aoc(day1, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| a * 10 + b).sum()
}

const NUMBERS_IN_ENGLISH_AND_LITERAL: &[&str] = &[
    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
    "eight", "8", "nine", "9",
];

#[aoc_generator(day1, part2, aho)]
fn parse_part2_aho_corasick(input: &str) -> Result<Vec<(u32, u32)>> {
    let ac = AhoCorasick::new(NUMBERS_IN_ENGLISH_AND_LITERAL)?;

    input
        .lines()
        .map(|line| {
            let mut iter = ac.find_iter(line);
            let first = iter
                .next()
                .ok_or_else(|| anyhow!("Could not find any number in {line}"))?;
            let last = iter.last().unwrap_or(first);
            Ok((
                first.pattern().as_u32() / 2 + 1,
                last.pattern().as_u32() / 2 + 1,
            ))
        })
        .collect()
}

#[aoc(day1, part2, aho)]
fn part2_aho_corasick(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| a * 10 + b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &[u8] = b"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_PART1)), 142);
    }

    const EXAMPLE_PART2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_PART2)), 281);
        assert_eq!(
            part2_aho_corasick(&parse_part2_aho_corasick(EXAMPLE_PART2).unwrap()),
            281
        );
    }
}
