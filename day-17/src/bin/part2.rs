use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct RowColumn {
    row: isize,
    column: isize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct DirectionOffset {
    row_offset: isize,
    column_offset: isize,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

fn parse_row(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, row_tiles) = many1(one_of("0123456789"))(input)?;
    let row = row_tiles
        .iter()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    Ok((input, row))
}

fn parse_grid(input: &str) -> IResult<&str, (usize, usize, HashMap<RowColumn, u8>)> {
    let (input, rows) = separated_list1(newline, parse_row)(input).unwrap();
    let row_count = rows.len();
    let col_count = rows[0].len();
    assert!(rows.iter().all(|row| row.len() == col_count));

    let mut grid: HashMap<_, _> = HashMap::new();

    for row in 0..rows.len() as isize {
        for column in 0..rows[row as usize].len() as isize {
            grid.insert(
                RowColumn { row, column },
                rows[row as usize][column as usize],
            );
        }
    }

    Ok((input, (row_count, col_count, grid)))
}

fn part2(input: &str) -> u32 {
    let (input, (row_count, col_count, grid)) = parse_grid(input).unwrap();
    assert_eq!(input, "");

    let start = RowColumn { row: 0, column: 0 };
    let goal = RowColumn {
        row: row_count as isize - 1,
        column: col_count as isize - 1,
    };
    let result: (Vec<(RowColumn, DirectionOffset)>, u32) = dijkstra(
        &(
            start,
            DirectionOffset {
                row_offset: 0,
                column_offset: 0,
            },
        ),
        |(pos, direction_offset)| {
            [
                // NSEW
                DirectionOffset {
                    row_offset: -1,
                    column_offset: 0,
                },
                DirectionOffset {
                    row_offset: 1,
                    column_offset: 0,
                },
                DirectionOffset {
                    row_offset: 0,
                    column_offset: 1,
                },
                DirectionOffset {
                    row_offset: 0,
                    column_offset: -1,
                },
            ]
            .into_iter()
            // Remove positions not in grid
            .filter(|do_candidate| {
                grid.contains_key(&RowColumn {
                    row: pos.row + do_candidate.row_offset,
                    column: pos.column + do_candidate.column_offset,
                })
            })
            .filter_map(|do_candidate| {
                let next_pos = RowColumn {
                    row: pos.row + do_candidate.row_offset,
                    column: pos.column + do_candidate.column_offset,
                };

                let col_candidate_polarity = do_candidate.column_offset.signum();
                let row_candidate_polarity = do_candidate.row_offset.signum();
                let col_polarity = direction_offset.column_offset.signum();
                let row_polarity = direction_offset.row_offset.signum();

                let col_polarity_changed = col_candidate_polarity != col_polarity;
                let row_polarity_changed = row_candidate_polarity != row_polarity;

                let steps_in_same_direction = direction_offset
                    .row_offset
                    .abs()
                    .max(direction_offset.column_offset.abs());

                match (row_polarity_changed, col_polarity_changed) {
                    (true, false) => {
                        // Make sure we're not back tracking... (only legal case should be just starting)
                        if row_polarity == -row_candidate_polarity {
                            None
                        } else {
                            Some((next_pos, do_candidate))
                        }
                    }
                    (false, true) => {
                        // Make sure we're not back tracking... (only legal case should be just starting)
                        if col_polarity == -col_candidate_polarity {
                            None
                        } else {
                            Some((next_pos, do_candidate))
                        }
                    }
                    (false, false) => {
                        // Heading in same direction, make sure we're not going over 10 moves
                        let new_do = DirectionOffset {
                            row_offset: direction_offset.row_offset + do_candidate.row_offset,
                            column_offset: direction_offset.column_offset
                                + do_candidate.column_offset,
                        };
                        if new_do.row_offset.abs() > 10 || new_do.column_offset.abs() > 10 {
                            None
                        } else {
                            Some((next_pos, new_do))
                        }
                    }
                    (true, true) => {
                        // Went from moving North/South to East/West or vice versa, just put in new direction, but first...
                        // Make sure we've moved at least 4 steps in same direction
                        if steps_in_same_direction < 4 {
                            None
                        } else {
                            Some((next_pos, do_candidate))
                        }
                    }
                }
            })
            .map(|(pos, direction_offset)| {
                let cost = grid.get(&pos).unwrap();
                ((pos, direction_offset), *cost as u32)
            })
            .collect::<Vec<((RowColumn, DirectionOffset), u32)>>()
        },
        |(pos, direction_offset)| {
            *pos == goal &&
                // Make sure at the end we went at least four steps in one direction
                direction_offset
                    .row_offset
                    .abs()
                    .max(direction_offset.column_offset.abs())
                    >= 4
        },
    )
    .unwrap();

    for (pos, direction_offset) in result.0 {
        println!("{:?} {:?}", pos, direction_offset);
    }

    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        // let input = include_str!("../../test.txt");
        // assert_eq!(part2(input), 94);

        let input = include_str!("../../test2.txt");
        assert_eq!(part2(input), 71);
    }
}
