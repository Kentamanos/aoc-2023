use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

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
    let (input, (left, right)) = separated_pair(alpha1, tag(", "), alpha1)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (left, right)))
}

fn map_entry(input: &str) -> IResult<&str, MapEntry> {
    let (input, (id, (l, r))) = separated_pair(alpha1, tag(" = "), left_right)(input)?;

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
fn part1(input: &str) -> u32 {
    let (input, map) = parse_map(input).unwrap();
    assert_eq!(input, "");

    let mut count = 0;
    let mut it = map.steps.iter().cycle();
    let mut current = "AAA".to_string();
    loop {
        let step = it.next().unwrap();
        count += 1;
        let next = match step {
            'L' => map.left_right_map.get(&current).unwrap().left.clone(),
            'R' => map.left_right_map.get(&current).unwrap().right.clone(),
            _ => panic!("unknown step"),
        };
        if next == "ZZZ" {
            return count;
        } else {
            current = next;
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let test = include_str!("../../test1_1.txt");
        assert_eq!(part1(test), 2);

        let test = include_str!("../../test1_2.txt");
        assert_eq!(part1(test), 6);
    }
}
