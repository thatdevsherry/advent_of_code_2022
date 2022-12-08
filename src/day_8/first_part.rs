use std::io::BufRead;

use crate::read_file;

fn run_day_eight_part_one(filename: &str) {
    let reader = read_file(filename);

    for line in reader.lines() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let filename = "./inputs/day_8/mocks/example.txt";
    }
}
