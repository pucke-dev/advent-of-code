use std::{cmp::Ordering, error::Error, fs::File, io::BufReader, io::prelude::*, path::Path};

const INPUT_FILE_PATH: &str = "src/input.txt";

struct Elve {
    items: Vec<i32>,
}

impl Elve {
    fn add_item(&mut self, item: i32) -> () {
        self.items.push(item)
    }
    fn get_total_calories(&self) -> i32 {
        self.items.iter().sum()
    }
}

impl Eq for Elve {}

impl PartialEq<Self> for Elve {
    fn eq(&self, other: &Self) -> bool {
        self.get_total_calories().eq(&other.get_total_calories())
    }
}

impl PartialOrd<Self> for Elve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_total_calories().partial_cmp(&other.get_total_calories())
    }
}

impl Ord for Elve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_total_calories().cmp(&other.get_total_calories())
    }
}

fn print_elves_leaderboard(elves: &mut Vec<Elve>, mut top_n: usize) -> () {
    if top_n > elves.len() {
        top_n = elves.len();
    }

    for (index, elve) in elves[elves.len() - top_n..].iter().rev().enumerate() {
        println!("{}. Elve carries {} calories", index + 1, elve.get_total_calories());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(INPUT_FILE_PATH);
    let file = File::open(&path)?;

    let mut elves: Vec<Elve> = vec![Elve { items: vec![] }];

    for line in BufReader::new(&file).lines() {
        let line_content = line?;

        if line_content.eq("") {
            elves.push(Elve { items: vec![] });
            continue;
        }

        let calories = line_content.parse::<i32>()?;

        let last_elve_index = elves.len() - 1;
        elves[last_elve_index].add_item(calories);
    }

    elves.sort();
    print_elves_leaderboard(&mut elves, 10);

    println!("Top 3 Elve's have {} calories in total", elves[elves.len() - 3..].iter().fold(0, |a, b| { a + b.get_total_calories() }));

    Ok(())
}
