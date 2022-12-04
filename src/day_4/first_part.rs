use std::{io::BufRead, str::FromStr};

use crate::read_file;

#[derive(Debug)]
pub struct AssignmentPair {
    start: u8,
    end: u8,
}

impl AssignmentPair {
    pub fn is_a_range_subset_of_other(first_range: &Self, second_range: &Self) -> bool {
        (first_range.start <= second_range.start && second_range.end <= first_range.end)
            || (first_range.start >= second_range.start && second_range.end >= first_range.end)
    }

    pub fn has_overlap(first_range: &Self, second_range: &Self) -> bool {
        // Now this was interesting, instead of matching all cases that DO overlap,
        // It ended up being easier to just check for cases that DO NOT overlap, and
        // that case was super concise.
        !(first_range.end < second_range.start || first_range.start > second_range.end)
    }
}

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range_split: Vec<&str> = s.split('-').collect();
        debug_assert!(range_split.len() == 2);
        Ok(AssignmentPair {
            start: u8::from_str(range_split[0]).unwrap(),
            end: u8::from_str(range_split[1]).unwrap(),
        })
    }
}

#[allow(dead_code)]
fn number_of_assignment_pairs_fully_containing_other(filename: &str) -> u32 {
    let reader = read_file(filename);

    let mut number_of_pairs_completely_overlapping_the_other: u32 = 0;

    for line in reader.lines() {
        let line_data = line.unwrap();

        let pair: Vec<&str> = line_data.split(',').collect();
        debug_assert!(pair.len() == 2);
        println!("First section range: {}", pair[0]);
        println!("Second section range: {}", pair[1]);

        let first_section_range = AssignmentPair::from_str(pair[0]).unwrap();
        let second_section_range = AssignmentPair::from_str(pair[1]).unwrap();
        println!("First section range: {:?}", first_section_range);
        println!("Second section range: {:?}", second_section_range);
        let is_a_pair_subset_of_other =
            AssignmentPair::is_a_range_subset_of_other(&first_section_range, &second_section_range);
        println!("Is subset: {}", is_a_pair_subset_of_other);
        if is_a_pair_subset_of_other {
            number_of_pairs_completely_overlapping_the_other += 1
        }
    }

    number_of_pairs_completely_overlapping_the_other
}

#[cfg(test)]
mod tests {
    use super::number_of_assignment_pairs_fully_containing_other;

    #[test]
    fn should_find_number_of_pairs_being_subset_example() {
        let file_path = "./inputs/day_4/mocks/example.txt";
        let result = number_of_assignment_pairs_fully_containing_other(file_path);
        assert_eq!(result, 2);
    }

    #[test]
    #[ignore]
    fn actual_test() {
        let file_path = "./inputs/day_4/aoc_input.txt";
        let result = number_of_assignment_pairs_fully_containing_other(file_path);
        assert_eq!(result, 2);
    }
}
