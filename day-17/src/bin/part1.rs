use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, VecDeque};

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

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
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

fn part1(input: &str) -> u32 {
    let (input, (row_count, col_count, grid)) = parse_grid(input).unwrap();
    assert_eq!(input, "");

    let start = RowColumn { row: 0, column: 0 };
    let goal = RowColumn {
        row: row_count as isize - 1,
        column: col_count as isize - 1,
    };
    let result: (Vec<(RowColumn, VecDeque<RowColumn>)>, u32) = dijkstra(
        &(start, VecDeque::from([start])),
        |(pos, deque)| {
            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                // Remove stuff not in the grid
                .filter(|(row_offset, col_offset)| {
                    grid.contains_key(&RowColumn {
                        row: pos.row + row_offset,
                        column: pos.column + col_offset,
                    })
                })
                .filter_map(|(row_offset, col_offset)| {
                    let next_pos = RowColumn {
                        row: pos.row + row_offset,
                        column: pos.column + col_offset,
                    };

                    // Don't backtrack to avoid going straight 3 times...
                    if deque.len() > 2 && deque[deque.len() - 2] == next_pos {
                        return None;
                    }

                    let mut new_deque = deque.clone();
                    new_deque.push_back(next_pos);
                    if new_deque.len() == 5 {
                        // Four moves, let's make sure this one isn't the same direction as the last 3 moves (last 4 directions)
                        if new_deque
                            .iter()
                            .tuple_windows()
                            .map(|(a, b)| {
                                let row_diff = a.row - b.row;
                                let col_diff = a.column - b.column;
                                (row_diff, col_diff)
                            })
                            .all_equal()
                        {
                            None
                        } else {
                            // Take off the first one to save space
                            new_deque.pop_front();
                            Some((next_pos, new_deque))
                        }
                    } else {
                        Some((next_pos, new_deque))
                    }
                })
                .map(|(pos, dequeue)| {
                    let cost = grid.get(&pos).unwrap();
                    ((pos, dequeue), *cost as u32)
                })
                .collect::<Vec<((RowColumn, VecDeque<RowColumn>), u32)>>()
        },
        |(pos, _deque)| *pos == goal,
    )
    .unwrap();

    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../test.txt");
        assert_eq!(part1(input), 102);
    }
}
