use colored::Colorize;
use std::process;
use wordle::*;

fn main() {
    let option_number = options();

    match option_number {
        Ok(val) => match val {
            1 => play(5),
            2 => start_game(),
            3 => display_stats(),
            4 => process::exit(1),
            _ => {
                main();
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    }
    main()
}

fn start_game() {
    play(ask_word_length())
}

fn play(w_len: u32) {
    let word = get_random_word(w_len);
    let mut chances = word.len() + 1;
    let mut has_won = false;
    let mut feedback_history = Vec::<Feedback>::new();
    println!(
        "Please, type your word ({} Letters).\nYou have {} chances",
        word.len(),
        chances
    );
    while chances > 0 && !has_won {
        let user_word = get_user_guess(&word).unwrap();
        has_won = check_word(&user_word, &word, &mut feedback_history);
        chances -= 1;
        clear_console();
        print_feedback_history(&feedback_history);
    }
    if has_won {
        println!("{}", "Congrats!!! You've won 🎉".green().underline());
    } else {
        println!(
            "{}",
            format!(
                "Better luck next time!\nThe word was {}",
                word.red().underline()
            ),
        );
    }
    let _ = save_stats(word.len() + 1 - chances, has_won);
}
fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
