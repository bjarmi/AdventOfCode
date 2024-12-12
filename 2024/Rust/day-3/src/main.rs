use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    println!("Hello, world!");

    match extract_memmory() {
        Ok(memmory) => {
            parse_memmory_to_uncurrupted_muls(memmory);
            println!("Safe reports");
        }
        Err(_e) => {
            println!("Could not parse file");
        }
    };
}

fn parse_memmory_to_uncurrupted_muls(memmory: String) {
    let mut cum_suma: i32 = 0;
    let mut expected_next_char
}

fn extract_memmory() -> io::Result<String> {
    // Open the file
    let file = File::open("src/reports.txt")?;
    let reader = BufReader::new(file);

    // iterate over all characters in the file
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line?);
    }

    Ok(contents)
}
