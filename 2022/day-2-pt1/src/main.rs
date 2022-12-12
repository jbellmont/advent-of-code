use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn get_score_from_game(opponent_move: &str, player_move: &str) -> i16 {
    let shape_scores: HashMap<&str, i16> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let outcome_scores: HashMap<&str, i16> = HashMap::from([("draw", 3), ("win", 6)]);

    let mut moves: String = opponent_move.to_owned();
    moves.push_str(player_move);

    let outcome_score: i16 = match moves.as_str() {
        "AY" | "BZ" | "CX" => outcome_scores["win"],
        "AX" | "BY" | "CZ" => outcome_scores["draw"],
        &_ => 0,
    };

    let shape_score = shape_scores[player_move];

    return shape_score + outcome_score;
}

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

    let mut total_score: i16 = 0;

    for line in lines {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split_whitespace().collect();
        let opponent_move = moves[0];
        let player_move = moves[1];

        total_score += get_score_from_game(opponent_move, player_move)
    }

    println!("Total score: {}", total_score);
}
