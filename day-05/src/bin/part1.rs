use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct Map {
    destination: u64,
    source: u64,
    range: u64,
}

impl Map {
    fn apply(&self, value: u64) -> Option<u64> {
        if value >= self.source && value < self.source + self.range {
            Some(self.destination + value - self.source)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Data {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn map(input: &str) -> IResult<&str, Map> {
    let (input, numbers) = separated_list1(tag(" "), complete::u64)(input)?;
    assert_eq!(numbers.len(), 3);
    Ok((
        input,
        Map {
            destination: numbers[0],
            source: numbers[1],
            range: numbers[2],
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), complete::u64)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("seed-to-soil map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, seed_to_soil) = separated_list1(line_ending, map)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("soil-to-fertilizer map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, soil_to_fertilizer) = separated_list1(line_ending, map)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("fertilizer-to-water map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, fertilizer_to_water) = separated_list1(line_ending, map)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("water-to-light map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, water_to_light) = separated_list1(line_ending, map)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("light-to-temperature map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, light_to_temperature) = separated_list1(line_ending, map)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("temperature-to-humidity map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, temperature_to_humidity) = separated_list1(line_ending, map)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("humidity-to-location map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, humidity_to_location) = separated_list1(line_ending, map)(input)?;

    let data = Data {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    Ok((input, data))
}

fn find_map_value(value: u64, maps: &Vec<Map>) -> Option<u64> {
    for map in maps {
        if let Some(result) = map.apply(value) {
            return Some(result);
        }
    }
    None
}

fn part1(input: &str) -> u64 {
    let mut min_loc = u64::MAX;
    let (input, data) = parse_input(input).unwrap();

    for seed in data.seeds {
        let soil = find_map_value(seed, &data.seed_to_soil)
            .or(Some(seed))
            .unwrap();
        let fertilizer = find_map_value(soil, &data.soil_to_fertilizer)
            .or(Some(soil))
            .unwrap();
        let water = find_map_value(fertilizer, &data.fertilizer_to_water)
            .or(Some(fertilizer))
            .unwrap();
        let light = find_map_value(water, &data.water_to_light)
            .or(Some(water))
            .unwrap();
        let temperature = find_map_value(light, &data.light_to_temperature)
            .or(Some(light))
            .unwrap();
        let humidity = find_map_value(temperature, &data.temperature_to_humidity)
            .or(Some(temperature))
            .unwrap();
        let location = find_map_value(humidity, &data.humidity_to_location)
            .or(Some(humidity))
            .unwrap();
        if location < min_loc {
            min_loc = location;
        }
    }

    min_loc
}

fn main() {
    let input = include_str!("../../input.txt");

    println!("part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part1(test), 35);
    }
}
