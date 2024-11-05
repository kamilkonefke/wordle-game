use std::io;

fn main() {
    println!("WORDLE!");

    let tries: usize = 5;
    let word: String = "kurwa".to_string();
    let mut flags: [usize; 5] = [0,0,0,0,0]; // 0 - nothing; 1 - contains; 2 - at this place;

    for _ in 0..tries {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("ERR");

        for cn in user_input.chars() {
            if word.find(cn).is_some() {
                // println!("Contains!");
                flags[user_input.find(cn).unwrap()] = 1;
            }
            if word.find(cn) == user_input.find(cn) {
                // println!("At this place!");
                flags[word.find(cn).unwrap()] = 2;
            }
        }

        println!("{}", flags_from_str(user_input, flags));
    }
}

fn flags_from_str(_input: String, flags: [usize; 5]) -> String {
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
