use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not load file for some raisen");

    let mut elf_calories = input.split("\n\n")
        .map(|elf| elf.lines()
             .map(|calorie| calorie.trim().parse::<i32>().unwrap())
             .fold(0, |acc, x| acc + x))
        .collect::<Vec<i32>>();

    elf_calories.sort_by(|a, b| b.cmp(a)); 

    let max_three_elves = elf_calories.iter()
        .take(3)
        .fold(0, |acc, x| acc + x);

    println!("the total of the top 3 calorie counts is {}", max_three_elves);
}
