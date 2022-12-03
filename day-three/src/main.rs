use std::fs;

fn main() {
    let challenge_data = fs::read_to_string("data").expect("Unable to read file");

    let rucksacks = parse_challenge_data(challenge_data);

    let part_one = part_one(rucksacks.clone());

    println!("Part one: {}", part_one);

    let part_two = part_two(rucksacks.clone());

    println!("Part two: {}", part_two);
}

#[derive(Clone)]
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

    fn all_items(&self) -> Vec<char> {
        let mut items = self.compartment_one.clone();
        items.extend(self.compartment_two.clone());
        items
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

fn determine_group_badge(group_rucksacks: [Rucksack; 3]) -> char {
    group_rucksacks[0].all_items().iter()
        .filter(|item| group_rucksacks[1].all_items().contains(item))
        .filter(|item| group_rucksacks[2].all_items().contains(item))
        .next()
        .map(|item| *item)
        .unwrap()
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

fn part_two(rucksacks: Vec<Rucksack>) -> i32 {
    let mut total = 0;
    for rucksack in rucksacks.chunks(3) {
        let group_rucksacks = [
            rucksack[0].clone(),
            rucksack[1].clone(),
            rucksack[2].clone(),
        ];
        let group_badge = determine_group_badge(group_rucksacks);
        total += get_item_priority(group_badge) as i32;
    }
    total
}
