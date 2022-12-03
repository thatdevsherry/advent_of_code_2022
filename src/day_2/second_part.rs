use std::io::BufRead;
use std::str::FromStr;

use crate::read_file;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn my_outcome_for_round(my_choice: &Choice, elf_choice: &Choice) -> Outcome {
        match my_choice {
            Choice::Rock => match elf_choice {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::TakeTheL,
                Choice::Scissors => Outcome::TakeTheW,
            },
            Choice::Paper => match elf_choice {
                Choice::Rock => Outcome::TakeTheW,
                Choice::Paper => Outcome::Draw,
                Choice::Scissors => Outcome::TakeTheL,
            },
            Choice::Scissors => match elf_choice {
                Choice::Rock => Outcome::TakeTheL,
                Choice::Paper => Outcome::TakeTheW,
                Choice::Scissors => Outcome::Draw,
            },
        }
    }
}

impl FromStr for Choice {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    TakeTheW,
    TakeTheL,
    Draw,
}

impl Outcome {
    fn score(&self) -> u8 {
        match self {
            Self::TakeTheL => 0,
            Self::Draw => 3,
            Self::TakeTheW => 6,
        }
    }

    fn my_choice(elf_choice: &Choice, expected_outcome: &Outcome) -> Choice {
        match expected_outcome {
            Outcome::Draw => match elf_choice {
                Choice::Rock => Choice::Rock,
                Choice::Paper => Choice::Paper,
                Choice::Scissors => Choice::Scissors,
            },
            Outcome::TakeTheL => match elf_choice {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::TakeTheW => match elf_choice {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
        }
    }
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::TakeTheL),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::TakeTheW),
            _ => panic!(),
        }
    }
}

#[allow(dead_code)]
fn get_score_according_to_strategy_guide(filename: &str) -> String {
    let reader = read_file(filename);

    let mut my_score: u32 = 0;

    for line in reader.lines() {
        let line_data = line.unwrap();

        let line_split: Vec<&str> = line_data.split(' ').collect();
        debug_assert!(line_split.len() == 2);
        let elf_choice_encrypted = line_split[0];
        let outcome_to_create_encrypted = line_split[1];
        let elf_choice = Choice::from_str(elf_choice_encrypted).unwrap();
        // let outcome_to_create = Choice::from_str(outcome_to_create_encrypted).unwrap();
        let outcome_to_create = Outcome::from_str(outcome_to_create_encrypted).unwrap();
        println!(
            "Opponent choice: {:?} => {:?}",
            elf_choice_encrypted, elf_choice
        );
        println!(
            "My Choice to make: {:?} => {:?}",
            outcome_to_create_encrypted, outcome_to_create
        );
        let my_choice = Outcome::my_choice(&elf_choice, &outcome_to_create);
        let my_choice_score = my_choice.score();
        let outcome_score = outcome_to_create.score();
        my_score += u32::try_from(outcome_score + my_choice_score).unwrap();
    }

    my_score.to_string()
    // "0".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day_2::second_part::get_score_according_to_strategy_guide;

    #[test]
    fn should_calculate_score_of_example() {
        let file_path = "./inputs/day_2/mocks/example.txt";
        let result = get_score_according_to_strategy_guide(file_path);
        assert_eq!(result, "12");
    }

    #[test]
    fn should_win_all() {
        let file_path = "./inputs/day_2/mocks/second_part/win.txt";
        let result = get_score_according_to_strategy_guide(file_path);
        assert_eq!(result, "24");
    }

    #[test]
    fn should_lose_all() {
        let file_path = "./inputs/day_2/mocks/second_part/lose.txt";
        let result = get_score_according_to_strategy_guide(file_path);
        assert_eq!(result, "6");
    }

    #[test]
    fn should_draw() {
        let file_path = "./inputs/day_2/mocks/second_part/draw.txt";
        let result = get_score_according_to_strategy_guide(file_path);
        assert_eq!(result, "15");
    }

    // #[test]
    // fn actual_test() {
    //     let file_path = "./inputs/day_2/aoc_input.txt";
    //     let result = get_score_according_to_strategy_guide(file_path);
    //     assert_eq!(result, "15");
    // }
}
