use rand::prelude::*;
use std::io::BufRead;

pub enum Roll {
    Roll4([u8; 4]),
    Roll5([u8; 5]),
}

pub fn roll(ask: bool, long: bool) -> Roll {
    let lim = if long { 5 } else { 4 };
    let mut rolls = Vec::with_capacity(lim);
    for _ in 0..lim {
        if ask {
            print!("Input a dice roll from 1 to 6: ");
            // Flush stdout so that the prompt appears
            use std::io::Write;
            std::io::stdout()
                .lock()
                .flush()
                .ok()
                .expect("Could not flush stdout");

            let value = std::io::stdin()
                .lock()
                .lines() // Get the lines
                .next() // The next line
                .map(|v| {
                    // Unwrap the value within the Option returned by next(), or return an ""
                    v.unwrap_or(String::new())
                })
                .unwrap_or(String::new()) // Unwrap the outer Option, or return a ""
                .parse::<u8>() // Parse it into a numer, or
                .unwrap_or_else(|e| {
                    // Unwrap it and print the error
                    eprintln!("Error reading input: {}", e);
                    std::process::exit(127)
                });
            // Ensure value is in the proper bounds
            let value = (value - 1) % 6 + 1;
            rolls.push(value);
        } else {
            // Fix this. Not secure
            rolls.push(thread_rng().gen_range(1, 7));
        }
    }

    if long {
        Roll::Roll5([rolls[0], rolls[1], rolls[2], rolls[3], rolls[4]])
    } else {
        Roll::Roll4([rolls[0], rolls[1], rolls[2], rolls[3]])
    }
}
