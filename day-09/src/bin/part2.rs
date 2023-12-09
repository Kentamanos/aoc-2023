use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1};

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

fn number_sequence(input: &str) -> nom::IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), nom::character::complete::i64)(input)
}

fn predict(sequence: Vec<i64>) -> i64 {
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

    // Add to end
    for i in 0..sequences.len() - 1 {
        let new_val = sequences[i].last().unwrap() + sequences[i + 1].last().unwrap();
        sequences[i + 1].push(new_val);
    }

    // Add to beginning
    for i in 0..sequences.len() - 1 {
        let new_val = sequences[i + 1].first().unwrap() - sequences[i].first().unwrap();

        let mut new_sequence = vec![new_val];
        new_sequence.append(sequences[i + 1].as_mut());

        sequences[i + 1] = new_sequence;
    }

    println!("{:?}", sequences);

    *sequences.last().unwrap().first().unwrap()
}

fn part2(input: &str) -> i64 {
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
    fn test_part2() {
        let test = include_str!("../../test.txt");
        assert_eq!(part2(test), 2);
    }
}
