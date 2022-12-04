use std::io::BufRead;
use std::str::FromStr;

use crate::{day_4::first_part::AssignmentPair, read_file};

#[allow(dead_code)]
fn find_number_of_pairs_overlapping(filename: &str) -> u32 {
    let reader = read_file(filename);

    let mut number_of_overlapping_pairs: u32 = 0;

    for line in reader.lines() {
        let line_data = line.unwrap();

        let pair: Vec<&str> = line_data.split(',').collect();
        debug_assert!(pair.len() == 2);
        println!("First section range: {}", pair[0]);
        println!("Second section range: {}", pair[1]);

        let first_section_range = AssignmentPair::from_str(pair[0]).unwrap();
        let second_section_range = AssignmentPair::from_str(pair[1]).unwrap();
        let has_overlapping_pairs =
            AssignmentPair::has_overlap(&first_section_range, &second_section_range);
        println!("Has overlap: {}", has_overlapping_pairs);
        if has_overlapping_pairs {
            number_of_overlapping_pairs += 1
        }
    }

    number_of_overlapping_pairs
}

#[cfg(test)]
mod tests {
    use super::find_number_of_pairs_overlapping;

    #[test]
    fn should_find_number_of_pairs_overlapping() {
        let file_path = "./inputs/day_4/mocks/example.txt";
        let result = find_number_of_pairs_overlapping(file_path);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn actual_test() {
        let file_path = "./inputs/day_4/aoc_input.txt";
        let result = find_number_of_pairs_overlapping(file_path);
        assert_eq!(result, 2);
    }
}
