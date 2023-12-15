use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Operation {
    Insert(u8),
    Remove,
}

#[derive(Debug)]
struct Instruction<'a> {
    label: &'a str,
    box_number: u8,
    operation: Operation,
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

fn calculate_hash(word: &Vec<char>) -> u8 {
    let mut hash = 0;
    for c in word {
        hash += *c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}

fn remove(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("-")(input)?;

    Ok((input, Operation::Remove))
}

fn insert(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("=")(input)?;
    let (input, focal_length) = complete::u8(input)?;

    Ok((input, Operation::Insert(focal_length)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, label) = alpha1(input)?;
    let (input, operation) = alt((remove, insert))(input)?;

    // let ret_val = word.iter().collect::<String>();

    let hash = calculate_hash(&label.chars().collect::<Vec<char>>());

    let instruction = Instruction {
        label,
        box_number: hash,
        operation,
    };

    Ok((input, instruction))
}

fn part2(input: &str) -> u64 {
    let (input, instructions) = separated_list1(tag(","), instruction)(input).unwrap();
    assert_eq!(input, "");

    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    for instruction in instructions {
        match instruction.operation {
            Operation::Insert(focal_length) => {
                if let Some(lenses_in_box) = boxes.get_mut(&instruction.box_number) {
                    if let Some(lense_with_label) = lenses_in_box
                        .iter_mut()
                        .find(|lens| lens.label == instruction.label)
                    {
                        let new_lens = Lens {
                            label: instruction.label,
                            focal_length,
                        };
                        *lense_with_label = new_lens;
                    } else {
                        lenses_in_box.push(Lens {
                            label: instruction.label,
                            focal_length,
                        });
                    }
                } else {
                    let new_lenses_vec = vec![Lens {
                        label: instruction.label,
                        focal_length,
                    }];
                    boxes.insert(instruction.box_number, new_lenses_vec);
                }
            }
            Operation::Remove => {
                if let Some(lenses_in_box) = boxes.get_mut(&instruction.box_number) {
                    lenses_in_box.retain(|lens| lens.label != instruction.label);
                }
            }
        }
    }

    let mut total = 0u64;
    for box_number in 0..=255 {
        if let Some(lenses_in_box) = boxes.get(&box_number) {
            println!("box_number: {} - {:?}", box_number, lenses_in_box);
            lenses_in_box.iter().enumerate().for_each(|(slot, lens)| {
                total += (box_number as u64 + 1) * lens.focal_length as u64 * (slot as u64 + 1);
            });
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../test.txt");
        assert_eq!(part2(input), 145);
    }

    #[test]
    fn check_labels() {
        let foo = "rn";
        let value = part2(foo);
        assert_eq!(value, 0);

        let foo = "qp";
        let value = part2(foo);
        assert_eq!(value, 1);

        let foo = "cm";
        let value = part2(foo);
        assert_eq!(value, 0);

        let foo = "pc";
        let value = part2(foo);
        assert_eq!(value, 3);

        let foo = "ot";
        let value = part2(foo);
        assert_eq!(value, 3);

        let foo = "ab";
        let value = part2(foo);
        assert_eq!(value, 3);
    }
}
