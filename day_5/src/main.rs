use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use regex::Regex;

const INPUT_FILE_PATH: &str = "src/input.txt";

struct Ship {
    crate_stack: Vec<Vec<String>>,
}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn parse_line(line: String, re: &Regex) -> Result<Instruction, String> {
        let captures = re
            .captures(line.as_str())
            .expect("Could not gather capture groups");

        let quantity = captures
            .name("quantity")
            .expect("Capture group not found")
            .as_str()
            .parse::<usize>()
            .expect("Parsing error");
        let from = captures
            .name("from")
            .expect("Capture group not found")
            .as_str()
            .parse::<usize>()
            .expect("Parsing error")
            - 1;
        let to = captures
            .name("to")
            .expect("Capture group not found")
            .as_str()
            .parse::<usize>()
            .expect("Parsing error")
            - 1;

        Ok(Instruction { quantity, from, to })
    }
}

enum CrateMover {
    V9000,
    #[allow(dead_code)]
    V9001,
}

struct CargoCrane<'a> {
    ship: &'a mut Ship,
    version: CrateMover,
}

impl CargoCrane<'_> {
    pub fn lift_crates(&mut self, index: usize, quantity: usize) -> Vec<String> {
        let mut result = vec![];
        for _ in 0..quantity {
            result.push(
                self.ship.crate_stack[index]
                    .pop()
                    .expect("Nothing to pop here"),
            );
        }

        match self.version {
            CrateMover::V9000 => (),
            CrateMover::V9001 => result.reverse(),
        };
        result
    }

    pub fn unload_crates(&mut self, crates: &mut Vec<String>, index: usize) {
        self.ship.crate_stack[index].append(crates);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ship = Ship {
        crate_stack: vec![
            vec![
                String::from("V"),
                String::from("C"),
                String::from("W"),
                String::from("L"),
                String::from("R"),
                String::from("M"),
                String::from("F"),
                String::from("Q"),
            ],
            vec![String::from("L"), String::from("Q"), String::from("D")],
            vec![
                String::from("B"),
                String::from("N"),
                String::from("C"),
                String::from("W"),
                String::from("G"),
                String::from("R"),
                String::from("S"),
                String::from("P"),
            ],
            vec![
                String::from("G"),
                String::from("Q"),
                String::from("B"),
                String::from("H"),
                String::from("D"),
                String::from("C"),
                String::from("L"),
            ],
            vec![
                String::from("S"),
                String::from("Z"),
                String::from("F"),
                String::from("L"),
                String::from("G"),
                String::from("V"),
            ],
            vec![
                String::from("P"),
                String::from("N"),
                String::from("G"),
                String::from("D"),
            ],
            vec![
                String::from("W"),
                String::from("C"),
                String::from("F"),
                String::from("V"),
                String::from("P"),
                String::from("Z"),
                String::from("D"),
            ],
            vec![
                String::from("S"),
                String::from("M"),
                String::from("D"),
                String::from("P"),
                String::from("C"),
            ],
            vec![
                String::from("C"),
                String::from("P"),
                String::from("M"),
                String::from("V"),
                String::from("T"),
                String::from("W"),
                String::from("N"),
                String::from("Z"),
            ],
        ],
    };
    for crate_stack in &mut ship.crate_stack {
        crate_stack.reverse()
    }

    let mut crane = CargoCrane {
        ship: &mut ship,
        version: CrateMover::V9000,
    };

    let path = Path::new(INPUT_FILE_PATH);
    let file = File::open(path)?;

    let re: Regex = Regex::new(r"move (?P<quantity>\d+) from (?P<from>\d+) to (?P<to>\d+)")?;

    for line in BufReader::new(&file).lines() {
        let instruction = Instruction::parse_line(line?, &re)?;

        let mut holding_crates = crane.lift_crates(instruction.from, instruction.quantity);
        crane.unload_crates(&mut holding_crates, instruction.to)
    }

    print!("The crates on top are: ");
    for crate_stack in &ship.crate_stack {
        print!("{}", crate_stack[crate_stack.len() - 1])
    }

    Ok(())
}
