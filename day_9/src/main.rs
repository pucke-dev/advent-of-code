const INPUT_FILE_PATH: &str = "day_9/src/input.txt";

use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone)]
struct RopeElement {
    row: isize,
    col: isize,
}
struct Rope {
    elements: Vec<RopeElement>,
}

struct Simulation {
    grid: Vec<Vec<String>>,
    visited_fields: HashSet<(isize, isize)>,
}

impl Simulation {
    pub fn init(dimensions: isize) -> Simulation {
        Simulation {
            grid: vec![vec![" . ".to_string(); dimensions as usize]; dimensions as usize],
            visited_fields: HashSet::new(),
        }
    }

    fn move_head_right(&mut self, head: &mut RopeElement) {
        head.col += 1;
    }

    fn move_head_left(&mut self, head: &mut RopeElement) {
        head.col -= 1;
    }

    fn move_head_up(&mut self, head: &mut RopeElement) {
        head.row -= 1;
    }

    fn move_head_down(&mut self, head: &mut RopeElement) {
        head.row += 1;
    }

    fn invariant_from_head_to_tail(&mut self, rope: &mut Rope) {
        for i in 0..rope.elements.len() - 1 {
            self.invariant(&rope.elements[i].clone(), &mut rope.elements[i + 1]);
        }
    }

    fn invariant(&mut self, head: &RopeElement, tail: &mut RopeElement) {
        // x x x
        // x HT x
        // x x x
        if head.row == tail.row && head.col == tail.col {
            return;
        }
        // x x x
        // x T H
        // x x x
        if head.row == tail.row && head.col - tail.col == 1 {
            return;
        }
        // x x x
        // H T x
        // x x x
        if head.row == tail.row && tail.col - head.col == 1 {
            return;
        }
        // x x x
        // x T x
        // x H x
        if head.col == tail.col && head.row - tail.row == 1 {
            return;
        }
        // x H x
        // x T x
        // x x x
        if head.col == tail.col && tail.row - head.row == 1 {
            return;
        }
        // x x H
        // x T x
        // x x x
        if head.col - tail.col == 1 && tail.row - head.row == 1 {
            return;
        }
        // H x x
        // x T x
        // x x x
        if tail.col - head.col == 1 && tail.row - head.row == 1 {
            return;
        }
        // x x x
        // x T x
        // H x x
        if tail.col - head.col == 1 && head.row - tail.row == 1 {
            return;
        }
        // x x x
        // x T x
        // x x H
        if head.col - tail.col == 1 && head.row - tail.row == 1 {
            return;
        }

        // ====

        // x x x x
        // x T x H
        // x x x x
        if head.row == tail.row && head.col - tail.col == 2 {
            tail.col += 1;
            return;
        }

        // x x x x
        // H x T x
        // x x x x
        if head.row == tail.row && tail.col - head.col == 2 {
            tail.col -= 1;
            return;
        }

        // x x x
        // x T x
        // x x x
        // x H x
        if head.col == tail.col && head.row - tail.row == 2 {
            tail.row += 1;
            return;
        }

        // x H x
        // x x x
        // x T x
        // x x x
        if head.col == tail.col && tail.row - head.row == 2 {
            tail.row -= 1;
            return;
        }

        // x x x x
        // x x x H
        // x T x x
        // x x x x
        if head.col - tail.col == 2 && tail.row - head.row == 1 {
            tail.row -= 1;
            tail.col += 1;
            return;
        }

        // x x H x
        // x x x x
        // x T x x
        // x x x x
        if head.col - tail.col == 1 && tail.row - head.row == 2 {
            tail.row -= 1;
            tail.col += 1;
            return;
        }

        // x H x x
        // x x x x
        // x x T x
        // x x x x
        if tail.col - head.col == 1 && tail.row - head.row == 2 {
            tail.row -= 1;
            tail.col -= 1;
            return;
        }

        // x x x x
        // H x x x
        // x x T x
        // x x x x
        if tail.col - head.col == 2 && tail.row - head.row == 1 {
            tail.row -= 1;
            tail.col -= 1;
            return;
        }

        // x x x x
        // x x T x
        // x x x x
        // x H x x
        if tail.col - head.col == 1 && head.row - tail.row == 2 {
            tail.col -= 1;
            tail.row += 1;
            return;
        }

        // x x x x
        // x x T x
        // H x x x
        // x x x x
        if tail.col - head.col == 2 && head.row - tail.row == 1 {
            tail.col -= 1;
            tail.row += 1;
            return;
        }

        // x x x x
        // x T x x
        // x x x x
        // x x H x
        if head.col - tail.col == 1 && head.row - tail.row == 2 {
            tail.col += 1;
            tail.row += 1;
            return;
        }

        // x x x x
        // x T x x
        // x x x H
        // x x x x
        if head.col - tail.col == 2 && head.row - tail.row == 1 {
            tail.col += 1;
            tail.row += 1;
        }

        // x x x x
        // x x H x
        // x x x x
        // T x x x
        if head.col - tail.col == 2 && tail.row - head.row == 2 {
            tail.col += 1;
            tail.row -= 1;
        }

        // x x x x
        // x H x x
        // x x x x
        // x x x T
        if tail.col - head.col == 2 && tail.row - head.row == 2 {
            tail.col -= 1;
            tail.row -= 1;
        }

        // T x x x
        // x x x x
        // x x H x
        // x x x x
        if head.col - tail.col == 2 && head.row - tail.row == 2 {
            tail.col += 1;
            tail.row += 1;
        }

        // x x x T
        // x x x x
        // x H x x
        // x x x x
        if tail.col - head.col == 2 && head.row - tail.row == 2 {
            tail.col -= 1;
            tail.row += 1;
        }
    }

    fn add_visited_field(&mut self, row: isize, col: isize) {
        self.visited_fields.insert((row, col));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut simulation = Simulation::init(300);

    let mut rope = Rope {
        elements: vec![
            RopeElement {
                row: (simulation.grid.len() / 2) as isize,
                col: (simulation.grid.len() / 2) as isize,
            };
            10
        ],
    };

    // let mut rope = Rope {
    //     elements: vec![
    //         RopeElement {
    //             row: (simulation.grid.len() / 2) as isize,
    //             col: (simulation.grid.len() / 2) as isize
    //         };
    //         2
    //     ],
    // };

    let input = read_to_string(INPUT_FILE_PATH)?;

    for (_row, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[..] {
            ["U", distance] => {
                for _ in 0..distance.parse::<i32>()? {
                    simulation.move_head_up(&mut rope.elements[0]);

                    simulation.invariant_from_head_to_tail(&mut rope);

                    simulation.add_visited_field(
                        rope.elements[rope.elements.len() - 1].row,
                        rope.elements[rope.elements.len() - 1].col,
                    );
                }
            }
            ["D", distance] => {
                for _ in 0..distance.parse::<i32>()? {
                    simulation.move_head_down(&mut rope.elements[0]);

                    simulation.invariant_from_head_to_tail(&mut rope);

                    simulation.add_visited_field(
                        rope.elements[rope.elements.len() - 1].row,
                        rope.elements[rope.elements.len() - 1].col,
                    );
                }
            }
            ["L", distance] => {
                for _ in 0..distance.parse::<i32>()? {
                    simulation.move_head_left(&mut rope.elements[0]);

                    simulation.invariant_from_head_to_tail(&mut rope);

                    simulation.add_visited_field(
                        rope.elements[rope.elements.len() - 1].row,
                        rope.elements[rope.elements.len() - 1].col,
                    );
                }
            }
            ["R", distance] => {
                for _ in 0..distance.parse::<i32>()? {
                    simulation.move_head_right(&mut rope.elements[0]);

                    simulation.invariant_from_head_to_tail(&mut rope);

                    simulation.add_visited_field(
                        rope.elements[rope.elements.len() - 1].row,
                        rope.elements[rope.elements.len() - 1].col,
                    );
                }
            }
            _ => panic!("Invalid direction"),
        }
    }
    println!("Visited Fields: {}", simulation.visited_fields.len());
    Ok(())
}
