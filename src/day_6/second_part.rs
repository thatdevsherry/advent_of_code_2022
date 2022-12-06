use std::io::BufRead;

use crate::read_file;

use super::first_part::has_duplicates;

#[allow(dead_code)]
fn run_day_six_part_two(filename: &str) -> u32 {
    let reader = read_file(filename);

    let mut result = 0;
    for line in reader.lines() {
        let data_stream = line.unwrap();
        result = parse_start_of_message(data_stream);
    }
    result
}

fn parse_start_of_message(data_stream: String) -> u32 {
    let mut buf: Vec<char> = Vec::new();

    // start reading data stream
    for (idx, i) in data_stream.chars().enumerate() {
        buf.push(i);

        if buf.len() > 13 {
            let potential_start_of_packet = buf.get(idx - 13..idx + 1).unwrap();
            match has_duplicates(potential_start_of_packet) {
                true => continue,
                false => {
                    println!("THIS IS IT: {:?}", potential_start_of_packet);
                    break;
                }
            }
        }
    }
    u32::try_from(buf.len()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::run_day_six_part_two;

    #[test]
    fn test_example_one() {
        let filename = "./inputs/day_6/mocks/example_1.txt";
        let result = run_day_six_part_two(filename);
        assert_eq!(result, 19);
    }

    #[test]
    fn test_example_two() {
        let filename = "./inputs/day_6/mocks/example_2.txt";
        let result = run_day_six_part_two(filename);
        assert_eq!(result, 23);
    }

    #[test]
    fn test_example_three() {
        let filename = "./inputs/day_6/mocks/example_3.txt";
        let result = run_day_six_part_two(filename);
        assert_eq!(result, 23);
    }

    #[test]
    fn test_example_four() {
        let filename = "./inputs/day_6/mocks/example_4.txt";
        let result = run_day_six_part_two(filename);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_example_five() {
        let filename = "./inputs/day_6/mocks/example_5.txt";
        let result = run_day_six_part_two(filename);
        assert_eq!(result, 26);
    }

    #[test]
    #[ignore]
    fn actual_test() {
        let filename = "./inputs/day_6/aoc_input.txt";
        let result = run_day_six_part_two(filename);
        assert_eq!(result, 0);
    }
}
