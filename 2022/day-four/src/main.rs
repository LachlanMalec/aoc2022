use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data").unwrap();
    let assignment_pairs = parse_data(&data);
    println!("Part 1: {}", part_one(&assignment_pairs));
    println!("Part 2: {}", part_two(&assignment_pairs));
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

#[derive(PartialEq)]
enum AssignmentOverlap {
    Total,
    Partial,
    None,
}

fn determine_assignment_overlap(assignment_pair: ((u32, u32), (u32, u32))) -> AssignmentOverlap {
    let a1 = assignment_pair.0;
    let a2 = assignment_pair.1;

    match (a1.0, a1.1, a2.0, a2.1) {
        (a1_lower, a1_upper, a2_lower, a2_upper) if a1_lower <= a2_lower && a1_upper >= a2_upper => {
            AssignmentOverlap::Total
        }
        (a1_lower, a1_upper, a2_lower, a2_upper) if a1_lower >= a2_lower && a1_upper <= a2_upper => {
            AssignmentOverlap::Total
        }
        (a1_lower, a1_upper, a2_lower, _) if a1_lower <= a2_lower && a1_upper >= a2_lower => {
            AssignmentOverlap::Partial
        }
        (a1_lower, a1_upper, _, a2_upper) if a1_lower <= a2_upper && a1_upper >= a2_upper => {
            AssignmentOverlap::Partial
        }
        _ => AssignmentOverlap::None,
    }
}

fn part_one(assignment_pairs: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut number_of_total_overlaps = 0;

    for assignment_pair in assignment_pairs {
        if determine_assignment_overlap(*assignment_pair) == AssignmentOverlap::Total {
            number_of_total_overlaps += 1;
        }
    }

    number_of_total_overlaps
}

fn part_two(assignment_pairs: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut number_of_overlaps = 0;

    for assignment_pair in assignment_pairs {
        if determine_assignment_overlap(*assignment_pair) != AssignmentOverlap::None {
            number_of_overlaps += 1;
        }
    }

    number_of_overlaps
}