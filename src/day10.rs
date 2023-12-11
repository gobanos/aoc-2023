use crate::nom_parser::to_result;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;
use std::iter::successors;
use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<Tile>>,
}

impl Maze {
    fn start_position(&self) -> MazeIndex {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(row, tiles)| {
                tiles.iter().enumerate().find_map(|(col, &tile)| {
                    (tile == Tile::StartingPosition).then_some(MazeIndex { row, col })
                })
            })
            .unwrap()
    }
}

impl Index<MazeIndex> for Maze {
    type Output = Tile;

    fn index(&self, index: MazeIndex) -> &Self::Output {
        &self.grid[index.row][index.col]
    }
}

impl IndexMut<MazeIndex> for Maze {
    fn index_mut(&mut self, index: MazeIndex) -> &mut Self::Output {
        &mut self.grid[index.row][index.col]
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct MazeIndex {
    row: usize,
    col: usize,
}

impl Add<Direction> for MazeIndex {
    type Output = MazeIndex;

    fn add(self, direction: Direction) -> Self::Output {
        let MazeIndex { row, col } = self;
        match direction {
            Direction::North => MazeIndex { row: row - 1, col },
            Direction::South => MazeIndex { row: row + 1, col },
            Direction::West => MazeIndex { col: col - 1, row },
            Direction::East => MazeIndex { col: col + 1, row },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

// Tests are sensitive to this ordering, there is no overflow check yet
const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::North,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

impl Tile {
    fn walk_tile(&self, from: Direction) -> Option<Direction> {
        match (self, from) {
            (Tile::Vertical, Direction::North)
            | (Tile::NorthEast, Direction::West)
            | (Tile::NorthWest, Direction::East) => Some(Direction::North),
            (Tile::Vertical, Direction::South)
            | (Tile::SouthEast, Direction::West)
            | (Tile::SouthWest, Direction::East) => Some(Direction::South),
            (Tile::Horizontal, Direction::West)
            | (Tile::NorthWest, Direction::South)
            | (Tile::SouthWest, Direction::North) => Some(Direction::West),
            (Tile::Horizontal, Direction::East)
            | (Tile::NorthEast, Direction::South)
            | (Tile::SouthEast, Direction::North) => Some(Direction::East),
            (Tile::Ground | Tile::StartingPosition, _)
            | (Tile::Vertical, Direction::East | Direction::West)
            | (Tile::Horizontal, Direction::North | Direction::South)
            | (Tile::NorthEast, Direction::North | Direction::East)
            | (Tile::NorthWest, Direction::North | Direction::West)
            | (Tile::SouthWest, Direction::South | Direction::West)
            | (Tile::SouthEast, Direction::South | Direction::East) => None,
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Maze> {
    to_result(parser::maze(input))
}

#[aoc(day10, part1)]
fn part1(maze: &Maze) -> usize {
    let starting_position = maze.start_position();
    let (first_cell, one_direction) = ALL_DIRECTIONS
        .into_iter()
        .find_map(|dir| {
            let first_pos = starting_position + dir;
            maze[first_pos].walk_tile(dir).map(|_| (first_pos, dir))
        })
        .unwrap();

    let walk = successors(Some((first_cell, one_direction)), |&(index, direction)| {
        maze[index]
            .walk_tile(direction)
            .map(|new_dir| (index + new_dir, new_dir))
    });
    walk.count() / 2
}

#[aoc(day10, part2)]
fn part2(maze: &Maze) -> usize {
    let mut maze = maze.clone();
    let starting_position = maze.start_position();
    let (first_cell, first_direction) = ALL_DIRECTIONS
        .into_iter()
        .find_map(|dir| {
            let first_pos = starting_position + dir;
            maze[first_pos].walk_tile(dir).map(|_| (first_pos, dir))
        })
        .unwrap();

    let walk = successors(
        Some((first_cell, first_direction)),
        |&(index, direction)| {
            maze[index]
                .walk_tile(direction)
                .map(|new_dir| (index + new_dir, new_dir))
        },
    );
    let mut loop_indices = BTreeSet::new();
    let (_, last_direction) = walk
        .inspect(|&(index, _)| {
            loop_indices.insert(index);
        })
        .last()
        .unwrap();
    maze[starting_position] = match (first_direction, last_direction) {
        (Direction::East | Direction::West, Direction::East | Direction::West) => Tile::Horizontal,
        (Direction::North | Direction::South, Direction::North | Direction::South) => {
            Tile::Vertical
        }
        (Direction::North | Direction::East, Direction::South | Direction::West) => Tile::NorthEast,
        (Direction::North | Direction::West, Direction::South | Direction::East) => Tile::NorthWest,
        (Direction::South | Direction::East, Direction::North | Direction::West) => Tile::SouthEast,
        (Direction::South | Direction::West, Direction::North | Direction::East) => Tile::SouthWest,
    };

    maze.grid
        .iter()
        .enumerate()
        .flat_map(|(row, tiles)| {
            tiles
                .iter()
                .enumerate()
                .map(move |(col, _)| MazeIndex { row, col })
        })
        .filter(|index| !loop_indices.contains(index))
        .filter(|&MazeIndex { col, row }| {
            (0..col)
                .rev()
                .map(|col| MazeIndex { col, row })
                .filter(|index| loop_indices.contains(index))
                .fold((0, None), |(crossings, partial_direction), index| {
                    match (maze[index], partial_direction) {
                        (Tile::Vertical, _) => (crossings + 1, None),
                        (Tile::Horizontal, partial_direction) => (crossings, partial_direction),
                        (Tile::NorthWest, _) => (crossings, Some(Direction::North)),
                        (Tile::SouthWest, _) => (crossings, Some(Direction::South)),
                        (Tile::NorthEast, Some(Direction::North)) => (crossings, None),
                        (Tile::NorthEast, _) => (crossings + 1, None),
                        (Tile::SouthEast, Some(Direction::South)) => (crossings, None),
                        (Tile::SouthEast, _) => (crossings + 1, None),
                        (Tile::Ground, _) => unreachable!(),
                        (Tile::StartingPosition, _) => unreachable!(),
                    }
                })
                .0
                % 2
                == 1
        })
        .count()
}

mod parser {
    use crate::day10::{Maze, Tile};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::combinator::map;
    use nom::multi::{many1, separated_list1};
    use nom::IResult;

    fn tile(input: &str) -> IResult<&str, Tile> {
        alt((
            map(tag("|"), |_| Tile::Vertical),
            map(tag("-"), |_| Tile::Horizontal),
            map(tag("L"), |_| Tile::NorthEast),
            map(tag("J"), |_| Tile::NorthWest),
            map(tag("7"), |_| Tile::SouthWest),
            map(tag("F"), |_| Tile::SouthEast),
            map(tag("."), |_| Tile::Ground),
            map(tag("S"), |_| Tile::StartingPosition),
        ))(input)
    }

    pub fn maze(input: &str) -> IResult<&str, Maze> {
        map(separated_list1(newline, many1(tile)), |grid| Maze { grid })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQUARE_LOOP: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const SQUARE_LOOP_OBFUSCATED: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const COMPLEX_LOOP: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const COMPLEX_LOOP_OBFUSCATED: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SQUARE_LOOP).unwrap()), 4);
        assert_eq!(part1(&parse(SQUARE_LOOP_OBFUSCATED).unwrap()), 4);
        assert_eq!(part1(&parse(COMPLEX_LOOP).unwrap()), 8);
        assert_eq!(part1(&parse(COMPLEX_LOOP_OBFUSCATED).unwrap()), 8);
    }

    const LOOP_4_TILES: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const LOOP_8_TILES: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const LOOP_10_TILES: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(LOOP_4_TILES).unwrap()), 4);
        assert_eq!(part2(&parse(LOOP_8_TILES).unwrap()), 8);
        assert_eq!(part2(&parse(LOOP_10_TILES).unwrap()), 10);
    }
}
