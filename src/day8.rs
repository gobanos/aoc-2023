use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use num::integer::lcm;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn into_usize(self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Node([u8; 3]);

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node")
            .field(&String::from_utf8_lossy(&self.0))
            .finish()
    }
}

const AAA: Node = Node([b'A', b'A', b'A']);
const ZZZ: Node = Node([b'Z', b'Z', b'Z']);

struct Map {
    directions: Vec<Direction>,
    nodes: Vec<(Node, [Node; 2])>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Map> {
    let (_, map) = parser::map_input(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(map)
}

#[aoc(day8, part1)]
fn part1(map: &Map) -> usize {
    let nodes: HashMap<_, _> = map.nodes.iter().copied().collect();

    let mut current_node = AAA;
    for (i, direction) in map.directions.iter().cycle().enumerate() {
        current_node = nodes[&current_node][direction.into_usize()];
        if current_node == ZZZ {
            return i + 1;
        }
    }
    unreachable!()
}

#[aoc(day8, part2)]
fn part2(map: &Map) -> usize {
    let nodes: HashMap<_, _> = map.nodes.iter().copied().collect();

    nodes.keys().filter(|node| matches!(node, Node([_, _, b'A'])))
        .copied()
        .map(|mut current_node| {
            for (i, direction) in map.directions.iter().cycle().enumerate() {
                current_node = nodes[&current_node][direction.into_usize()];
                if matches!(current_node, Node([_, _, b'Z'])) {
                    return i + 1;
                }
            }
            unreachable!()
        })
        .fold(1, lcm)
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take};
    use nom::character::complete::newline;
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::{many1, many1_count, separated_list1};
    use nom::sequence::{delimited, pair, separated_pair};
    use crate::day8::{Direction, Map, Node};

    fn node(input: &str) -> IResult<&str, Node> {
        map_res(take(3usize), |s: &str| s.as_bytes().try_into().map(Node))(input)
    }

    pub fn map_input(input: &str) -> IResult<&str, Map> {
        let (input, directions) = many1(alt((
            map(tag("L"), |_| Direction::Left),
            map(tag("R"), |_| Direction::Right),
        )))(input)?;
        let (input, _) = many1_count(newline)(input)?;

        let (input, nodes) = separated_list1(newline, pair(
            node,
            delimited(
                tag(" = ("),
                map(
                    separated_pair(
                        node,
                        tag(", "),
                        node
                    ),
                    |(left, right)| [left, right]
                ),
                tag(")"),
            )
        ))(input)?;

        Ok((input, Map {
            directions,
            nodes,
        }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_RL: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2_LLR: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1_RL).unwrap()), 2);
        assert_eq!(part1(&parse(EXAMPLE2_LLR).unwrap()), 6);
    }

    const EXAMPLE2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2).unwrap()), 6);
    }
}
