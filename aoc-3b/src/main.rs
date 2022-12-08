use std::fs;
use itertools::Itertools;

const LOWER_CASE_OFFSET: u32 = 96;
const UPPER_CASE_OFFSET: u32 = 64;


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not load file for some raisen");

    let elf_groups = input.lines()
        .into_iter()
        .chunks(3);

    let badge = elf_groups.into_iter()
        .map(|iter| find_badge(iter.collect()))
        .map(|x| convert_item_to_score(x))
        .fold(0, |acc, x| acc + x);

    println!("total of badge values {}", badge as u32);
}

fn find_badge(rucksacks: Vec<&str>) -> char {
    get_common_item(rucksacks[0], rucksacks[1], rucksacks[2])
}

fn get_common_item(bag1: &str, bag2: &str, bag3: &str) -> char{
    bag1.chars()
        .filter(|x| bag2.contains(*x))
        .filter(|x| bag3.contains(*x))
        .next().unwrap()
}

fn convert_item_to_score(score: char) -> u32 {
    // since upper case have a priority of 26 higher, we can simply take 26 off its ascii offset
    score as u32 - if score.is_uppercase() {UPPER_CASE_OFFSET - 26} else {LOWER_CASE_OFFSET}

}
