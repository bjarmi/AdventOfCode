use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    match extract_lists() {
        Ok(lists) => {
            let sum = get_sum_of_distances(&lists.0, &lists.1);
            println!("Sum of distances: {}", sum);

            let score = get_similarity_score(&lists.0, &lists.1);
            println!("Similarity score: {}", score);
        }
        Err(_e) => {
            println!("Could not parse file")
        }
    }
}

// Solution to Part 1
fn get_sum_of_distances(left: &[u32], right: &[u32]) -> u32 {
    // Clone and sort the lists
    let mut sorted_left = left.to_vec();
    sorted_left.sort();

    let mut sorted_right = right.to_vec();
    sorted_right.sort();

    sorted_left
        .iter()
        .zip(sorted_right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn get_similarity_score(left: &[u32], right: &[u32]) -> u32 {
    let mut score = 0;

    for left_num in left {
        let mut appearances = 0;
        for right_num in right {
            if *left_num == *right_num {
                appearances += *left_num;
            }
        }

        score += appearances;
    }

    score
}

fn extract_lists() -> io::Result<(Vec<u32>, Vec<u32>)> {
    // Open the file
    let file = File::open("src/lists.txt")?;
    let reader = BufReader::new(file);

    // Initialize the vectors to store numbers
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    // Iterate over each line in the file
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; // Handle potential I/O errors

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split the line into parts
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Check if the line has exactly two parts
        if parts.len() != 2 {
            eprintln!(
                "Warning: Line {} does not contain exactly two numbers: '{}'",
                line_number + 1,
                line
            );
            continue; // Skip this line
        }

        // Parse the numbers and handle potential parsing errors
        match (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            (Ok(left_num), Ok(right_num)) => {
                left_numbers.push(left_num);
                right_numbers.push(right_num);
            }
            _ => {
                eprintln!(
                    "Warning: Line {} contains invalid numbers: '{}'",
                    line_number + 1,
                    line
                );
                continue; // Skip this line
            }
        }
    }
    Ok((left_numbers, right_numbers))
}
