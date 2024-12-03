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
    let mut count = 0;

    for report in reports {
        if is_safe_report(report) {
            count += 1;
        }
    }

    count
}

fn is_safe_report(report: &[i32]) -> bool {
    let mut found_different_levels = false;
    let mut is_increasing = true;

    let current_level = report[0];
    for next_level in report.iter().skip(1) {
        if current_level == *next_level {
            continue;
        }

        if !found_different_levels {
            found_different_levels = true;
            is_increasing = current_level < *next_level;
        }

        let first_level = if is_increasing {
            *next_level
        } else {
            current_level
        };
        let second_level = if is_increasing {
            current_level
        } else {
            *next_level
        };

        let difference = first_level - second_level;
        if !(0..=3).contains(&difference) {
            return false;
        }
    }

    true
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
