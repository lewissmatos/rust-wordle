use colored::{ColoredString, Colorize};
use std::{
    char,
    io::{stdin, Result},
};
pub type Feedback = Vec<ColoredString>;

pub fn get_user_guess(word: &str) -> Result<String> {
    let mut user_word = String::new();
    stdin().read_line(&mut user_word)?;

    if user_word.trim().chars().any(|c| !c.is_alphabetic()) {
        println!("{}", "Please, enter a valid word".blue().italic());
        return get_user_guess(word);
    } else if user_word.trim().len() != word.len() {
        println!(
            "{}",
            format!(
                "Please, enter a word with {} characters",
                word.len().to_string().bold()
            )
            .blue()
            .italic()
        );

        return get_user_guess(word);
    }

    Ok(user_word.trim().to_string())
}

pub fn check_word(user_guess: &str, word: &str, feedback_history: &mut Vec<Feedback>) -> bool {
    let sanitized_user_guess = user_guess.trim().to_uppercase();
    let sanitized_word = word.trim().to_uppercase();
    let mut feedback: Feedback = Vec::<ColoredString>::new();
    let mut word_chars: Vec<Option<char>> = sanitized_word.chars().map(Some).collect();

    // Fill all the chars and mark the full succeeded ones (the green ones)
    for (u_idx, g_char) in sanitized_user_guess.chars().enumerate() {
        if let Some(w_char) = word_chars[u_idx] {
            if g_char == w_char {
                feedback.push(g_char.to_string().green().bold());
                word_chars[u_idx] = None; // Mark this character as matched
            } else {
                feedback.push("".to_string().normal());
            }
        }
    }

    // Check if there are some coincidences (magenta ones) and also mark the white ones
    for (u_idx, g_char) in sanitized_user_guess.chars().enumerate() {
        if feedback[u_idx].to_string().is_empty() {
            if let Some(pos) = word_chars.iter().position(|&w_char| w_char == Some(g_char)) {
                feedback[u_idx] = g_char.to_string().magenta();
                word_chars[pos] = None; // Mark as matched
            } else {
                feedback[u_idx] = g_char.to_string().white();
            }
        }
    }

    // for char in feedback.iter() {
    //     print!("{}", char);
    // }
    feedback_history.push(feedback);
    println!();

    sanitized_user_guess == sanitized_word
}

pub fn print_feedback_history(history: &Vec<Feedback>) {
    for feedback in history.iter() {
        for letter in feedback.iter() {
            print!("{}", letter)
        }
        println!();
    }
}

pub fn options() -> Result<i32> {
    println!("\nSelect an option:\n[1]-Play now (Diff. 5)\n[2]-Select the difficulty and play\n[3]-Check stats\n[4]-Exit");
    let mut option = String::new();
    stdin().read_line(&mut option)?;

    Ok(option.trim().parse::<i32>().unwrap_or(0))
}

pub fn ask_word_length() -> u32 {
    println!(
        "{}",
        format!("Select the difficulty: from {} to {}", "5".red(), "9".red()).blue()
    );
    let mut word_length = String::new();
    match stdin().read_line(&mut word_length) {
        Ok(_) => {
            let len = word_length.trim().parse::<u32>().unwrap_or(0);

            if len < 5 || len > 9 {
                println!(
                    "{}",
                    format!(
                        "You must type a number from  from {} to {}",
                        "5".red(),
                        "9".red()
                    )
                    .blue()
                );
                ask_word_length()
            } else {
                len
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            ask_word_length()
        }
    }
}
