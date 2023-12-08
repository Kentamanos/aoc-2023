use std::{collections::HashMap, ops::Deref};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use prime_factorization::Factorization;

#[derive(Debug)]
struct MapEntry {
    id: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    steps: Vec<char>,
    left_right_map: HashMap<String, MapEntry>,
}

fn left_right(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = tag("(")(input)?;
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (left, right)))
}

fn map_entry(input: &str) -> IResult<&str, MapEntry> {
    let (input, (id, (l, r))) = separated_pair(alphanumeric1, tag(" = "), left_right)(input)?;

    Ok((
        input,
        MapEntry {
            id: id.to_string(),
            left: l.to_string(),
            right: r.to_string(),
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, steps) = many1(one_of("LR"))(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;

    let (input, map_entries) = separated_list1(newline, map_entry)(input)?;

    let mut left_right_map: HashMap<String, MapEntry> = HashMap::new();
    for entry in map_entries {
        left_right_map.insert(entry.id.clone(), entry);
    }

    Ok((
        input,
        Map {
            steps,
            left_right_map,
        },
    ))
}
fn get_repetition_frequency(start: String, map: &Map) -> u64 {
    let mut count = 0;
    let mut it = map.steps.iter().cycle();
    let mut location = start.clone();

    loop {
        let step = it.next().unwrap();
        count += 1;

        let entry = map.left_right_map.get(location.deref()).unwrap();
        let next = match step {
            'L' => &entry.left,
            'R' => &entry.right,
            _ => panic!("unexpected step"),
        };

        // TODO: Get rid of cloning here
        location = next.clone();

        if location.ends_with('Z') {
            break;
        }
    }

    count
}

// Returns the prime factors grouped by count
fn prime_factorization(input: u64) -> HashMap<u64, usize> {
    let mut counted_factors = HashMap::<u64, usize>::new();
    let factor_repr = Factorization::run(input);

    for factor in factor_repr.factors {
        let count = counted_factors.entry(factor).or_insert(0);
        *count += 1;
    }

    counted_factors
}

// Returns the least common multiple of the prime factors
fn least_common_multiple(input: Vec<HashMap<u64, usize>>) -> u64 {
    let mut lcm_map = HashMap::<u64, usize>::new();

    for value_factors in input {
        for (factor, count) in value_factors {
            let current_count = lcm_map.entry(factor).or_insert(0);
            if count > *current_count {
                *current_count = count;
            }
        }
    }

    let mut lcm = 1u64;
    for (factor, count) in lcm_map {
        lcm *= factor.pow(count as u32);
    }

    lcm
}

fn part2(text: &str) -> u64 {
    let (input, map) = parse_map(text).unwrap();
    assert_eq!(input, "");

    let starting_spots = map
        .left_right_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let mut prime_factors = Vec::<HashMap<u64, usize>>::new();
    starting_spots.iter().for_each(|spot| {
        let freq = get_repetition_frequency(spot.to_string(), &map);
        let primes = prime_factorization(freq);
        prime_factors.push(primes);
    });

    least_common_multiple(prime_factors)
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let test = include_str!("../../test2.txt");
        assert_eq!(part2(test), 6);
    }
}
