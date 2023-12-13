use crate::new_type;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::successors;

new_type! {
    struct X(usize);
    struct Y(usize);
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<(X, Y)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &c)| c == b'#')
                .map(move |(x, _)| (X(x), Y(y)))
        })
        .collect()
}

fn solve<const VOID: u64>(galaxies: &[(X, Y)]) -> u64 {
    let mut seen_x = [false; 140];
    let mut seen_y = [false; 140];
    for &(x, y) in galaxies {
        seen_x[x.0] = true;
        seen_y[y.0] = true;
    }

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(&(ax, ay), &(bx, by))| -> u64 {
            let min_x = ax.min(bx);
            let diff_x = ax.max(bx) - min_x;

            let min_y = ay.min(by);
            let diff_y = ay.max(by) - min_y;

            successors(Some(min_x), |&x| Some(x + 1))
                .map(|x| if seen_x[x.0] { 1u64 } else { VOID })
                .take(diff_x.into())
                .sum::<u64>()
                + successors(Some(min_y), |&y| Some(y + 1))
                    .map(|y| if seen_y[y.0] { 1u64 } else { VOID })
                    .take(diff_y.into())
                    .sum::<u64>()
        })
        .sum()
}

#[aoc(day11, part1)]
fn part1(galaxies: &[(X, Y)]) -> u64 {
    solve::<2>(galaxies)
}

#[aoc(day11, part2)]
fn part2(galaxies: &[(X, Y)]) -> u64 {
    solve::<1_000_000>(galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 374);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve::<10>(&parse(EXAMPLE)), 1030);
        assert_eq!(solve::<100>(&parse(EXAMPLE)), 8410);
    }
}
