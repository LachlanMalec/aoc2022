use std::fs;

fn main() {
    let game_data: Vec<String> = read_challenge_data();

    const MOVE_VALUES: [(Move, i32); 3] = [
        (Move::Rock, 1),
        (Move::Paper, 2),
        (Move::Scissors, 3),
    ];

    // in the form of (winning, losing)
    const WIN_CONDITIONS: [(Move, Move); 3] = [
        (Move::Rock, Move::Scissors),
        (Move::Paper, Move::Rock),
        (Move::Scissors, Move::Paper),
    ];

    let win_value: i32 = 6;

    let draw_value: i32 = 3;

    let part_one_score: i32 = part_one(
        game_data.clone(),
        WIN_CONDITIONS,
        MOVE_VALUES,
        win_value,
        draw_value,
    );

    println!(
        "Part one score: {}",
        part_one_score
    );

    let part_two_score: i32 = part_two(
        game_data.clone(),
        WIN_CONDITIONS,
        MOVE_VALUES,
        win_value,
        draw_value,
    );

    println!(
        "Part two score : {}",
        part_two_score
    );
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Draw,
    Loss,
}

fn determine_move(move_key: char) -> Move {
    match move_key {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!("Invalid move key"),
    }
}

fn determine_result(result_key: char) -> Result {
    match result_key {
        'Z' => Result::Win,
        'Y' => Result::Draw,
        'X' => Result::Loss,
        _ => panic!("Invalid result key"),
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

fn part_two(
    game_plan: Vec<String>,
    win_conditions: [(Move, Move); 3],
    move_values: [(Move, i32); 3],
    win_value: i32,
    draw_value: i32,
) -> i32 {
    let mut total_score: i32 = 0;

    for game_result in game_plan {
        let challenger_move = determine_move(game_result.chars().nth(0).unwrap());
        let game_outcome = determine_result(game_result.chars().nth(2).unwrap());

        let my_move = match game_outcome {
            Result::Win => match win_conditions
                .iter()
                .find(|(_,losing_move)| losing_move == &challenger_move) {
                Some((winning_move, _)) => {
                    total_score += win_value;
                    winning_move
                },
                None => panic!("Invalid move"),
            },
            Result::Draw => {
                total_score += draw_value;
                &challenger_move
            },
            Result::Loss => match win_conditions
                .iter()
                .find(|(winning_move, _)| winning_move == &challenger_move) {
                Some((_, losing_move)) => losing_move,
                None => panic!("Invalid move"),
            },
        };

        total_score += match move_values.iter().find(|(move_value_key, _)| move_value_key == my_move) {
            Some((_, my_move_value)) => my_move_value,
            None => panic!("Invalid move"),
        };
    }

    total_score
}
