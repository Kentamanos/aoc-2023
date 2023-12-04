use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

struct Card {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    numbers_present: HashSet<u32>,
}

fn numbers_separated_by_space(input: &str) -> IResult<&str, Vec<u32>> {
    // Trim off possible leading space
    let (input, _) = space0(input)?;
    separated_list1(space1, complete::u32)(input)
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, card_number) = complete::u32(input)?;

    let (input, _) = tag(": ")(input)?;

    let (input, (winning_numbers, numbers_present)) = separated_pair(
        numbers_separated_by_space,
        tag(" | "),
        numbers_separated_by_space,
    )(input)?;

    Ok((
        input,
        Card {
            card_number,
            winning_numbers: winning_numbers.into_iter().collect(),
            numbers_present: numbers_present.into_iter().collect(),
        },
    ))
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}

fn part1(input: &str) -> u32 {
    let mut score = 0;
    let cards = cards(input).unwrap().1;
    for card in cards {
        let intersection = card.numbers_present.intersection(&card.winning_numbers);
        let count = intersection.count() as u32;
        if count > 0 {
            score += 2u32.pow(count - 1)
        }
    }
    score
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (_, card) = card(input).unwrap();
        assert_eq!(card.card_number, 1);
        assert_eq!(card.winning_numbers.len(), 5);
        assert_eq!(card.numbers_present.len(), 8);
    }

    #[test]
    fn parse_cards() {
        let input = include_str!("../../test.txt");
        let (remaining, cards) = cards(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cards.len(), 6);
    }

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part1(test), 13);
    }
}
