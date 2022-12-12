const INPUT_FILE_PATH: &str = "day_10/src/input.txt";

use std::error::Error;
use std::fs::read_to_string;

const COLS: usize = 40;
const ROWS: usize = 6;

struct Cpu {
    clock_cycle: usize,
    register: i32,
}

impl Cpu {
    fn compute_pixel(&self, col: usize) -> String {
        if col == (self.register - 1) as usize
            || col == self.register as usize
            || col == (self.register + 1) as usize
        {
            "#".to_string()
        } else {
            " ".to_string()
        }
    }
}

struct Crt {
    pixels: Vec<String>,
}

impl Crt {
    fn display(&self) {
        for (index, pixel) in self.pixels.iter().enumerate() {
            if index % 40 == 0 {
                println!();
                continue;
            }
            print!("{}", pixel);
        }
        println!()
    }
}

fn signal_of_interest(clock_cycle: usize) -> bool {
    matches!(clock_cycle, 20 | 60 | 100 | 140 | 180 | 220)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(INPUT_FILE_PATH)?;

    let mut total_signal_strength = 0;
    let mut cpu = Cpu {
        clock_cycle: 1,
        register: 1,
    };
    let mut crt = Crt {
        pixels: vec![" ".to_string(); (COLS + 1) * ROWS],
    };

    for line in input.lines() {
        if signal_of_interest(cpu.clock_cycle) {
            total_signal_strength += cpu.clock_cycle * cpu.register as usize;
        }

        crt.pixels[cpu.clock_cycle] = cpu.compute_pixel((cpu.clock_cycle - 1) % COLS);

        cpu.clock_cycle += 1;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if let ["addx", num] = parts[..] {
            if signal_of_interest(cpu.clock_cycle) {
                total_signal_strength += cpu.clock_cycle * cpu.register as usize;
            }

            crt.pixels[cpu.clock_cycle] = cpu.compute_pixel((cpu.clock_cycle - 1) % COLS);

            cpu.register += num.parse::<i32>().expect("`num` should be an i32");
            cpu.clock_cycle += 1;
        }
    }
    println!("Total signal strength: {}", total_signal_strength);

    crt.display();

    Ok(())
}
