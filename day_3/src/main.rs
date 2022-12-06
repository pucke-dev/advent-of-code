extern crate core;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Seek, SeekFrom};
use std::path::Path;

use itertools::{Chunk, Itertools};

const INPUT_FILE_PATH: &str = "src/input.txt";

fn get_char_value(char: &char) -> i32 {
    match char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid char"),
    }
}

fn split_in_half(string: &String) -> (&str, &str) {
    string.split_at(string.len() / 2)
}

fn get_backpacks_from<'a>(
    chunk: Chunk<'a, Lines<BufReader<&'a File>>>,
) -> Result<(HashSet<char>, HashSet<char>, HashSet<char>), String> {
    let mut backpacks: (HashSet<char>, HashSet<char>, HashSet<char>) =
        (HashSet::new(), HashSet::new(), HashSet::new());

    for (index, content) in chunk.enumerate() {
        match index {
            0 => backpacks.0 = HashSet::from_iter(content.expect("Could not parse").chars()),
            1 => backpacks.1 = HashSet::from_iter(content.expect("Could not parse").chars()),
            2 => backpacks.2 = HashSet::from_iter(content.expect("Could not parse").chars()),
            _ => return Err("I did not expect that many elements".to_string()),
        };
    }

    Ok(backpacks)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(INPUT_FILE_PATH);
    let mut file = File::open(&path)?;

    let mut duplicate_item_in_compartments_sum = 0;
    for line in BufReader::new(&file).lines() {
        let line_content = line.expect("Could not parse line");

        let (first_half, second_half) = split_in_half(&line_content);

        let compartment_1: HashSet<char> = HashSet::from_iter(first_half.chars());
        let compartment_2: HashSet<char> = HashSet::from_iter(second_half.chars());

        // Due to the assignment, we can assume that there is only ONE item type in both compartments.
        for character in compartment_1.intersection(&compartment_2) {
            duplicate_item_in_compartments_sum += get_char_value(character)
        }
    }

    println!(
        "Duplicate item in compartment sum: {}",
        duplicate_item_in_compartments_sum
    );

    // Reset File to read from start
    file.seek(SeekFrom::Start(0))
        .expect("Could not reset file buffer");

    let mut all_badges_sum = 0;
    for chunk in &BufReader::new(&file).lines().chunks(3) {
        let backpacks = get_backpacks_from(chunk).expect("Could not get backpacks");

        let common_items = backpacks
            .0
            .iter()
            .filter(|k| [&backpacks.1, &backpacks.2].iter().all(|s| s.contains(k)));
        let group_sum = common_items.fold(0, |a, b| a + get_char_value(b));
        all_badges_sum += group_sum;
    }

    println!("All badges sum: {}", all_badges_sum);
    Ok(())
}
