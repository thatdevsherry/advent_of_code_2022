use std::{io::BufRead, str::FromStr};

use crate::read_file;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, PartialEq)]
pub struct MyBinarySearchTree {
    left: Option<Box<MyBinarySearchTree>>,
    right: Option<Box<MyBinarySearchTree>>,
    data: u32,
}

impl MyBinarySearchTree {
    pub fn new(input: u32) -> Self {
        MyBinarySearchTree {
            left: None,
            right: None,
            data: input,
        }
    }

    pub fn push(&mut self, input: u32) {
        if input < self.data && self.left.is_none() {
            let branch = MyBinarySearchTree::new(input);
            self.left = Some(Box::new(branch))
        }
        if input > self.data && self.right.is_none() {
            let branch = MyBinarySearchTree::new(input);
            self.right = Some(Box::new(branch))
        }

        // wow recursive
        if input < self.data && self.left.is_some() {
            self.left.as_mut().unwrap().push(input);
        }
        if input > self.data && self.right.is_some() {
            self.right.as_mut().unwrap().push(input);
        }
    }

    pub fn does_item_exist(&self, input: u32) -> bool {
        if input < self.data && self.left.is_some() {
            self.left.as_ref().unwrap().does_item_exist(input)
        } else if input > self.data && self.right.is_some() {
            self.right.as_ref().unwrap().does_item_exist(input)
        } else {
            input == self.data
        }
    }
}

impl FromStr for MyBinarySearchTree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let first_char = iter.next().unwrap();
        let item_priority = u32::try_from(get_priority_of_item(first_char)).unwrap();
        let mut shajara = MyBinarySearchTree::new(item_priority);
        for i in iter {
            let item_priority = u32::try_from(get_priority_of_item(i)).unwrap();
            shajara.push(item_priority);
        }
        Ok(shajara)
    }
}

pub fn get_priority_of_item(input: char) -> u8 {
    let matched_char_index = CHARS.chars().position(|x| x == input).unwrap();
    u8::try_from(matched_char_index + 1).unwrap()
}

#[allow(dead_code)]
fn sum_of_item_priorities(filename: &str) -> String {
    let reader = read_file(filename);

    let mut sum_of_priorities = 0;

    for line in reader.lines() {
        let items_in_bag = line.unwrap();
        let common_item_priority: u32;

        // since all compartments hold equal number of items, the total number
        // of items in each bag will always be an even number, so we can split
        // the items string mid-way (i.e. half of total length).
        let items_split_into_comparts = items_in_bag.split_at(items_in_bag.len() / 2);
        let items_in_first_compartment = items_split_into_comparts.0;
        let items_in_second_compartment = items_split_into_comparts.1;
        debug_assert!(
            items_in_bag.len()
                == items_in_first_compartment.len() + items_in_second_compartment.len()
        );
        println!("Items in first compartment: {}", items_in_first_compartment);
        let first_compartment_shajara =
            MyBinarySearchTree::from_str(items_in_first_compartment).unwrap();
        println!(
            "Items in second compartment: {}",
            items_in_second_compartment
        );
        println!("First compartment shajara: {:?}", first_compartment_shajara);

        for i in items_in_second_compartment.chars() {
            let item_priority = u32::try_from(get_priority_of_item(i)).unwrap();
            if first_compartment_shajara.does_item_exist(item_priority) {
                println!("Found common: {}", item_priority);
                common_item_priority = item_priority;
                sum_of_priorities += common_item_priority;
                break;
            }
        }
    }

    sum_of_priorities.to_string()
}

#[cfg(test)]
mod tests {
    use super::sum_of_item_priorities;

    #[test]
    fn should_find_sum_of_priorities() {
        let file_path = "./inputs/day_3/mocks/example.txt";
        let result = sum_of_item_priorities(file_path);
        assert_eq!(result, "157")
    }

    #[test]
    #[ignore]
    fn actual_test() {
        let file_path = "./inputs/day_3/aoc_input.txt";
        let result = sum_of_item_priorities(file_path);
        assert_eq!(result, "157")
    }
}
