use colored::{ColoredString, Colorize};
use std::io::{stdin, Result};

pub fn get_user_guess(word: &str) -> Result<String> {
    let mut user_word = String::new();
    stdin().read_line(&mut user_word)?;

    if user_word.trim().chars().any(|c| !c.is_alphabetic()) {
        println!("Please, enter a valid word");
        return get_user_guess(word);
    } else if user_word.trim().len() != word.len() {
        println!("Please, enter a word with {} characters", word.len());
        return get_user_guess(word);
    }

    Ok(user_word.trim().to_string())
}

pub fn check_word(user_guess: &str, word: &str) -> bool {
    let sanitized_user_guess = user_guess.trim().to_uppercase();
    let sanitized_word = word.trim().to_uppercase();
    let mut feedback = Vec::<ColoredString>::new();
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

    // Check if there are some coincidences (yellows ones) and also mark the white ones
    for (u_idx, g_char) in sanitized_user_guess.chars().enumerate() {
        if feedback[u_idx].to_string().is_empty() {
            if let Some(pos) = word_chars.iter().position(|&w_char| w_char == Some(g_char)) {
                feedback[u_idx] = g_char.to_string().yellow();
                word_chars[pos] = None; // Mark as matched
            } else {
                feedback[u_idx] = g_char.to_string().white();
            }
        }
    }

    for char in feedback.iter() {
        print!("{}", char);
    }

    println!();

    sanitized_user_guess == sanitized_word
}
