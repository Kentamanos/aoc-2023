use std::{cell::RefCell, collections::HashSet};

use grid::*;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    MirrorNorthEast,
    MirrorNorthWest,
    VerticalSplit,
    HorizontalSplit,
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    energized: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

fn tile(input: &str) -> IResult<&str, RefCell<Tile>> {
    let (input, tile_type) = one_of(r".\/|-")(input)?;
    let tile_type = match tile_type {
        '.' => TileType::Empty,
        '/' => TileType::MirrorNorthEast,
        '\\' => TileType::MirrorNorthWest,
        '|' => TileType::VerticalSplit,
        '-' => TileType::HorizontalSplit,
        _ => panic!("unexpected tile type"),
    };

    Ok((
        input,
        RefCell::new(Tile {
            tile_type,
            energized: false,
        }),
    ))
}

fn tile_row(input: &str) -> IResult<&str, Vec<RefCell<Tile>>> {
    let (input, row_tiles) = many1(tile)(input)?;
    Ok((input, row_tiles))
}

fn grid(input: &str) -> IResult<&str, Grid<RefCell<Tile>>> {
    let (input, tiles) = separated_list1(newline, tile_row)(input)?;

    let mut grid: Grid<RefCell<Tile>> = grid![];
    for row in tiles {
        grid.push_row(row);
    }

    Ok((input, grid))
}

fn move_into(
    hs: &mut HashSet<(isize, isize, Direction)>,
    grid: &Grid<RefCell<Tile>>,
    row: isize,
    col: isize,
    heading: Direction,
) {
    // TODO: if we'd be out of bounds, end the beam
    if row < 0 || row >= grid.rows() as isize || col < 0 || col >= grid.cols() as isize {
        return;
    }

    if hs.contains(&(row, col, heading.clone())) {
        return;
    }

    hs.insert((row, col, heading.clone()));

    // Energize our cell
    grid.get(row as usize, col as usize)
        .unwrap()
        .borrow_mut()
        .energized = true;

    // Figure out where to head next
    let tile_type = grid
        .get(row as usize, col as usize)
        .unwrap()
        .borrow()
        .tile_type
        .clone();

    match tile_type {
        TileType::Empty => match heading {
            Direction::North => move_into(hs, grid, row - 1, col, heading),
            Direction::South => move_into(hs, grid, row + 1, col, heading),
            Direction::East => move_into(hs, grid, row, col + 1, heading),
            Direction::West => move_into(hs, grid, row, col - 1, heading),
        },
        TileType::MirrorNorthEast => match heading {
            Direction::North => move_into(hs, grid, row, col + 1, Direction::East),
            Direction::South => move_into(hs, grid, row, col - 1, Direction::West),
            Direction::East => move_into(hs, grid, row - 1, col, Direction::North),
            Direction::West => move_into(hs, grid, row + 1, col, Direction::South),
        },
        TileType::MirrorNorthWest => match heading {
            Direction::North => move_into(hs, grid, row, col - 1, Direction::West),
            Direction::South => move_into(hs, grid, row, col + 1, Direction::East),
            Direction::East => move_into(hs, grid, row + 1, col, Direction::South),
            Direction::West => move_into(hs, grid, row - 1, col, Direction::North),
        },
        TileType::VerticalSplit => match heading {
            Direction::North => move_into(hs, grid, row - 1, col, heading),
            Direction::South => move_into(hs, grid, row + 1, col, heading),
            Direction::East | Direction::West => {
                move_into(hs, grid, row - 1, col, Direction::North);
                move_into(hs, grid, row + 1, col, Direction::South);
            }
        },
        TileType::HorizontalSplit => match heading {
            Direction::East => move_into(hs, grid, row, col + 1, heading),
            Direction::West => move_into(hs, grid, row, col - 1, heading),
            Direction::North | Direction::South => {
                move_into(hs, grid, row, col - 1, Direction::West);
                move_into(hs, grid, row, col + 1, Direction::East);
            }
        },
    }
}

fn count_energized(grid: &Grid<RefCell<Tile>>) -> u64 {
    grid.iter().filter(|tile| tile.borrow().energized).count() as u64
}

fn part2(input: &str) -> u64 {
    let (input, grid) = grid(input).unwrap();
    assert_eq!(input, "");

    let passes = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut max_energized = 0;
    for pass in passes {
        match pass {
            Direction::East | Direction::West => {
                let start_col = match pass {
                    Direction::East => 0isize,
                    Direction::West => grid.cols() as isize - 1,
                    _ => panic!("unexpected pass"),
                };
                for row in 0..grid.rows() as isize {
                    let grid_clone = grid.clone();
                    let mut hs: HashSet<(isize, isize, Direction)> = HashSet::new();

                    move_into(&mut hs, &grid_clone, row, start_col, pass.clone());
                    let energized = count_energized(&grid_clone);
                    if energized > max_energized {
                        max_energized = energized;
                    }
                }
            }
            Direction::North | Direction::South => {
                let start_row = match pass {
                    Direction::South => 0isize,
                    Direction::North => grid.rows() as isize - 1,
                    _ => panic!("unexpected pass"),
                };
                for col in 0..grid.cols() as isize {
                    let grid_clone = grid.clone();
                    let mut hs: HashSet<(isize, isize, Direction)> = HashSet::new();

                    move_into(&mut hs, &grid_clone, start_row, col, pass.clone());
                    let energized = count_energized(&grid_clone);
                    if energized > max_energized {
                        max_energized = energized;
                    }
                }
            }
        }
    }

    max_energized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../test.txt");
        assert_eq!(part2(input), 51);
    }
}
