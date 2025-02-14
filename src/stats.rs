type AttemptReg = HashMap<String, u32>;
const FILE_NAME: &str = "data/stats.txt";

use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Result, Write},
};

use colored::Colorize;
pub fn save_stats(user_attempt: usize, has_won: bool) -> Result<()> {
    let (stats_contents, mut stats_header, summary) = load_stats_contents();

    let mut attempts: Vec<String> = summary.iter().skip(1).cloned().collect();

    if stats_contents.is_empty() {
        // Fill the stats.txt file for the very first time
        fill_file(&mut stats_header, &mut attempts, has_won);
    } else {
        // Update he stats header
        update_header(&mut stats_header, has_won);
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

    file.write_all(&format!("{}\n{}", stats_header, stringified_attempts).as_bytes())?;

    Ok(())
}

fn fill_file(stats_header: &mut String, attempts: &mut Vec<String>, has_won: bool) {
    *stats_header = format!(
        "Plays: 1 | Victories: {} | %Vic: {}%",
        if has_won { 1 } else { 0 },
        if has_won { 100.00 } else { 0.00 },
    );

    for i in 1..=10 {
        attempts.push(format!("{}: 0", i));
    }
}

fn update_header(stats_header: &mut String, has_won: bool) {
    let numeric_data = stats_header
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

    *stats_header = format!(
        "Plays: {} | Victories: {} | %Vic: {:.2}%",
        plays, victories, victories_per
    );
}

fn load_stats_contents() -> (String, String, Vec<String>) {
    let stats_contents = fs::read_to_string(FILE_NAME).unwrap_or_else(|_| {
        println!("Failed to read from the file. Creating file...");
        File::create(FILE_NAME).unwrap_or_else(|e| {
            println!("Failed to create the file: {}", e);
            File::create(FILE_NAME).unwrap()
        });
        String::new()
    });

    let summary: Vec<String> = stats_contents.lines().map(|x| x.to_string()).collect();
    let stats_header: String = summary.iter().next().unwrap_or(&"".to_string()).to_string();

    (stats_contents, stats_header, summary)
}
pub fn display_stats() {
    let (stats_contents, stats_header, _) = load_stats_contents();

    let mut iterable = stats_contents.lines().into_iter();

    let numeric_data = stats_header
        .split('|')
        .filter_map(|text| {
            let vals = text.split(":").collect::<Vec<&str>>();
            let number = vals[1].trim().replace("%", "").parse::<u32>().unwrap_or(0);
            Some(number)
        })
        .collect::<Vec<u32>>();

    let victories = numeric_data[1];
    println!("{}", iterable.next().unwrap_or("").blue().bold());
    for line in iterable {
        let vals = line.split(":").collect::<Vec<_>>();
        let attempt = (vals[0], vals[1]);
        let (k, v) = attempt;
        println!(
            "{k}:{v}",
            k = k.white(),
            v = format!(
                "{} ({:.2}%)",
                v,
                (v.trim().parse::<f64>().unwrap_or(0.0) / victories as f64) * 100.0
            )
            .green()
        )
    }
}
