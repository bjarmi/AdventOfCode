use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;
use std::collections::HashMap;


/// Reads lines from a file at a given filepath and returns an iterator over the lines.
///
/// # Arguments
/// * `filepath` - A string slice that holds the path to the file to be read.
///
/// # Returns
/// A Result containing either an iterator over the lines of the file, 
/// or an error encapsulated in a Box<dyn Error> if the file could not be opened.
fn read_file_lines(filepath: &str) -> Result<impl Iterator<Item = Result<String, io::Error>>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

/// Calculates a calibration value from a string.
/// The value is composed of the first and last numeric characters in the string, concatenated.
///
/// # Arguments
/// * `s` - A string slice to extract the calibration value from.
///
/// # Returns
/// An i32 representing the concatenated value of the first and last numeric characters.
/// Returns 0 if no numeric characters are found or if the concatenation does not produce a valid number.
fn get_calibration_value(s: &str) -> i32 {
    let number_map: HashMap<&str, i32> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].into_iter().collect();

    let mut first_number: Option<i32> = None;
    let mut last_number: Option<i32> = None;

    let length = s.len();

    for (i, ch) in s.char_indices() {
        // Check first number
        if ch.is_numeric() && first_number.is_none() {
            first_number = Some(ch.to_digit(10).unwrap() as i32);

        } else if first_number.is_none() && ch.is_alphabetic(){
            // Check if the first word is a number

            for j in 3..6 {
                if i+j >= length {
                    break;
                }

                let word = &s[i..i+j];
                if let Some(number) = number_map.get(word) {
                    first_number = Some(*number);
                    break;
                }
            }
        }


        // Check last number
        let second_char = s.chars().nth(length - i - 1).unwrap();
        if second_char.is_numeric() && last_number.is_none() {
            last_number = Some(s.chars().nth(length - i - 1).unwrap().to_digit(10).unwrap() as i32);

        } else if last_number.is_none() && second_char.is_alphabetic() {
            // Check if the last word is a number
            for j in 3..6 {
                if i+j >= length {
                    break;
                }
                let word = &s[length-(j+i)..length - i];
                if let Some(number) = number_map.get(word) {
                    last_number = Some(*number);
                    break;
                }
            }
        }

    }
    
    match (first_number, last_number) {
        (Some(first), Some(last)) => {
            // Convert the first and last digits to i32 and concatenate them
            return format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
        },
        _ => 0
    }
}

fn main() {
    // Read lines from a file and calculate the sum of calibration values
    match read_file_lines("data.txt") {
        Ok(lines) => {
            // Sum the calibration values from each line
            let sum_calibration_values = lines
                .filter_map(Result::ok)
                .map(|s| get_calibration_value(&s))
                .sum::<i32>();
            println!("Sum of calibration values: {}", sum_calibration_values);
        },
        Err(err) => println!("Error: {}", err),
    }
}
