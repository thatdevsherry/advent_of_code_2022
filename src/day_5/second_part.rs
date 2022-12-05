use std::{io::BufRead, str::FromStr};

use crate::{day_5::first_part::Operation, read_file};

use super::first_part::Crane;

impl Crane {
    fn perform_operation_9001_model(operation: &Operation, dock: &mut [Vec<char>]) {
        let source = usize::try_from(operation.move_from_stack - 1).unwrap();
        let destination = usize::try_from(operation.destination_stack - 1).unwrap();
        let move_repetition = usize::try_from(operation.move_repetition).unwrap();
        let stack = dock.get_mut(source).unwrap();
        let movable_crates: Vec<char> = stack.drain(stack.len() - move_repetition..).collect();
        let destination_stack = dock.get_mut(destination).unwrap();
        for i in movable_crates {
            destination_stack.push(i);
        }
    }
}

#[allow(dead_code)]
fn run_day_five_second_part(filename: &str, dock: &mut Vec<Vec<char>>) -> String {
    let reader = read_file(filename);

    for line in reader.lines() {
        let instruction = line.unwrap();
        let operation = Operation::from_str(&instruction).unwrap();
        Crane::perform_operation_9001_model(&operation, dock);
    }

    println!("Final Stacks: {:?}", dock);

    Crane::get_top_crates(dock.to_vec())
}
#[cfg(test)]
mod tests {
    use super::run_day_five_second_part;

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

        let result = run_day_five_second_part(file_path, dock.as_mut());
        assert_eq!(result, "???");
    }
}
