use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1};

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

fn number_sequence(input: &str) -> nom::IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), nom::character::complete::i32)(input)
}

fn predict(sequence: Vec<i32>) -> i32 {
    let mut sequences = vec![];
    sequences.push(sequence);

    loop {
        let last_sequence = sequences.last().unwrap();
        let all_zeroes = last_sequence.iter().all(|&x| x == 0);

        if all_zeroes {
            break;
        } else {
            let mut new_sequence = vec![];
            for i in 0..last_sequence.len() - 1 {
                new_sequence.push(last_sequence[i + 1] - last_sequence[i]);
            }

            sequences.push(new_sequence);
        }
    }

    //let mut sequences = sequences[0..sequences.len() - 1].to_vec();
    sequences.reverse();

    for i in 0..sequences.len() - 1 {
        let new_val = sequences[i].last().unwrap() + sequences[i + 1].last().unwrap();
        sequences[i + 1].push(new_val);
    }

    println!("{:?}", sequences);

    *sequences.last().unwrap().last().unwrap()
}

fn part1(input: &str) -> i32 {
    let (input, sequences) = separated_list1(newline, number_sequence)(input).unwrap();
    assert_eq!(input, "");
    println!("{:?}", sequences);

    let mut total = 0;
    for sequence in sequences {
        total += predict(sequence);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part1(test), 114);
    }
}
