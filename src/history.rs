type AttemptReg = HashMap<String, u32>;
const FILE_NAME: &str = "history.txt";

use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Result, Write},
};
pub fn save_history(user_attempt: usize, has_won: bool, word: &str) -> Result<()> {
    let history_contents = fs::read_to_string(FILE_NAME).unwrap_or_else(|_| {
        println!("Failed to read from the file. Creating file...");
        File::create(FILE_NAME).unwrap_or_else(|e| {
            println!("Failed to create the file: {}", e);
            File::create(FILE_NAME).unwrap()
        });
        String::new()
    });

    let summary: Vec<String> = history_contents.lines().map(|x| x.to_string()).collect();
    let mut history_header: String = summary.iter().next().unwrap_or(&"".to_string()).to_string();
    let mut attempts: Vec<String> = summary.iter().skip(1).cloned().collect();

    if history_contents.is_empty() {
        // Fill the history.txt file for the very first time
        fill_file(&mut history_header, &mut attempts, has_won, &word);
    } else {
        // Update he history header
        update_header(&mut history_header, has_won);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_NAME)?;

    let mut attempt_regs: AttemptReg = attempts
        .iter()
        .map(|attempt| {
            let at_vec = attempt.split(":").collect::<Vec<&str>>();
            let attempt_number = at_vec[0].trim().to_string();
            let attempt_value = at_vec[1].trim().parse::<u32>().unwrap_or(0);
            (attempt_number, attempt_value)
        })
        .collect();

    if has_won {
        *attempt_regs.entry(user_attempt.to_string()).or_insert(0) += 1;
    }

    let mut sorted_attempts: Vec<_> = attempt_regs.iter().collect();
    sorted_attempts.sort_by_key(|attempt| attempt.0.parse::<u32>().unwrap_or(0));
    let stringified_attempts: String = sorted_attempts
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n");

    file.write_all(&format!("{}\n{}", history_header, stringified_attempts).as_bytes())?;

    Ok(())
}

fn fill_file(history_header: &mut String, attempts: &mut Vec<String>, has_won: bool, word: &str) {
    *history_header = format!(
        "Plays: 1 | Victories: {} | %Vic: {}%",
        if has_won { 1 } else { 0 },
        if has_won { 100.00 } else { 0.00 },
    );

    for i in 1..=word.len() {
        attempts.push(format!("{}: 0", i));
    }
}

fn update_header(history_header: &mut String, has_won: bool) {
    let numeric_data = history_header
        .split('|')
        .filter_map(|text| {
            let vals = text.split(":").collect::<Vec<&str>>();
            let number = vals[1].trim().replace("%", "").parse::<u32>().unwrap_or(0);
            Some(number)
        })
        .collect::<Vec<u32>>();

    let plays = numeric_data[0] + 1;
    let victories = numeric_data[1] + if has_won { 1 } else { 0 };
    let victories_per = (victories as f64 / plays as f64) * 100.0;

    *history_header = format!(
        "Plays: {} | Victories: {} | %Vic: {:.2}%",
        plays, victories, victories_per
    );
}
