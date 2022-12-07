use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub};

const INPUT_FILE_PATH: &str = "day_7/src/input.txt";

const TOTAL_DISK_SPACE: Size = Size(70000000);
const FREE_SPACE_NEEDED: Size = Size(30000000);

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Copy)]
struct Size(u32);
impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
impl Sub for Size {
    type Output = Size;

    fn sub(self, rhs: Self) -> Self::Output {
        Size(self.0 - rhs.0)
    }
}
impl Sum for Size {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Size(0), |a, b| {
            let tmp = a.0 + b.0;
            Size(tmp)
        })
    }
}
impl Add for Size {
    type Output = Size;

    fn add(self, rhs: Self) -> Self::Output {
        Size(self.0 + rhs.0)
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Path(String);
impl Path {
    pub fn from_array(paths: Vec<&str>) -> Path {
        Path(paths.join("/"))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(INPUT_FILE_PATH)?;

    let mut directories: HashMap<Path, Size> = HashMap::new();
    let mut affected_directories: Vec<&str> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "ls"] => continue,
            ["dir", _] => continue,
            ["$", "cd", ".."] => {
                affected_directories.pop();
            }
            ["$", "cd", name] => {
                affected_directories.push(name);
            }
            [size, _] => {
                let size: Size = Size(size.parse()?);
                for index in 0..affected_directories.len() {
                    let path = Path::from_array(affected_directories[..=index].to_vec());
                    *directories.entry(path).or_insert(Size(0)) += size;
                }
            }
            _ => {}
        };
    }

    println!(
        "Sum of all directorys with size < 100_00: {}",
        directories
            .clone()
            .into_values()
            .filter(|size| *size <= Size(100_000))
            .sum::<Size>()
            .0
    );

    let root = directories.get(&Path("/".to_string())).unwrap();
    let available = TOTAL_DISK_SPACE - *root;

    let recommended_directory = directories
        .into_values()
        .filter(|size| available + *size >= FREE_SPACE_NEEDED)
        .min()
        .unwrap();

    println!(
        "Should remove directory with size: {}",
        recommended_directory.0
    );

    Ok(())
}
