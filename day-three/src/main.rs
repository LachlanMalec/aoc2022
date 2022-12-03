use std::fs;

fn main() {
    let challenge_data = fs::read_to_string("data").expect("Unable to read file");

    let rucksacks = parse_challenge_data(challenge_data);

    let part_one = part_one(rucksacks);

    println!("Part one: {}", part_one);
}

struct Rucksack {
    compartment_one: Vec<char>,
    compartment_two: Vec<char>,
}

impl Rucksack {
    fn duplicated_item(&self) -> Option<char> {
        for item in &self.compartment_one {
            if self.compartment_two.contains(item) {
                return Some(*item);
            }
        }
        None
    }
}

fn parse_challenge_data(challenge_data: String) -> Vec<Rucksack> {
    challenge_data.lines()
        .map(|line| {
            let total_items = line.chars().count();
            let half_items = total_items / 2;
            let compartment_one = line.chars().take(half_items).collect();
            let compartment_two = line.chars().skip(half_items).collect();
            Rucksack {
                compartment_one,
                compartment_two,
            }
        })
        .collect()
}

fn get_item_priority(item: char) -> u8 {
    if item.is_lowercase() {
        item as u8 - 96
    } else {
        item as u8 - 38
    }
}

fn part_one(rucksacks: Vec<Rucksack>) -> i32 {
    let mut total = 0;
    for rucksack in rucksacks {
        if let Some(duplicated_item) = rucksack.duplicated_item() {
            total += get_item_priority(duplicated_item) as i32;
        }
    }
    total
}
