use std::io::BufRead;

use crate::read_file;

/// Working is as follows.
///
/// It starts looping over the input file, and has an internal counter
/// for an individual elf.
///
/// On each line iter, it will keep adding to the individual elf counter
/// until we hit an empty line, after which it will add the total calories
/// of the individual elf into a variable, which holds total calories of previous
/// elf calculated. It also resets the individual elf counter to start fresh for next elf.
///
/// On next iter, it will do the same but the difference is that after it encounters
/// an empty line, it'll compare the individual elf total calories with the previous
/// elf calories, and keep the one that is the largest. This flow continues until EOF.
///
/// This way, we do not need to store all elf calories in an array or do any sorting on it.
/// This was my initial plan, but the problem can be solved without needing an array.
#[allow(dead_code)]
fn find_largest_calorie_held_by_an_elf(filename: &str) -> String {
    // let file = File::open(filename).unwrap();
    // let reader = BufReader::new(file);
    let reader = read_file(filename);

    let mut previous_elf_calories_counted: Option<u32> = None;
    let mut current_elf_calories_counter: u32 = 0;

    for line in reader.lines() {
        let line_data = line.unwrap();

        if !line_data.is_empty() {
            let calorie_entry = line_data.parse::<u32>().unwrap();
            println!("found data {}", calorie_entry);
            current_elf_calories_counter += calorie_entry;
            continue;
        }
        println!("found empty line");

        if previous_elf_calories_counted.is_some() {
            if let Some(previous_elf_calories) = previous_elf_calories_counted {
                println!(
                    "comparing prev and new: {} - {}",
                    previous_elf_calories, current_elf_calories_counter
                );
                if previous_elf_calories >= current_elf_calories_counter {
                    current_elf_calories_counter = 0;
                    continue;
                }

                previous_elf_calories_counted = Some(current_elf_calories_counter);
                current_elf_calories_counter = 0;
                continue;
            }
        }

        previous_elf_calories_counted = Some(current_elf_calories_counter);
        current_elf_calories_counter = 0;
    }

    // if last line is not empty, our custom logic does not run, so repeating here
    if let Some(previous_elf_calories) = previous_elf_calories_counted {
        println!(
            "comparing prev and new: {} - {}",
            previous_elf_calories, current_elf_calories_counter
        );
        if previous_elf_calories < current_elf_calories_counter {
            previous_elf_calories_counted = Some(current_elf_calories_counter);
        }
    }

    previous_elf_calories_counted.unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_largest_if_input_in_ascending() {
        let file_path = "./inputs/day_1/mocks/asc.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "50");
    }

    #[test]
    fn should_find_largest_if_input_in_descending() {
        let file_path = "./inputs/day_1/mocks/desc.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "100");
    }

    #[test]
    fn should_find_largest_if_input_in_ascending_with_empty_line_at_end() {
        let file_path = "./inputs/day_1/mocks/asc_end_empty_line.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "50");
    }

    #[test]
    fn should_find_largest_if_input_in_descending_with_empty_line_at_end() {
        let file_path = "./inputs/day_1/mocks/desc_end_empty_line.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "100");
    }

    #[test]
    #[ignore]
    fn the_actual_test() {
        let file_path = "./inputs/day_1/aoc_input.txt";
        let result = find_largest_calorie_held_by_an_elf(file_path);
        assert_eq!(result, "100");
    }
}
