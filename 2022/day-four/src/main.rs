use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data").unwrap();
    let assignment_pairs = parse_data(&data);
    println!("Part 1: {}", part_one(&assignment_pairs));
}

fn parse_data(data: &str) -> Vec<((u32, u32), (u32, u32))> {
    data.lines()
        .map(|assignment_pair_string| parse_assignment_pair(assignment_pair_string))
        .collect()
}

fn parse_assignment_pair(assignment_pair: &str) -> ((u32, u32), (u32, u32)) {
    let mut assignment_pair = assignment_pair.split(',');
    let first_assignment = parse_assignment(assignment_pair.next().unwrap());
    let second_assignment = parse_assignment(assignment_pair.next().unwrap());
    (first_assignment, second_assignment)
}

fn parse_assignment(assignment: &str) -> (u32, u32) {
    let mut assignment = assignment.split('-');
    let first = assignment.next().unwrap().parse().unwrap();
    let second = assignment.next().unwrap().parse().unwrap();
    (first, second)
}

fn determine_assignment_overlap(assignment_pair: ((u32, u32), (u32, u32))) -> bool {
    let first_assignment = assignment_pair.0;
    let second_assignment = assignment_pair.1;

    let first_assignment_start = first_assignment.0;
    let first_assignment_end = first_assignment.1;

    let second_assignment_start = second_assignment.0;
    let second_assignment_end = second_assignment.1;

    if first_assignment_start <= second_assignment_start
        && second_assignment_end <= first_assignment_end
        || first_assignment_start >= second_assignment_start
            && second_assignment_end >= first_assignment_end
    {
        return true;
    }

    false
}

fn part_one(assignment_pairs: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut number_of_overlaps = 0;

    for assignment_pair in assignment_pairs {
        if determine_assignment_overlap(*assignment_pair) {
            number_of_overlaps += 1;
        }
    }

    number_of_overlaps
}
