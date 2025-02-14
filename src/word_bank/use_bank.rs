use std::{fs, io::Result};
pub fn select_word_bank(word_length: u32) -> Result<String> {
    let contents = fs::read_to_string(format!("data/{}_WORD_BANK.txt", word_length))?;
    Ok(contents)
}
