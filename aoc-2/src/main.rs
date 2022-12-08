use std::fs;

#[derive(Clone, Debug)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

impl Weapon {
    fn get_score(&self) -> i32 {
        match self {
            Weapon::Rock => 1,
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
        }
    }
    fn beats(&self, other: &Weapon) -> i32 {
        match self {
            Weapon::Rock => {
                match other {
                    Weapon::Rock => DRAW,
                    Weapon::Paper => LOSS,
                    Weapon::Scissors => WIN,
                }
            }
            Weapon::Paper => {
                match other {
                    Weapon::Rock => WIN,
                    Weapon::Paper => DRAW,
                    Weapon::Scissors => LOSS,
                }
            }
            Weapon::Scissors => {
                match other {
                    Weapon::Rock => LOSS,
                    Weapon::Paper => WIN,
                    Weapon::Scissors => DRAW,
                }
            }

        }
    }
    fn from(input: &str) -> Option<Weapon> {
        match input {
            "A"|"X" => Some(Weapon::Rock),
            "B"|"Y" => Some(Weapon::Paper),
            "C"|"Z" => Some(Weapon::Scissors),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Round {
    player_choice: Weapon,
    opponent_choice: Weapon,
}

impl Round {
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not load file for some raisen");

    let weapons: Vec<Vec<Weapon>> = input.lines()
        .map(|line| line.trim()
             .split_whitespace()
             .map(|choice| Weapon::from(choice).unwrap())
             .collect()
            )
        .collect();

    let rounds: Vec<Round> = weapons.iter()
        .map(|vector| Round {
            player_choice: vector[1].clone(),
            opponent_choice: vector[0].clone(),
        })
    .collect();

    let total_score: i32 = rounds.iter()
        .map(|round| determine_score(&round))
        .fold(0, |acc, x| acc + x);

    println!("total score: {}", total_score);
}

fn determine_score(round: &Round) -> i32 {
    return round.player_choice.beats(&round.opponent_choice) + &round.player_choice.get_score();
}
