use std::{io::BufRead, str::FromStr};

use crate::read_file;

struct Crane;

impl Crane {
    fn perform_operation(operation: &Operation, dock: &mut Vec<Vec<char>>) {
        for _move_repetition in 0..operation.move_repetition {
            println!("Operation: {:?}", operation);
            println!("Dock {:?}", dock);
            let source = usize::try_from(operation.move_from_stack - 1).unwrap();
            let destination = usize::try_from(operation.destination_stack - 1).unwrap();
            let stack = dock.get_mut(source).unwrap();
            let movable_crate = stack.pop().unwrap();
            let destination_stack = dock.get_mut(destination).unwrap();
            destination_stack.push(movable_crate);
        }
    }

    fn get_top_crates(dock: Vec<Vec<char>>) -> String {
        let mut top_crates = "".to_string();
        for stack in dock {
            let top_crate_of_stack = stack.last().unwrap().to_owned();
            top_crates.push(top_crate_of_stack);
        }
        top_crates
    }
}

#[derive(Debug)]
struct Operation {
    move_repetition: u8,
    move_from_stack: u8,
    destination_stack: u8,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" from ").collect();
        debug_assert!(split.len() == 2);
        let move_repetition: Vec<&str> = split[0].split("move ").collect();
        debug_assert!(move_repetition.len() == 2);
        let move_operation: Vec<&str> = split[1].split(" to ").collect();
        debug_assert!(move_operation.len() == 2);
        Ok(Operation {
            move_repetition: u8::from_str(move_repetition[1]).unwrap(),
            move_from_stack: u8::from_str(move_operation[0]).unwrap(),
            destination_stack: u8::from_str(move_operation[1]).unwrap(),
        })
    }
}

#[allow(dead_code)]
fn run_day_five_first_part(filename: &str, dock: &mut Vec<Vec<char>>) -> String {
    let reader = read_file(filename);

    for line in reader.lines() {
        let instruction = line.unwrap();
        let operation = Operation::from_str(&instruction).unwrap();
        Crane::perform_operation(&operation, dock);
    }

    println!("Final Stacks: {:?}", dock);

    Crane::get_top_crates(dock.to_vec())
    // "LOL".to_string()
}
#[cfg(test)]
mod tests {
    use super::run_day_five_first_part;

    #[test]
    fn test_example() {
        let file_path = "./inputs/day_5/aoc_input.txt";
        // I didn't want to write a parser for this, so added by hand.
        let mut dock: Vec<Vec<char>> = vec![
            vec!['S', 'C', 'V', 'N'],
            vec!['Z', 'M', 'J', 'H', 'N', 'S'],
            vec!['M', 'C', 'T', 'G', 'J', 'N', 'D'],
            vec!['T', 'D', 'F', 'J', 'W', 'R', 'M'],
            vec!['P', 'F', 'H'],
            vec!['C', 'T', 'Z', 'H', 'J'],
            vec!['D', 'P', 'R', 'Q', 'F', 'S', 'L', 'Z'],
            vec!['C', 'S', 'L', 'H', 'D', 'F', 'P', 'W'],
            vec!['D', 'S', 'M', 'P', 'F', 'N', 'G', 'Z'],
        ];
        debug_assert!(dock.len() == 9);

        let result = run_day_five_first_part(file_path, dock.as_mut());
        assert_eq!(result, "???");
    }
}
