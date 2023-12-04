use std::{cell::RefCell, collections::HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

struct Card {
    copies: u32,
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
fn card(input: &str) -> IResult<&str, RefCell<Card>> {
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
        RefCell::new(Card {
            copies: 1,
            card_number,
            winning_numbers: winning_numbers.into_iter().collect(),
            numbers_present: numbers_present.into_iter().collect(),
        }),
    ))
}

fn cards(input: &str) -> IResult<&str, Vec<RefCell<Card>>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}

fn part2(input: &str) -> u32 {
    let mut cards = cards(input).unwrap().1;
    for (i, card) in cards.iter().enumerate() {
        let count = card
            .borrow()
            .numbers_present
            .intersection(&card.borrow().winning_numbers)
            .count();
        for j in i + 1..i + 1 + count {
            cards[j].borrow_mut().copies += card.borrow().copies;
        }
    }

    cards.iter().map(|card| card.borrow().copies).sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (_, card) = card(input).unwrap();
        assert_eq!(card.borrow().card_number, 1);
        assert_eq!(card.borrow().winning_numbers.len(), 5);
        assert_eq!(card.borrow().numbers_present.len(), 8);
    }

    #[test]
    fn parse_cards() {
        let input = include_str!("../../test.txt");
        let (remaining, cards) = cards(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cards.len(), 6);
    }

    #[test]
    fn test_part2() {
        let test = include_str!("../../test.txt");
        let count = part2(test);
        assert_eq!(count, 30);
    }
}
