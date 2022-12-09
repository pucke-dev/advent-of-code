use std::error::Error;
use std::fs::read_to_string;

const INPUT_FILE_PATH: &str = "day_8/src/input.txt";

struct Tree {
    row: usize,
    col: usize,
    height: i32,
}

struct Forest(Vec<Vec<Tree>>);
impl Forest {
    pub fn from_string(string: String) -> Forest {
        let mut forest = Forest(vec![]);

        for (row, line) in string.lines().enumerate() {
            forest.0.push(vec![]);
            for (col, char) in line.chars().enumerate() {
                let current_row_length = &forest.0.len();
                forest.0[current_row_length - 1].push(Tree {
                    row,
                    col,
                    height: char.to_digit(10).unwrap() as i32,
                })
            }
        }

        forest
    }

    fn is_tree_visible(&self, tree: &Tree) -> bool {
        let visible_from_left = || -> bool {
            self.0[tree.row][..tree.col]
                .iter()
                .all(|t| t.height < tree.height)
        };
        let visible_from_right = || -> bool {
            self.0[tree.row][tree.col + 1..]
                .iter()
                .all(|t| t.height < tree.height)
        };
        let visible_from_above = || -> bool {
            self.0[..tree.row]
                .iter()
                .all(|t| t[tree.col].height < tree.height)
        };
        let visible_from_below = || -> bool {
            self.0[tree.row + 1..]
                .iter()
                .all(|t| t[tree.col].height < tree.height)
        };

        visible_from_left() || visible_from_right() || visible_from_above() || visible_from_below()
    }

    fn scenic_score(&self, tree: &Tree) -> i32 {
        let scenic_score_left = || -> i32 {
            let mut score = 0;
            for neighbor in self.0[tree.row][..tree.col].iter().rev() {
                score += 1;

                if neighbor.height >= tree.height {
                    break;
                }
            }
            score
        };
        let scenic_score_right = || -> i32 {
            let mut score = 0;
            for neighbor in self.0[tree.row][tree.col + 1..].iter() {
                score += 1;

                if neighbor.height >= tree.height {
                    break;
                }
            }
            score
        };
        let scenic_score_above = || -> i32 {
            let mut score = 0;
            for neighbor in self.0[..tree.row].iter().rev() {
                score += 1;

                if neighbor[tree.col].height >= tree.height {
                    break;
                }
            }
            score
        };
        let scenic_score_below = || -> i32 {
            let mut score = 0;
            for neighbor in self.0[tree.row + 1..].iter() {
                score += 1;

                if neighbor[tree.col].height >= tree.height {
                    break;
                }
            }
            score
        };

        scenic_score_left() * scenic_score_right() * scenic_score_above() * scenic_score_below()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(INPUT_FILE_PATH)?;

    let forest = Forest::from_string(input);

    let mut visible_trees = 0;
    let mut highest_scenic_score = 0;

    for trees in &forest.0 {
        for tree in trees {
            if forest.is_tree_visible(tree) {
                visible_trees += 1;
            }

            let tree_scenic_score = forest.scenic_score(tree);

            if tree_scenic_score > highest_scenic_score {
                highest_scenic_score = tree_scenic_score;
            }
        }
    }

    println!("Total number of visible tree's: {}", visible_trees);
    println!("Highest scenic score: {}", highest_scenic_score);
    Ok(())
}
