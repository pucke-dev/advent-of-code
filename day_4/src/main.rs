use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE_PATH: &str = "src/input.txt";

struct Section {
    start: i32,
    end: i32,
}

impl Section {
    fn completely_overlaps(&self, b: &Section) -> bool {
        b.start >= self.start && b.end <= self.end
    }

    fn partially_overlaps(&self, b: &Section) -> bool {
        (b.start >= self.start && b.start <= self.end) || (b.end <= self.end && b.end >= self.start)
    }
}

fn parse_line_into_sections(line: String) -> (Section, Section) {
    let (s1, s2) = line.split_once(",").expect("Could not split at char `,`");
    let (s1_start, s1_end) = s1.split_once("-").expect("Could not split at char `-`");
    let (s2_start, s2_end) = s2.split_once("-").expect("Could not split at char `-`");

    (
        Section {
            start: s1_start.parse::<i32>().expect("Could not parse to i32"),
            end: s1_end.parse::<i32>().expect("Could not parse to i32"),
        },
        Section {
            start: s2_start.parse::<i32>().expect("Could not parse to i32"),
            end: s2_end.parse::<i32>().expect("Could not parse to i32"),
        }
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(INPUT_FILE_PATH);
    let file = File::open(&path)?;

    let mut fully_overlapping_sections: i32 = 0;
    let mut partially_overlapping_sections: i32 = 0;

    for line in BufReader::new(&file).lines() {
        let (section_a, section_b) = parse_line_into_sections(line.expect("Could not parse line"));

        if section_a.completely_overlaps(&section_b) || section_b.completely_overlaps(&section_a) {
            fully_overlapping_sections += 1
        }
        if section_a.partially_overlaps(&section_b) || section_b.partially_overlaps(&section_a) {
            partially_overlapping_sections += 1
        }
    }

    println!("Number of fully overlapping sections: {}", fully_overlapping_sections);
    println!("Number of partially overlapping sections: {}", partially_overlapping_sections);

    Ok(())
}
