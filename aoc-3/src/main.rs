use std::fs;

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

const LOWER_CASE_OFFSET: u32 = 96;
const UPPER_CASE_OFFSET: u32 = 64;

impl Rucksack {
    fn from_string(full_bag: &str) -> Rucksack {
        let split = full_bag.split_at(full_bag.len() / 2);

        Rucksack { 
            compartment1: (String::from(split.0)),
            compartment2: (String::from(split.1)),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not load file for some raisen");

    let common_item = input.lines()
        .map(|x| Rucksack::from_string(x))
        .map(|x| get_common_item(&x))
        .map(|x| convert_item_to_score(x))
        .fold(0, |acc, x| acc + x);
    
    println!("total of all extra items {}", common_item as u32);
}

fn get_common_item(bag: &Rucksack) -> char {
    bag.compartment1.chars()
        .filter(|x| bag.compartment2.contains(*x))
        .next().unwrap()
}

fn convert_item_to_score(score: char) -> u32 {
    // since upper case have a priority of 26 higher, we can simply take 26 off its ascii offset
    score as u32 - if score.is_uppercase() {UPPER_CASE_OFFSET - 26} else {LOWER_CASE_OFFSET}

}
