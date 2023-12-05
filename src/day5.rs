use crate::new_type;
use crate::new_type::NewType;
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

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
            if (source..source + length).contains(&a) {
                Some(B::from((a - source).into()) + dest)
            } else {
                None
            }
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

    seeds.iter().copied()
        .map(|s| map(s, seed_to_soil))
        .map(|s| map(s, soil_to_fertilizer))
        .map(|s| map(s, fertilizer_to_water))
        .map(|s| map(s, water_to_light))
        .map(|s| map(s, light_to_temperature))
        .map(|s| map(s, temperature_to_humidity))
        .map(|s| map(s, humidity_to_location))
        .min().unwrap()
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
    seeds.chunks(2)
        .flat_map(|seeds| {
            let base: u64 = seeds[0].into();
            let length: u64 = seeds[1].into();
            (base..base+length)
                .map(Seed)
        })
        .map(|s| map(s, seed_to_soil))
        .map(|s| map(s, soil_to_fertilizer))
        .map(|s| map(s, fertilizer_to_water))
        .map(|s| map(s, water_to_light))
        .map(|s| map(s, light_to_temperature))
        .map(|s| map(s, temperature_to_humidity))
        .map(|s| map(s, humidity_to_location))
        .min().unwrap()
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
        let (input, seed_to_soil) = separated_list1(tag("\n"), map_tuple)(input)?;

        let (input, _) = tag("\n\nsoil-to-fertilizer map:\n")(input)?;
        let (input, soil_to_fertilizer) = separated_list1(tag("\n"), map_tuple)(input)?;

        let (input, _) = tag("\n\nfertilizer-to-water map:\n")(input)?;
        let (input, fertilizer_to_water) = separated_list1(tag("\n"), map_tuple)(input)?;

        let (input, _) = tag("\n\nwater-to-light map:\n")(input)?;
        let (input, water_to_light) = separated_list1(tag("\n"), map_tuple)(input)?;

        let (input, _) = tag("\n\nlight-to-temperature map:\n")(input)?;
        let (input, light_to_temperature) = separated_list1(tag("\n"), map_tuple)(input)?;

        let (input, _) = tag("\n\ntemperature-to-humidity map:\n")(input)?;
        let (input, temperature_to_humidity) = separated_list1(tag("\n"), map_tuple)(input)?;

        let (input, _) = tag("\n\nhumidity-to-location map:\n")(input)?;
        let (input, humidity_to_location) = separated_list1(tag("\n"), map_tuple)(input)?;

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
