use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    match extract_reports() {
        Ok(reports) => {
            let safe_reports = count_safe_reports(&reports);
            println!("Safe reports: {}", safe_reports);
        }
        Err(_e) => {
            println!("Could not parse file");
        }
    };
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> u32 {
    let mut count: u32 = 0;

    for report in reports {
        if brute_force_is_safe_report(report) {
            count += 1;
        }
    }

    count
}

// Part 1
fn is_safe_report(report: &[i32]) -> bool {
    let mut current_level: &i32 = &report[0];

    let is_increasing: bool = *current_level < report[1];

    for next_level in report.iter().skip(1) {
        let first_level: &i32;
        let second_level: &i32;

        if is_increasing {
            first_level = next_level;
            second_level = current_level;
        } else {
            first_level = current_level;
            second_level = next_level;
        };

        let difference: i32 = first_level - second_level;
        if !(1..=3).contains(&difference) {
            return false;
        }
        current_level = next_level
    }

    true
}

// Part 2
fn brute_force_is_safe_report(report: &[i32]) -> bool {
    if is_safe_report(report) {
        return true;
    }

    let report_length = report.len();
    for idx in 0..report_length {
        let mut new_report: Vec<i32> = Vec::new();
        if idx > 0 {
            new_report.extend_from_slice(&report[0..idx]);
        }
        if idx < report_length - 1 {
            new_report.extend_from_slice(&report[(idx + 1)..]);
        }

        if is_safe_report(&new_report) {
            return true;
        }
    }

    false
}

fn extract_reports() -> io::Result<Vec<Vec<i32>>> {
    // Open the file
    let file = File::open("src/reports.txt")?;
    let reader = BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for (report_number, line) in reader.lines().enumerate() {
        let line = line?; // Handle potential I/O errors

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split the line into a vector of i32
        let report: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect();

        match report {
            Ok(report) => {
                reports.push(report);
            }
            Err(_e) => {
                println!("Could not parse report {} on line {}", line, report_number);
            }
        }
    }
    Ok(reports)
}
