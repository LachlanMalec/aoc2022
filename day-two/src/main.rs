use std::fs;

fn main() {
    let move_combinations: Vec<String> = parse_challenge_data();

    // in the form of (challenger_move_key, defender_move_key, move_value)
    let move_values: [(char, char, i32); 3] = [
        ('A', 'X', 1), // rock
        ('B', 'Y', 2), // paper
        ('C', 'Z', 3), // scissors
    ];

    let win_conditions: [(char, char); 3] = [
        ('C', 'X'), // rock beats scissors
        ('A', 'Y'), // paper beats rock
        ('B', 'Z'), // scissors beats paper
    ];

    let win_value: i32 = 6;

    let draw_value: i32 = 3;

    let total_score_following_guide: i32 = part_one(
        move_combinations,
        win_conditions,
        move_values,
        win_value,
        draw_value,
    );

    println!(
        "Total score following guide: {}",
        total_score_following_guide
    );
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

fn determine_move(move_key: char) -> Move {
    match move_key {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!("Invalid move key"),
    }
}

fn parse_challenge_data() -> Vec<String> {
    let challenge_data = fs::read_to_string("data").expect("Unable to read file");
    challenge_data.lines().map(|s| s.to_string()).collect()
}

fn part_one(
    move_combinations: Vec<String>,
    win_conditions: [(char, char); 3],
    move_values: [(char, char, i32); 3],
    win_value: i32,
    draw_value: i32,
) -> i32 {
    let mut total_score: i32 = 0;

    for move_combination in move_combinations {
        let challenger_move = move_combination.chars().nth(0).unwrap();
        let your_move = move_combination.chars().nth(2).unwrap();

        total_score += match move_values
            .iter()
            .find(|(_, your_move_key, _)| *your_move_key == your_move)
        {
            Some((_, _, move_value)) => *move_value,
            None => panic!("Invalid move"),
        };

        total_score += match win_conditions
            .iter()
            .find(|(challenger_move_key, your_move_key)| {
                *challenger_move_key == challenger_move && *your_move_key == your_move
            }) {
            Some(_) => win_value,
            None => 0,
        };

        total_score += match move_values
            .iter()
            .find(|(challenger_move_key, your_move_key, _)| {
                *challenger_move_key == challenger_move && *your_move_key == your_move
            }) {
            Some(_) => draw_value,
            None => 0,
        };

        total_score += match move_values
            .iter()
            .find(|(challenger_move_key, your_move_key, _)| {
                *challenger_move_key == your_move && *your_move_key == challenger_move
            }) {
            Some(_) => draw_value,
            None => 0,
        };
    }

    return total_score;
}
