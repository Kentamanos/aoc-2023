use nom::{
    bytes::complete::tag,
    character::complete::none_of,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

fn word(input: &str) -> IResult<&str, Vec<char>> {
    let (input, word) = many1(none_of(",\n"))(input)?;

    Ok((input, word))
}

fn calculate_hash(word: &Vec<char>) -> u32 {
    let mut hash = 0;
    for c in word {
        hash += *c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part1(input: &str) -> u32 {
    let (input, words) = separated_list1(tag(","), word)(input).unwrap();
    assert_eq!(input, "");

    let hash_total = words.iter().map(calculate_hash).sum::<u32>();

    hash_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../test.txt");
        assert_eq!(part1(input), 1320);
    }
}
