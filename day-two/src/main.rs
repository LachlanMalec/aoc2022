use std::fs;

fn main() {
    let move_combinations: Vec<String> = read_challenge_data();

    let move_values: [(Move, i32); 3] = [
        (Move::Rock, 1),
        (Move::Paper, 2),
        (Move::Scissors, 3),
    ];

    // in the form of (winning, losing)
    let win_conditions: [(Move, Move); 3] = [
        (Move::Rock, Move::Scissors),
        (Move::Paper, Move::Rock),
        (Move::Scissors, Move::Paper),
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

#[derive(PartialEq)]
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

fn read_challenge_data() -> Vec<String> {
    let challenge_data = fs::read_to_string("data").expect("Unable to read file");
    challenge_data.lines().map(|s| s.to_string()).collect()
}

fn part_one(
    move_combinations: Vec<String>,
    win_conditions: [(Move, Move); 3],
    move_values: [(Move, i32); 3],
    win_value: i32,
    draw_value: i32,
) -> i32 {
    let mut total_score: i32 = 0;

    for move_combination in move_combinations {
        let challenger_move = determine_move(move_combination.chars().nth(0).unwrap());
        let my_move = determine_move(move_combination.chars().nth(2).unwrap());

        total_score += match move_values.iter().find(|(move_value_key, _)| move_value_key == &my_move) {
            Some((_, my_move_value)) => my_move_value,
            None => panic!("Invalid move"),
        };

        total_score += match win_conditions
            .iter()
            .find(|(winning_move, losing_move)| winning_move == &my_move && losing_move == &challenger_move) {
            Some(_) => win_value,
            None => 0,
        };

        total_score += match my_move == challenger_move {
            true => draw_value,
            false => 0,
        };
    }

    return total_score;
}
