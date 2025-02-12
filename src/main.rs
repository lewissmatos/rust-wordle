use colored::{ColoredString, Colorize};
use std::io::{stdin, Result};
fn main() -> Result<()> {
    let word = "TRAIT";

    let mut chances = word.len();
    let mut has_won = false;
    println!("Please, type your word.\nYou have {} chances", chances);
    while chances > 0 && !has_won {
        let user_word = get_user_guess(&word).unwrap();
        has_won = check_word(&user_word, &word);
        chances -= 1;
    }
    match has_won {
        false => println!("Better luck next time!\nThe word was {word}"),
        true => println!("Congrats!!! You've won 🎉"),
    }
    Ok(())
}

fn get_user_guess(word: &str) -> Result<String> {
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

fn check_word(user_guess: &str, word: &str) -> bool {
    let mut feedback = Vec::<ColoredString>::new();
    let sanitized_user_guess = user_guess.trim().to_uppercase();
    let sanitized_word = word.trim().to_uppercase();
    sanitized_user_guess
        .chars()
        .enumerate()
        .for_each(|(u_idx, g_char)| {
            let is_green = sanitized_word.chars().nth(u_idx).unwrap_or_default() == g_char;
            if is_green {
                feedback.push(g_char.to_string().green().bold());
            } else if sanitized_word.chars().any(|w_char| w_char == g_char)
                && !feedback.iter().any(|char| char.input == g_char.to_string())
            {
                feedback.push(g_char.to_string().yellow());
            } else {
                feedback.push(g_char.to_string().white());
            }
        });
    let mut final_user_guess = String::new();
    for char in feedback.iter() {
        print!("{}", char);
        final_user_guess.push_str(&char.input);
    }
    println!();
    final_user_guess == sanitized_word
}
