use core::panic;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::anychar, multi::count, sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
enum Card {
    Joker,
    Number(u32),
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'J' => Card::Joker,
            '2'..='9' => Card::Number(c.to_digit(10).unwrap()),
            'T' => Card::Number(10),
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPairs(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Hand {
        Hand {
            hand_type: Hand::determine_hand_type(cards),
            bid,
        }
    }

    fn determine_hand_type(cards: Vec<Card>) -> HandType {
        let joker_count = cards.iter().filter(|card| **card == Card::Joker).count();
        if joker_count == 5 {
            return HandType::FiveOfAKind(cards);
        }

        let mut card_counts: HashMap<&Card, usize> = HashMap::new();
        for card in cards.iter() {
            if card == &Card::Joker {
                continue;
            }
            *card_counts.entry(card).or_default() += 1;
        }

        let mut card_counts: Vec<_> = card_counts.values().collect::<Vec<_>>();
        card_counts.sort();

        // Increment the last item to account for jokers
        let last_count = card_counts.last_mut().unwrap();
        let new_count = *last_count + joker_count;
        *last_count = &(new_count);

        match card_counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard(cards),
            [1, 1, 1, 2] => HandType::OnePair(cards),
            [1, 2, 2] => HandType::TwoPairs(cards),
            [1, 1, 3] => HandType::ThreeOfAKind(cards),
            [2, 3] => HandType::FullHouse(cards),
            [1, 4] => HandType::FourOfAKind(cards),
            [5] => HandType::FiveOfAKind(cards),
            _ => panic!("Invalid hand: {:?}", cards),
        }
    }
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, c) = anychar(input)?;
    Ok((input, Card::from(c)))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) =
        separated_pair(count(card, 5), tag(" "), nom::character::complete::u32)(input)?;
    Ok((input, Hand::new(cards, bid)))
}

fn part2(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| parse_hand(line).unwrap().1)
        .collect::<Vec<_>>();
    hands.sort();

    let mut score = 0;
    for (i, hand) in hands.iter().enumerate() {
        score += hand.bid * (i as u32 + 1);
    }

    score
}

fn main() {
    let input = include_str!("../../input.txt");

    println!("part1: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part2(test), 5905);
    }
}
