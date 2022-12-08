use std::{
    collections::HashMap,
    fs::File as FsFile,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
    str::FromStr,
};

use crate::read_file;

#[derive(Debug, PartialEq, Eq)]
enum DirectoryContent {
    File(File),
    Directory(Directory),
}
#[derive(Debug, PartialEq, Eq)]
struct Directory {
    name: String,
    contents: HashMap<String, DirectoryContent>,
    directory_size: u32,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            contents: HashMap::new(),
            directory_size: 0,
        }
    }

    fn add_content(&mut self, name: String, content: DirectoryContent) {
        self.contents.insert(name, content);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> Self {
        File { name, size }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ChangeDirectoryArg {
    Back,
    Into,
}

impl FromStr for ChangeDirectoryArg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let false = s.is_empty() {
            return Err("No arg present for command".to_string());
        }

        match s {
            ".." => Ok(ChangeDirectoryArg::Back),
            _ => Ok(ChangeDirectoryArg::Into),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    ChangeDirectory(String),
    ListContents,
}

impl Command {
    fn get_directory_name(command: Command) -> Result<String, String> {
        match command {
            Command::ChangeDirectory(directory) => Ok(directory),
            _ => Err("Invalid command".to_string()),
        }
    }
}

impl Command {
    fn remove_shell_sign(s: &str) -> &str {
        match s.starts_with('$') {
            true => s.get(2..).unwrap(),
            false => s,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ListCommandOutput {
    File(String, u32),
    Directory(String),
}

impl FromStr for ListCommandOutput {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        debug_assert!(split.len() == 2);

        let first_text = split.first().unwrap().to_owned();
        let name = split.last().unwrap().to_owned();

        match first_text {
            "dir" => Ok(ListCommandOutput::Directory(name.to_string())),
            _ => Ok(ListCommandOutput::File(
                name.to_string(),
                u32::from_str(first_text).unwrap(),
            )),
        }
    }
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shell_sign_removed = Command::remove_shell_sign(s);
        let command = shell_sign_removed
            .get(0..2)
            .expect("should have received a valid command as this is intended for the puzzle");
        debug_assert!(command.len() == 2);
        match command {
            "cd" => Ok(Command::ChangeDirectory(
                shell_sign_removed.get(3..).unwrap().to_string(),
            )),
            "ls" => Ok(Command::ListContents),
            _ => Err("Command not recognized".to_string()),
        }
    }
}

fn handle_ls_output(line_data: String) -> (String, DirectoryContent) {
    let ls_command_output_type = ListCommandOutput::from_str(&line_data).unwrap();
    match ls_command_output_type {
        ListCommandOutput::Directory(name) => {
            let directory = Directory::new(name.clone());
            (name, DirectoryContent::Directory(directory))
        }
        ListCommandOutput::File(name, size) => {
            let file = File::new(name.clone(), size);
            (name, DirectoryContent::File(file))
        }
    }
}

/// This will take in an iter, and we will assume the first item will be the
/// command "cd {something}".
///
/// It will then assume the next command is `ls`, and it will store everything
/// in a `Directory`. If there is a nested directory, it will recursively
/// call itself, and pass along the iter recursively.
///
/// The recursive iteration will then handle the nested directory.
fn recursive_file_size_reader(line_iter: &mut Peekable<Lines<BufReader<FsFile>>>) -> Directory {
    let first_command = Command::from_str(&line_iter.next().unwrap().unwrap()).unwrap();
    println!("First command (cd): {first_command:?}");
    let mut directory = Directory::new(Command::get_directory_name(first_command).unwrap());

    let second_command = Command::from_str(&line_iter.next().unwrap().unwrap()).unwrap();
    println!("Second command (ls): {second_command:?}");

    while Command::from_str(line_iter.peek().unwrap().as_ref().unwrap()).is_err() {
        let ls_output = line_iter.next().unwrap().unwrap();
        let (name, content) = handle_ls_output(ls_output);
        directory.add_content(name, content);
    }

    // The last command is `ls`, so we also need to check if there even is a next command.
    // If not, then we are at the last recursion and have to return early, after `ls` output.
    let next_command_peek = line_iter.peek().unwrap().as_ref();
    if next_command_peek.is_err() {
        // return early
        return directory;
    }
    let next_cd_command = Command::from_str(line_iter.peek().unwrap().as_ref().unwrap()).unwrap();
    println!("Next command (peek): {next_cd_command:?}");

    match next_cd_command {
        Command::ListContents => panic!(),
        Command::ChangeDirectory(direction) => {
            if direction == ".." {
                // return from this recursive fn
                line_iter.next();
                return directory;
            } else {
                // start another recursion
                let recursive_directory = recursive_file_size_reader(line_iter);
                // line_iter.next();
                println!("Returned from {:?}", recursive_directory.name);
                let peep = line_iter.peek().unwrap().as_ref().unwrap();
                println!("Next command peek: {:?}", peep);
                let recursive_directory_name = recursive_directory.name.clone();
                directory.contents.insert(
                    recursive_directory_name,
                    DirectoryContent::Directory(recursive_directory),
                );
            }
        }
    };

    directory
}

#[allow(dead_code)]
fn run_day_seven_part_one(filename: &str) -> u32 {
    let reader = read_file(filename);
    let mut line_iter = reader.lines().peekable();

    let result = recursive_file_size_reader(&mut line_iter);
    println!("Result");
    println!("{:?}", result);
    0
}

#[cfg(test)]
mod tests {

    use super::run_day_seven_part_one;

    #[test]
    fn test_provided_example() {
        let file_path = "./inputs/day_7/mocks/example.txt";
        let result = run_day_seven_part_one(file_path);
        assert_eq!(result, 95437);
    }
}
