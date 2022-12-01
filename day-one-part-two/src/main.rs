use std::fs;

fn main() {
    let data_string = fs::read_to_string("data").expect("Unable to read file");

    let elves_array: Vec<Vec<i32>> = parse_string(&data_string);

    let elves_sum_array: Vec<i32> = elves_array.iter().map(|x: &Vec<i32> | x.iter().sum()).collect();

    let mut elves_sum_array_sorted = elves_sum_array.clone();

    elves_sum_array_sorted.sort();

    println!("{}", elves_sum_array_sorted[elves_sum_array_sorted.len() - 1]);
}

fn parse_string(input: &str) -> Vec<Vec<i32>> {
    let mut elves: Vec<Vec<i32>> = Vec::new();

    let mut elf: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse::<i32>().unwrap());
        }
    }

    return elves;
}