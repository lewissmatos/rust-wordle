mod history;
mod input;
use history::save_history;
use input::{check_word, get_user_guess};
use std::io::Result;
fn main() -> Result<()> {
    let word = "AMADA";
    let mut chances = word.len() + 1;
    let mut has_won = false;
    println!("Please, type your word.\nYou have {} chances", chances);
    while chances > 0 && !has_won {
        let user_word = get_user_guess(&word).unwrap();
        has_won = check_word(&user_word, &word);
        chances -= 1;
    }
    if has_won {
        println!("Congrats!!! You've won 🎉");
    } else {
        println!("Better luck next time!\nThe word was {word}");
    }
    let _ = save_history(word.len() + 1 - chances, has_won, word);

    Ok(())
}
