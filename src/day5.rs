use crate::new_type;
use crate::new_type::NewType;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::iter::{once, once_with};
use std::ops::Range;

struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: Vec<(Soil, Seed, u64)>,
    soil_to_fertilizer: Vec<(Fertilizer, Soil, u64)>,
    fertilizer_to_water: Vec<(Water, Fertilizer, u64)>,
    water_to_light: Vec<(Light, Water, u64)>,
    light_to_temperature: Vec<(Temperature, Light, u64)>,
    temperature_to_humidity: Vec<(Humidity, Temperature, u64)>,
    humidity_to_location: Vec<(Location, Humidity, u64)>,
}

new_type! {
    struct Seed(u64);
    struct Soil(u64);
    struct Fertilizer(u64);
    struct Water(u64);
    struct Light(u64);
    struct Temperature(u64);
    struct Humidity(u64);
    struct Location(u64);
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Almanac> {
    let (_, almanac) = parser::almanac(input).map_err(|err| {
        err.map(|error| nom::error::Error::new(error.input.to_string(), error.code))
    })?;
    Ok(almanac)
}

fn map<A: NewType<u64>, B: NewType<u64>>(a: A, maps: &[(B, A, u64)]) -> B {
    maps.iter()
        .find_map(|&(dest, source, length)| {
            (source..source + length)
                .contains(&a)
                .then(|| B::from((a - source).into()) + dest)
        })
        .unwrap_or_else(|| B::from(a.into()))
}

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> Location {
    let Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    } = almanac;

    seeds
        .iter()
        .copied()
        .map(|s| map(s, seed_to_soil))
        .map(|s| map(s, soil_to_fertilizer))
        .map(|s| map(s, fertilizer_to_water))
        .map(|s| map(s, water_to_light))
        .map(|s| map(s, light_to_temperature))
        .map(|s| map(s, temperature_to_humidity))
        .map(|s| map(s, humidity_to_location))
        .min()
        .unwrap()
}

fn overlaps<T: NewType<u64>>(a: Range<T>, b: Range<T>) -> Option<Range<T>> {
    // A1.......A2  B1....B2  => None
    // B1....B2   A1.......A2 => None
    // A1.....B1..A2..B2      => B1..A2
    // B1...A1..B2....A2      => A1..B2
    // A1...B1....B2....A2    => B1..B2
    // B1...A1....A2....B2    => A1..A2

    match (a.end.cmp(&b.start), a.start.cmp(&b.end)) {
        (Ordering::Less | Ordering::Equal, _) | (_, Ordering::Greater | Ordering::Equal) => None,
        (Ordering::Greater, Ordering::Less) => Some(T::max(a.start, b.start)..T::min(a.end, b.end)),
    }
}

fn map_range<A: NewType<u64>, B: NewType<u64>>(
    range: Range<A>,
    maps: &[(B, A, u64)],
) -> Vec<Range<B>> {
    let range = RefCell::new(range);
    maps.iter()
        .filter_map(|&(dest, source, length)| {
            overlaps(range.borrow().clone(), source..source + length)
                .map(|overlap| (overlap, source, dest))
        })
        .flat_map(|(overlap, source, dest)| {
            let before = B::from(range.borrow().start.into())..B::from(overlap.start.into());
            let mapped_start = B::from((overlap.start - source).into()) + dest;
            let mapped_end = B::from((overlap.end - source).into()) + dest;

            range.borrow_mut().start = overlap.end;
            once(before).chain(once(mapped_start..mapped_end))
        })
        .chain(once_with(|| {
            B::from(range.borrow().start.into())..B::from(range.borrow().end.into())
        }))
        .filter(|range| !range.is_empty())
        .collect()
}

#[aoc(day5, part2)]
fn part2(almanac: &Almanac) -> Location {
    let Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    } = almanac;
    seeds
        .chunks(2)
        .map(|seeds| {
            let base = seeds[0];
            let length = seeds[1];
            base..base + length
        })
        .flat_map(|r| map_range(r, seed_to_soil))
        .flat_map(|r| map_range(r, soil_to_fertilizer))
        .flat_map(|r| map_range(r, fertilizer_to_water))
        .flat_map(|r| map_range(r, water_to_light))
        .flat_map(|r| map_range(r, light_to_temperature))
        .flat_map(|r| map_range(r, temperature_to_humidity))
        .flat_map(|r| map_range(r, humidity_to_location))
        .map(|r| r.start)
        .min()
        .unwrap()
}

mod parser {
    use crate::day5::Almanac;
    use crate::nom_parser::number;
    use nom::bytes::complete::tag;
    use nom::multi::separated_list1;
    use nom::IResult;
    use std::str::FromStr;

    fn map_tuple<A: FromStr, B: FromStr, C: FromStr>(input: &str) -> IResult<&str, (A, B, C)> {
        let (input, a) = number(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, b) = number(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, c) = number(input)?;

        Ok((input, (a, b, c)))
    }

    pub fn almanac(input: &str) -> IResult<&str, Almanac> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(tag(" "), number)(input)?;

        let (input, _) = tag("\n\nseed-to-soil map:\n")(input)?;
        let (input, mut seed_to_soil) = separated_list1(tag("\n"), map_tuple)(input)?;
        seed_to_soil.sort_by_key(|&(_, k, _)| k);

        let (input, _) = tag("\n\nsoil-to-fertilizer map:\n")(input)?;
        let (input, mut soil_to_fertilizer) = separated_list1(tag("\n"), map_tuple)(input)?;
        soil_to_fertilizer.sort_by_key(|&(_, k, _)| k);

        let (input, _) = tag("\n\nfertilizer-to-water map:\n")(input)?;
        let (input, mut fertilizer_to_water) = separated_list1(tag("\n"), map_tuple)(input)?;
        fertilizer_to_water.sort_by_key(|&(_, k, _)| k);

        let (input, _) = tag("\n\nwater-to-light map:\n")(input)?;
        let (input, mut water_to_light) = separated_list1(tag("\n"), map_tuple)(input)?;
        water_to_light.sort_by_key(|&(_, k, _)| k);

        let (input, _) = tag("\n\nlight-to-temperature map:\n")(input)?;
        let (input, mut light_to_temperature) = separated_list1(tag("\n"), map_tuple)(input)?;
        light_to_temperature.sort_by_key(|&(_, k, _)| k);

        let (input, _) = tag("\n\ntemperature-to-humidity map:\n")(input)?;
        let (input, mut temperature_to_humidity) = separated_list1(tag("\n"), map_tuple)(input)?;
        temperature_to_humidity.sort_by_key(|&(_, k, _)| k);

        let (input, _) = tag("\n\nhumidity-to-location map:\n")(input)?;
        let (input, mut humidity_to_location) = separated_list1(tag("\n"), map_tuple)(input)?;
        humidity_to_location.sort_by_key(|&(_, k, _)| k);

        Ok((
            input,
            Almanac {
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), Location(35));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), Location(46));
    }
}
