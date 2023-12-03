use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Either, Itertools};

#[derive(Copy, Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Debug)]
struct Span {
    position: Position,
    length: usize,
}

#[derive(Copy, Clone, Debug)]
struct Number {
    value: u32,
    span: Span,
}

impl Number {
    fn is_adjacent_to(&self, symbol: &Symbol) -> bool {
        let Number {
            span:
                Span {
                    position: Position { row, col },
                    length,
                },
            ..
        } = *self;
        (row.saturating_sub(1)..=(row + 1)).contains(&symbol.position.row)
            && (col.saturating_sub(1)..=(col + length)).contains(&symbol.position.col)
    }
}

#[derive(Copy, Clone, Debug)]
struct Symbol {
    char: char,
    position: Position,
}

#[derive(Copy, Clone, Debug)]
enum Element {
    Number(Number),
    Symbol(Symbol),
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<Element>> {
    let (_, elements) = parser::elements(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(elements)
}

#[aoc(day3, part1)]
fn part1(input: &[Element]) -> u32 {
    let (numbers, symbols): (Vec<Number>, Vec<Symbol>) =
        input.iter().partition_map(|element| match &element {
            Element::Number(number) => Either::Left(number),
            Element::Symbol(symbol) => Either::Right(symbol),
        });

    numbers
        .iter()
        .filter(|number| symbols.iter().any(|symbol| number.is_adjacent_to(symbol)))
        .map(|number| number.value)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Element]) -> u32 {
    let (numbers, symbols): (Vec<Number>, Vec<Symbol>) =
        input.iter().partition_map(|element| match &element {
            Element::Number(number) => Either::Left(number),
            Element::Symbol(symbol) => Either::Right(symbol),
        });

    symbols
        .iter()
        .filter(|&&Symbol { char, .. }| char == '*')
        .filter_map(|symbol| {
            let Some((a, b)) = numbers
                .iter()
                .filter(|number| number.is_adjacent_to(symbol))
                .collect_tuple()
            else {
                return None;
            };
            Some(a.value * b.value)
        })
        .sum()
}

mod parser {
    use crate::day3::{Element, Number, Position, Span, Symbol};
    use crate::nom_parser::number;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::anychar;
    use nom::multi::{fold_many0, many0};
    use nom::{IResult, Offset, Parser};

    fn dots(input: &str) -> IResult<&str, ()> {
        fold_many0(tag("."), || (), |acc, _| acc)(input)
    }

    fn element<'a, 'line: 'a>(
        input: &'a str,
        row: usize,
        line: &'line str,
    ) -> IResult<&'a str, Element> {
        let (input, ()) = dots(input)?;
        let position = Position {
            row,
            col: line.offset(input),
        };
        let (input, mut element) = alt((
            number.map(|value| {
                Element::Number(Number {
                    value,
                    span: Span {
                        position,
                        length: 0,
                    },
                })
            }),
            anychar.map(|char| Element::Symbol(Symbol { char, position })),
        ))(input)?;
        if let Element::Number(Number {
            span: Span { length, .. },
            ..
        }) = &mut element
        {
            *length = line.offset(input) - position.col;
        }
        Ok((input, element))
    }

    pub fn elements(input: &str) -> IResult<&str, Vec<Element>> {
        let mut all_elements = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let (_, line_elements) = many0(|input| element(input, row, line))(line)?;
            all_elements.extend(line_elements);
        }
        Ok(("", all_elements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 467835);
    }
}
