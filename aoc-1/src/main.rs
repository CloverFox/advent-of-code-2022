use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not load file for some raisen");

    let max_calorie = input.split("\n\n")
        .map(|elf| elf.lines()
             .map(|calorie| calorie.trim().parse::<i32>().unwrap())
             .fold(0, |acc, x| acc + x))
        .max().unwrap();

    println!("the max calorie count is {}", max_calorie);
}
