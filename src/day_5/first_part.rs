use std::io::BufRead;

use crate::read_file;

#[allow(dead_code)]
fn run_day_five_first_part(filename: &str) {
    let reader = read_file(filename);

    for line in reader.lines() {}
}
#[cfg(test)]
mod tests {
    use super::run_day_five_first_part;

    #[test]
    fn test_example() {
        let file_path = "./inputs/day_5/mocks/example.txt";
        let result = run_day_five_first_part(file_path);
        // assert_eq!(result, "100");
    }
}
