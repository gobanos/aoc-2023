use crate::nom_parser::to_result;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

#[derive(Debug, Clone)]
struct ConditionRecord {
    entries: Vec<ConditionEntry>,
}

#[derive(Debug, Clone)]
struct ConditionEntry {
    statuses: Vec<Status>,
    damaged_groups: Vec<u8>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

// Pointer-based hashkey
#[derive(Eq, PartialEq, Hash)]
struct HashKey {
    ptr1: usize,
    len1: usize,
    ptr2: usize,
    len2: usize,
}

impl HashKey {
    fn new(statuses: &[Status], groups: &[u8], taken: u8) -> Self {
        HashKey {
            ptr1: statuses.as_ptr() as usize + (taken & 0xF) as usize,
            len1: statuses.len(),
            ptr2: groups.as_ptr() as usize + (taken >> 4) as usize,
            len2: groups.len(),
        }
    }
}

fn solve(statuses: &[Status], groups: &[u8], taken: u8, cache: &mut HashMap<HashKey, u64>) -> u64 {
    if let Some(&cached_value) = cache.get(&HashKey::new(statuses, groups, taken)) {
        cached_value
    } else {
        let continue_if_matches =
            |rest: &[Status], cache: &mut HashMap<HashKey, u64>| match (taken, groups) {
                (0, [0, groups @ ..]) => solve(rest, groups, 0, cache),
                (0, groups) => solve(rest, groups, 0, cache),
                (taken, [group, groups @ ..]) if taken == *group => solve(rest, groups, 0, cache),
                _ => 0,
            };
        let computed_value = match statuses {
            [Status::Operational, rest @ ..] => continue_if_matches(rest, cache),
            [Status::Damaged, rest @ ..] => solve(rest, groups, taken + 1, cache),
            [Status::Unknown, rest @ ..] => {
                solve(rest, groups, taken + 1, cache) + continue_if_matches(rest, cache)
            }
            [] => match (taken, groups) {
                (0, []) => 1,
                (taken, [group]) if taken == *group => 1,
                _ => 0,
            },
        };
        cache.insert(HashKey::new(statuses, groups, taken), computed_value);
        computed_value
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<ConditionRecord> {
    to_result(parser::condition_record(input))
}

#[aoc(day12, part1)]
fn part1(record: &ConditionRecord) -> u64 {
    record
        .entries
        .iter()
        .map(|entry| {
            let mut cache = HashMap::new();
            solve(&entry.statuses, &entry.damaged_groups, 0, &mut cache)
        })
        .sum()
}

#[aoc(day12, part2)]
fn part2(record: &ConditionRecord) -> u64 {
    record
        .entries
        .par_iter()
        .map(|entry| {
            let statuses = entry
                .statuses
                .iter()
                .copied()
                .chain(once(Status::Unknown))
                .cycle()
                .take(entry.statuses.len() * 5 + 4)
                .collect_vec();
            let damaged_groups = entry
                .damaged_groups
                .iter()
                .copied()
                .cycle()
                .take(entry.damaged_groups.len() * 5)
                .collect_vec();
            let mut cache = HashMap::new();
            solve(&statuses, &damaged_groups, 0, &mut cache)
        })
        .sum()
}

mod parser {
    use crate::day12::{ConditionEntry, ConditionRecord, Status};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, u8};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::separated_pair;
    use nom::IResult;

    fn statuses(input: &str) -> IResult<&str, Vec<Status>> {
        many1(alt((
            map(tag("."), |_| Status::Operational),
            map(tag("#"), |_| Status::Damaged),
            map(tag("?"), |_| Status::Unknown),
        )))(input)
    }

    pub fn condition_record(input: &str) -> IResult<&str, ConditionRecord> {
        map(
            separated_list1(
                newline,
                map(
                    separated_pair(statuses, tag(" "), separated_list1(tag(","), u8)),
                    |(statuses, damaged_groups)| ConditionEntry {
                        statuses,
                        damaged_groups,
                    },
                ),
            ),
            |entries| ConditionRecord { entries },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_example() {
        for (i, (line, expected)) in EXAMPLE.lines().zip([1, 4, 1, 1, 4, 10]).enumerate() {
            assert_eq!(
                part1(&parse(line).unwrap()),
                expected,
                "Failed at line {} [{line}]",
                i + 1
            );
        }
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 21);
    }

    #[test]
    fn part2_example() {
        for (i, (line, expected)) in EXAMPLE
            .lines()
            .zip([1, 16384, 1, 16, 2500, 506250])
            .enumerate()
        {
            assert_eq!(
                part2(&parse(line).unwrap()),
                expected,
                "Failed at line {} [{line}]",
                i + 1
            );
        }
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 525152);
    }
}
