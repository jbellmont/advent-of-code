use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();

    let mut most_calories_carried: i32 = 0;
    let mut sum_of_calories: i32 = 0;

    for line in lines {
        let line = line.unwrap();
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            sum_of_calories = 0;
            continue;
        } else {
            let calories: i32 = trimmed_line.parse().expect("Wanted a number");
            sum_of_calories += calories;
        }

        if sum_of_calories > most_calories_carried {
            most_calories_carried = sum_of_calories;
        }
    }

    println!("Most calories carried: {}", most_calories_carried)
}
