mod use_bank;
use rand::Rng;
use std::process;
use use_bank::select_word_bank;

fn load_words(w_len: u32) -> Vec<String> {
    let contents = select_word_bank(w_len)
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            String::new()
        })
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    if contents.is_empty() {
        eprintln!("Empty or invalid word list");
        eprintln!("Difficulty not available");
        process::exit(1);
    }
    contents
}

pub fn get_random_word(w_len: u32) -> String {
    let words = load_words(w_len);

    let word = words
        .get(generate_random_number(words.len()))
        .unwrap_or(&"".to_string())
        .to_string();

    if word.trim().len() != w_len as usize {
        return get_random_word(w_len);
    }
    remove_tildes(word.trim()).to_uppercase()
}

fn generate_random_number(limit: usize) -> usize {
    rand::rng().random_range(0..limit) as usize
}
fn remove_tildes(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'á' => 'a',
            'é' => 'e',
            'í' => 'i',
            'ó' => 'o',
            'ú' => 'u',
            'Á' => 'A',
            'É' => 'E',
            'Í' => 'I',
            'Ó' => 'O',
            'Ú' => 'U',
            'ñ' => 'n',
            'Ñ' => 'N',
            _ => c,
        })
        .collect()
}
