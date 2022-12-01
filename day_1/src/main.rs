extern crate core;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const INPUT_FILE_PATH: &str = "src/input.txt";

fn main() {
    let path = Path::new(INPUT_FILE_PATH);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file
    };

    let mut calories_sum: Vec<i32> = vec![];
    let mut temp_calories_count = 0;

    for line in io::BufReader::new(&file).lines() {
        let line_content = match line {
            Err(why) => panic!("Could not read line: {}", why),
            Ok(string) => string
        };

        if line_content.eq("") {
            calories_sum.push(temp_calories_count);
            temp_calories_count = 0;
            continue;
        }

        let calories = match line_content.parse::<i32>() {
            Ok(calories) => calories,
            Err(why) => panic!("Could not parse number to i32 int: {}", why)
        };

        temp_calories_count += calories;
    }
    calories_sum.sort();

    println!("1. Elve has {} Calories", calories_sum[calories_sum.len() - 1]);
    println!("2. Elve has {} Calories", calories_sum[calories_sum.len() - 2]);
    println!("3. Elve has {} Calories", calories_sum[calories_sum.len() - 3]);
    println!("Top 3 Elve's have {} calories in total", calories_sum[calories_sum.len() - 1] + calories_sum[calories_sum.len() - 2] + calories_sum[calories_sum.len() - 3])
}
