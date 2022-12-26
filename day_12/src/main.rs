use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_PATH: &str = "day_12/src/input.txt";

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Cell {
    row: usize,
    col: usize,
    distance: usize,
    value: char,
}

fn parse_input(path: &str) -> Option<Vec<Vec<Cell>>> {
    let path = Path::new(path);
    let file = File::open(path).unwrap();

    let mut grid = vec![];

    for (row, line) in BufReader::new(&file).lines().enumerate() {
        grid.push(
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, char)| Cell {
                    row,
                    col,
                    distance: 0,
                    value: char,
                })
                .collect(),
        )
    }

    Some(grid)
}

fn get_adjacents<V>(grid: &Vec<Vec<Cell>>, row: usize, col: usize, is_valid_move: V) -> Vec<&Cell>
where
    V: Fn(char, char) -> bool,
{
    let mut adjacent = vec![];

    // Down
    if row > 0 && is_valid_move(grid[row][col].value, grid[row - 1][col].value) {
        adjacent.push(&grid[row - 1][col]);
    }

    // Up
    if row < grid.len() - 1 && is_valid_move(grid[row][col].value, grid[row + 1][col].value) {
        adjacent.push(&grid[row + 1][col]);
    }

    // Left
    if col > 0 && is_valid_move(grid[row][col].value, grid[row][col - 1].value) {
        adjacent.push(&grid[row][col - 1]);
    }

    // Right
    if col < grid[0].len() - 1 && is_valid_move(grid[row][col].value, grid[row][col + 1].value) {
        adjacent.push(&grid[row][col + 1]);
    }

    adjacent
}

fn find_cell(grid: &Vec<Vec<Cell>>, char: char) -> Option<Cell> {
    for row in grid {
        for col in row {
            if col.value == char {
                return Some(*col);
            }
        }
    }
    None
}

fn breadth_first_search<F, V>(
    grid: Vec<Vec<Cell>>,
    start: Cell,
    destination: F,
    is_valid_move: V,
) -> Option<usize>
where
    F: Fn(&Cell) -> bool,
    V: Fn(char, char) -> bool,
{
    let mut queue: VecDeque<Cell> = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(cell) = queue.pop_front() {
        if destination(&cell) {
            return Some(cell.distance);
        }

        for next in get_adjacents(&grid, cell.row, cell.col, &is_valid_move) {
            if visited.contains(next) {
                continue;
            }

            visited.insert(*next);
            queue.push_back(Cell {
                row: next.row,
                col: next.col,
                distance: cell.distance + 1,
                value: next.value,
            });
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = parse_input(INPUT_FILE_PATH).unwrap();

    let start = find_cell(&grid, 'S').unwrap();
    let destination = find_cell(&grid, 'E').unwrap();

    let solution = breadth_first_search(
        grid.clone(),
        start,
        |node| node.value == destination.value,
        |mut from, mut to| {
            if from == 'S' {
                from = 'a';
            }
            if to == 'E' {
                to = 'z';
            }

            if (to as u32) < (from as u32) {
                return true;
            }

            if (to as u32) - (from as u32) <= 1 {
                return true;
            }

            false
        },
    )
    .expect("No way to the destination found");
    println!("Part 1: {solution}");

    let solution = breadth_first_search(
        grid,
        destination,
        |node| node.value == 'a',
        |mut from, mut to| {
            if from == 'S' {
                from = 'a';
            }
            if to == 'E' {
                to = 'z';
            }

            if (to as u32) > (from as u32) {
                return true;
            }

            if (from as u32) - (to as u32) <= 1 {
                return true;
            }

            false
        },
    )
    .unwrap();
    println!("Part 2: {solution}");

    Ok(())
}
