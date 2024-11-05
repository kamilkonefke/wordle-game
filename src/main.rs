use std::io;

fn main() {
    let tries: usize = 5;
    let word: String = get_random_word(" ");
    let mut flags: [usize; 5] = [0, 0, 0, 0, 0]; // 0 - nothing; 1 - contains; 2 - at this place;

    println!("WORDLE!");

    for _ in 0..tries {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("ERR");

        // Check for win
        if input.trim() == word.trim() {
            println!("You win!");
            return;
        }

        // Check for correct char count
        if input.trim().len() > 5 {
            continue;
        }

        // Set flags
        for c in input.chars() {
            if word.find(c).is_some() {
                flags[input.find(c).unwrap()] = 1; // Containing char
            }
            if word.find(c) == input.find(c) {
                flags[word.find(c).unwrap()] = 2; // Char is on point
            }
        }
        println!("{}", string_from_flags(input, flags));
    }
}

fn get_random_word(_path: &str) -> String {
    // TODO - Read from text file and pick random line
    "abcde".to_string()
}

fn string_from_flags(_input: String, flags: [usize; 5]) -> String {
    let mut result = String::new();
    for i in 0..flags.len() {
        match flags[i] {
            0 => result.push_str("-"),
            1 => result.push_str("?"),
            2 => result.push_str("!"),
            _ => result.push_str("-"),
        }
    }
    result
}
