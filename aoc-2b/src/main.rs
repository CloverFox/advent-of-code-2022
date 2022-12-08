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
            "A" => Some(Weapon::Rock),
            "B" => Some(Weapon::Paper),
            "C" => Some(Weapon::Scissors),
            _ => None,
        }
    }
    fn find_winner(&self) -> Weapon {
        match self {
            Weapon::Rock => Weapon::Paper,
            Weapon::Paper => Weapon::Scissors,
            Weapon::Scissors => Weapon::Rock,
        }
    }
    fn find_draw(&self) -> Weapon {
        return self.clone();
    }
    fn find_loss(&self) -> Weapon {
        match self {
            Weapon::Rock => Weapon::Scissors,
            Weapon::Paper => Weapon::Rock,
            Weapon::Scissors => Weapon::Paper,
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

    let rounds: Vec<Round> = input.lines()
        .map(|line| line.trim()
             .split_whitespace()
            )
        .map(|mut iter| {
            let opponent_choice: Weapon = Weapon::from(iter.next().unwrap()).unwrap();
            let desired_oucome: &str = iter.next().unwrap();

            Round { 
                player_choice: determine_weapon(&opponent_choice, desired_oucome), 
                opponent_choice: (opponent_choice) 
            }
        }).collect();

    print!("{:#?}", rounds);

    let total_score: i32 = rounds.iter()
        .map(|round| determine_score(&round))
        .fold(0, |acc, x| acc + x);

    println!("total score: {}", total_score);
}

fn determine_weapon(opponent_choice: &Weapon, outcome: &str) -> Weapon {
    match outcome {
        "X" => Some(opponent_choice.find_loss()),
        "Y" => Some(opponent_choice.find_draw()),
        "Z" => Some(opponent_choice.find_winner()),
        _ => None
    }.unwrap()
}

fn determine_score(round: &Round) -> i32 {
    return round.player_choice.beats(&round.opponent_choice) + &round.player_choice.get_score();
}
