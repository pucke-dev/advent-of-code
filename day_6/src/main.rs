use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

const INPUT_FILE_PATH: &str = "src/input.txt";

struct SlidingWindow<'a> {
    buffer: &'a [u8],
    window_size: usize,
    character_set: &'a mut HashSet<char>,
}

impl SlidingWindow<'_> {
    fn find_distinct_continuous_char_sequence(&mut self) -> Result<usize, String> {
        for (index, window) in self.buffer.windows(self.window_size).enumerate() {
            self.character_set.clear();

            for character in window {
                self.character_set.insert(*character as char);
            }

            if self.character_set.len() == self.window_size {
                return Ok(index + self.window_size);
            }
        }

        Err("no continuous distinct char sequence found".to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = read_to_string(INPUT_FILE_PATH).expect("Should have been able to read the file");

    let mut character_set: HashSet<char> = HashSet::new();

    // This only works, because we absolutely know that our string contains asci chars only.
    let mut packet_sliding_window = SlidingWindow {
        buffer: buffer.as_bytes(),
        window_size: 4,
        character_set: &mut character_set,
    };
    let packet_marker = packet_sliding_window.find_distinct_continuous_char_sequence()?;
    println!("packet marker index: {}", packet_marker);

    let mut message_sliding_window = SlidingWindow {
        buffer: buffer.as_bytes(),
        window_size: 14,
        character_set: &mut character_set,
    };
    let message_marker = message_sliding_window.find_distinct_continuous_char_sequence()?;
    println!("message marker index: {}", message_marker);

    Ok(())
}
