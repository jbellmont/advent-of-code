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

    let mut calorie_amounts = Vec::new();
    let mut sum_of_calories: i32 = 0;

    for line in lines {
        let line = line.unwrap();
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            calorie_amounts.push(sum_of_calories);
            sum_of_calories = 0;
            continue;
        } else {
            let calories: i32 = trimmed_line.parse().expect("Wanted a number");
            sum_of_calories += calories;
        }
    }

    calorie_amounts.sort();
    let length = calorie_amounts.len();
    let sum_of_top_three =
        calorie_amounts[length - 1] + calorie_amounts[length - 2] + calorie_amounts[length - 3];

    println!("{:?}", calorie_amounts);
    println!("Sum of top three Elves' calories: {}", sum_of_top_three);
}
