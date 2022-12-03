use std::fs::File;
use std::io::{BufRead, BufReader};

/// Small BST that has no sub-trees. Number of nodes match nicely with elves in
/// the problem.
///
/// Representation:
///
///#       b
///#     /   \
///#    a     c
///
/// where a < b and b < c
#[derive(Debug)]
struct TreeO {
    left: Option<u32>,
    right: Option<u32>,
    data: u32,
}

impl TreeO {
    fn new(data: u32) -> Self {
        TreeO {
            left: None,
            right: None,
            data,
        }
    }

    fn push(&mut self, data: u32) {
        // just add since left node is empty
        if data < self.data && self.left.is_none() {
            self.left = Some(data)
        }
        // just add since right node is empty
        if data > self.data && self.right.is_none() {
            self.right = Some(data)
        }

        // these the real deals

        // a = b, b = c, c = x
        if data > self.data && self.right.is_some() && data > self.right.unwrap() {
            self.left = Some(self.data);
            self.data = self.right.unwrap();
            self.right = Some(data)
        }

        // a = b, b = x
        if data > self.data && self.right.is_some() && data < self.right.unwrap() {
            self.left = Some(self.data);
            self.data = data
        }

        // a = x
        if data < self.data && self.left.is_some() && data > self.left.unwrap() {
            self.left = Some(data)
        }
    }

    fn sum(self) -> Result<u32, String> {
        if self.left.is_some() && self.right.is_some() {
            Ok(self.data + self.left.unwrap() + self.right.unwrap())
        } else {
            Err("A node is empty".to_string())
        }
    }
}

#[allow(dead_code)]
fn find_largest_calorie_held_by_an_elf(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_elf_calories_counter: u32 = 0;
    let mut shajara = TreeO::new(0);

    for line in reader.lines() {
        let line_data = line.unwrap();

        if !line_data.is_empty() {
            let calorie_entry = line_data.parse::<u32>().unwrap();
            println!("found data {}", calorie_entry);
            current_elf_calories_counter += calorie_entry;
            continue;
        }

        println!("found empty line");

        shajara.push(current_elf_calories_counter);
        current_elf_calories_counter = 0;
        println!("shajara {:?}", shajara);
    }

    // if last line is not empty, our custom logic does not run, so repeating here
    shajara.push(current_elf_calories_counter);
    println!("shajara {:?}", shajara);

    let result = shajara.sum();
    match result {
        Ok(calories_total) => calories_total.to_string(),
        Err(error_string) => panic!("{}", error_string),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_largest_if_input_in_ascending() {
        let file_path = "./inputs/day_1/mocks/asc.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "120");
    }

    #[test]
    fn should_find_largest_if_input_in_descending() {
        let file_path = "./inputs/day_1/mocks/desc.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "130");
    }

    #[test]
    fn should_find_largest_if_input_in_ascending_with_empty_line_at_end() {
        let file_path = "./inputs/day_1/mocks/asc_end_empty_line.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "120");
    }

    #[test]
    fn should_find_largest_if_input_in_descending_with_empty_line_at_end() {
        let file_path = "./inputs/day_1/mocks/desc_end_empty_line.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "130");
    }

    // #[test]
    // fn the_actual_test() {
    //     let file_path = "./inputs/day_1/aoc_input.txt";
    //     let result = find_largest_calorie_held_by_an_elf(file_path);
    //     assert_eq!(result, "100");
    // }
}
