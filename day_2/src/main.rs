use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const INPUT_FILE_PATH: &str = "src/input.txt";

const POINTS_WIN: i32 = 6;
const POINTS_DRAW: i32 = 3;
const POINTS_LOOSE: i32 = 0;

const POINTS_ROCK: i32 = 1;
const POINTS_PAPER: i32 = 2;
const POINTS_SCISSOR: i32 = 3;

enum HandShape {
    Rock,
    Paper,
    Scissor,
}

struct Player {
    hand_shape: HandShape,
}

impl Player {
    fn get_hand_shape_score(&self) -> i32 {
        match self.hand_shape {
            HandShape::Rock => POINTS_ROCK,
            HandShape::Paper => POINTS_PAPER,
            HandShape::Scissor => POINTS_SCISSOR,
        }
    }
    pub fn new(str: &str) -> Player {
        match str {
            "A" => Player { hand_shape: HandShape::Rock },
            "B" => Player { hand_shape: HandShape::Paper },
            "C" => Player { hand_shape: HandShape::Scissor },
            "X" => Player { hand_shape: HandShape::Rock },
            "Y" => Player { hand_shape: HandShape::Paper },
            "Z" => Player { hand_shape: HandShape::Scissor },
            _ => panic!("Invalid hand shape")
        }
    }
}

struct Game {
    player_1: Player,
    player_2: Player,
}

impl Game {
    fn get_game_score(&self) -> i32 {
        match self.player_2.hand_shape {
            HandShape::Rock => {
                match self.player_1.hand_shape {
                    HandShape::Scissor => self.player_2.get_hand_shape_score() + POINTS_WIN,
                    HandShape::Rock => self.player_2.get_hand_shape_score() + POINTS_DRAW,
                    HandShape::Paper => self.player_2.get_hand_shape_score() + POINTS_LOOSE
                }
            }
            HandShape::Paper => {
                match self.player_1.hand_shape {
                    HandShape::Scissor => self.player_2.get_hand_shape_score() + POINTS_LOOSE,
                    HandShape::Rock => self.player_2.get_hand_shape_score() + POINTS_WIN,
                    HandShape::Paper => self.player_2.get_hand_shape_score() + POINTS_DRAW
                }
            }
            HandShape::Scissor => {
                match self.player_1.hand_shape {
                    HandShape::Scissor => self.player_2.get_hand_shape_score() + POINTS_DRAW,
                    HandShape::Rock => self.player_2.get_hand_shape_score() + POINTS_LOOSE,
                    HandShape::Paper => self.player_2.get_hand_shape_score() + POINTS_WIN
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(INPUT_FILE_PATH);
    let file = File::open(&path)?;

    let mut my_total_score: i32 = 0;
    for line in io::BufReader::new(&file).lines() {
        let line_content = line?;
        let strategy_guide: Vec<&str> = line_content.split(" ").collect();
        let player_1 = Player::new(strategy_guide[0]);
        let player_2 = Player::new(strategy_guide[1]);
        let game: Game = Game { player_1, player_2 };

        my_total_score += game.get_game_score();
    }

    println!("My total score: {}", my_total_score);
    Ok(())
}
