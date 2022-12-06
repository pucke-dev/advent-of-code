use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const INPUT_FILE_PATH: &str = "src/input.txt";

const POINTS_WIN: i32 = 6;
const POINTS_DRAW: i32 = 3;
const POINTS_LOSS: i32 = 0;

const POINTS_ROCK: i32 = 1;
const POINTS_PAPER: i32 = 2;
const POINTS_SCISSOR: i32 = 3;

enum HandShape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Win,
    Loss,
    Draw,
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
            "A" => Player {
                hand_shape: HandShape::Rock,
            },
            "B" => Player {
                hand_shape: HandShape::Paper,
            },
            "C" => Player {
                hand_shape: HandShape::Scissor,
            },
            "X" => Player {
                hand_shape: HandShape::Rock,
            },
            "Y" => Player {
                hand_shape: HandShape::Paper,
            },
            "Z" => Player {
                hand_shape: HandShape::Scissor,
            },
            _ => panic!("Invalid hand shape"),
        }
    }
}

struct Game<'a> {
    player_1: &'a Player,
    player_2: &'a Player,
}

impl Game<'_> {
    fn get_game_score(&self) -> i32 {
        match self.player_2.hand_shape {
            HandShape::Rock => match self.player_1.hand_shape {
                HandShape::Scissor => self.player_2.get_hand_shape_score() + POINTS_WIN,
                HandShape::Rock => self.player_2.get_hand_shape_score() + POINTS_DRAW,
                HandShape::Paper => self.player_2.get_hand_shape_score() + POINTS_LOSS,
            },
            HandShape::Paper => match self.player_1.hand_shape {
                HandShape::Scissor => self.player_2.get_hand_shape_score() + POINTS_LOSS,
                HandShape::Rock => self.player_2.get_hand_shape_score() + POINTS_WIN,
                HandShape::Paper => self.player_2.get_hand_shape_score() + POINTS_DRAW,
            },
            HandShape::Scissor => match self.player_1.hand_shape {
                HandShape::Scissor => self.player_2.get_hand_shape_score() + POINTS_DRAW,
                HandShape::Rock => self.player_2.get_hand_shape_score() + POINTS_LOSS,
                HandShape::Paper => self.player_2.get_hand_shape_score() + POINTS_WIN,
            },
        }
    }
}

struct GameWithOutcome<'a> {
    player_1: &'a Player,
    result: Outcome,
}

impl GameWithOutcome<'_> {
    pub fn new<'a>(player_1: &'a Player, outcome: &str) -> GameWithOutcome<'a> {
        match outcome {
            "X" => GameWithOutcome {
                player_1,
                result: Outcome::Loss,
            },
            "Y" => GameWithOutcome {
                player_1,
                result: Outcome::Draw,
            },
            "Z" => GameWithOutcome {
                player_1,
                result: Outcome::Win,
            },
            _ => panic!("Invalid game outcome"),
        }
    }
    fn get_game_score(&self) -> i32 {
        match self.result {
            Outcome::Win => match self.player_1.hand_shape {
                HandShape::Rock => POINTS_PAPER + POINTS_WIN,
                HandShape::Scissor => POINTS_ROCK + POINTS_WIN,
                HandShape::Paper => POINTS_SCISSOR + POINTS_WIN,
            },
            Outcome::Loss => match self.player_1.hand_shape {
                HandShape::Rock => POINTS_SCISSOR + POINTS_LOSS,
                HandShape::Scissor => POINTS_PAPER + POINTS_LOSS,
                HandShape::Paper => POINTS_ROCK + POINTS_LOSS,
            },
            Outcome::Draw => match self.player_1.hand_shape {
                HandShape::Rock => POINTS_ROCK + POINTS_DRAW,
                HandShape::Scissor => POINTS_SCISSOR + POINTS_DRAW,
                HandShape::Paper => POINTS_PAPER + POINTS_DRAW,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(INPUT_FILE_PATH);
    let file = File::open(path)?;

    let mut my_total_score: i32 = 0;
    let mut my_new_total_score: i32 = 0;

    for line in io::BufReader::new(&file).lines() {
        let line_content = line?;

        let strategy_guide: Vec<&str> = line_content.split(' ').collect();

        let player_1 = Player::new(strategy_guide[0]);
        let player_2 = Player::new(strategy_guide[1]);

        let game: Game = Game {
            player_1: &player_1,
            player_2: &player_2,
        };
        let game_with_outcome = GameWithOutcome::new(&player_1, strategy_guide[1]);

        my_total_score += game.get_game_score();
        my_new_total_score += game_with_outcome.get_game_score();
    }

    println!("My total score: {}", my_total_score);
    println!("My new total score: {}", my_new_total_score);
    Ok(())
}
