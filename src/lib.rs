use std::{fs::File, io::BufReader};

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

pub fn read_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}
