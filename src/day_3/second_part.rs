use std::{collections::HashMap, io::BufRead, str::FromStr};

use crate::read_file;

use super::first_part::{get_priority_of_item, MyBinarySearchTree};

#[derive(Debug)]
struct CommonItem {
    inner: HashMap<u8, u8>,
}

impl CommonItem {
    fn new() -> Self {
        CommonItem {
            inner: HashMap::new(),
        }
    }

    fn push(&mut self, data: u8) {
        if self.inner.get(&data).is_none() {
            self.inner.insert(data, 1);
        }

        let value = self.inner.get(&data).unwrap();
        self.inner.insert(data, value + 1);
    }
}

#[allow(dead_code)]
fn sum_of_badge_priorities(filename: &str) -> String {
    let reader = read_file(filename);

    let mut total_count: u32 = 0;
    let mut elf_bags: Vec<String> = Vec::new();
    let mut elf_counter = 0;

    for line in reader.lines() {
        let line_data = line.unwrap();
        println!("Reading: {}", line_data);
        elf_bags.push(line_data);
        elf_counter += 1;

        if elf_counter == 3 {
            println!("Calculating boundary for badge");
            let badge = find_badge_from_bags(&elf_bags[..]);
            println!("Counter: {} --- Attempt: {}", total_count, badge);
            total_count += u32::try_from(badge).unwrap();
            elf_counter = 0;
            elf_bags.clear();
        }
    }

    total_count.to_string()
}

#[allow(dead_code)]
fn find_badge_from_bags(elf_bags: &[String]) -> u8 {
    let mut iter = elf_bags.iter();
    let first_bag = iter.next().unwrap();
    let first_bag_to_represent_as_tree = MyBinarySearchTree::from_str(first_bag).unwrap();
    println!("Tree: {:?}", first_bag_to_represent_as_tree);
    let mut common_item_manager = CommonItem::new();

    for bag in iter {
        let mut already_added: Vec<u32> = Vec::new();

        for i in bag.chars() {
            let item_priority = u32::try_from(get_priority_of_item(i)).unwrap();
            if first_bag_to_represent_as_tree.does_item_exist(item_priority) {
                println!("Found potential: {}", item_priority);
                if already_added.contains(&item_priority) {
                    println!("Redundant, ignoring");
                    continue;
                }
                common_item_manager.push(u8::try_from(item_priority).unwrap());
                already_added.push(item_priority);
            }
        }
    }

    println!("My boi status pls: {:?}", common_item_manager);

    for (k, v) in common_item_manager.inner {
        return match v == 3 {
            true => k,
            false => continue,
        };
    }

    panic!("Shouldn't reach here")
}

#[cfg(test)]
mod tests {
    use super::sum_of_badge_priorities;

    #[test]
    fn should_get_sum_of_badge_priorities_of_a_group() {
        let file_path = "./inputs/day_3/mocks/example.txt";
        let result = sum_of_badge_priorities(file_path);
        assert_eq!(result, "70")
    }

    #[test]
    fn should_get_sum_of_badge_priorities_of_a_group_custom() {
        let file_path = "./inputs/day_3/mocks/my_example.txt";
        let result = sum_of_badge_priorities(file_path);
        assert_eq!(result, "3")
    }

    #[test]
    #[ignore]
    fn actual_test() {
        let file_path = "./inputs/day_3/aoc_input.txt";
        let result = sum_of_badge_priorities(file_path);
        assert_eq!(result, "70")
    }
}
