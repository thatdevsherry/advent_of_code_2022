use std::{collections::HashMap, io::BufRead, str::FromStr};

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

#[allow(dead_code)]
fn run_day_seven_part_one(filename: &str) -> u32 {
    let reader = read_file(filename);
    let mut line_iter = reader.lines();
    let first_line = line_iter.next().unwrap().unwrap();

    // first command is to cd into root.
    let first_command = Command::from_str(&first_line).unwrap();
    debug_assert!(first_command == Command::ChangeDirectory("/".to_string()));

    let mut starting_directory;
    if let Command::ChangeDirectory(ref directory_name) = first_command {
        starting_directory = Directory::new(directory_name.to_owned());
    } else {
        panic!("Umm first command isn't root?")
    }

    println!("Starting dir: {starting_directory:?}");

    let mut command_last_read = first_command;
    let mut traversal_stack: Vec<&mut Directory> = vec![&mut starting_directory];
    let mut nested_indicator = "-".to_string();

    for line in line_iter {
        let line_data = line.unwrap();
        // check if last command read was `ls`. If so, do not read the next commands
        // as actual commands, but parse them as `ls` output and check if any of them
        // start with "$", which would mean the `ls` output is complete and we have
        // received a new command.
        if !line_data.starts_with('$') && command_last_read == Command::ListContents {
            let ls_command_output_type = ListCommandOutput::from_str(&line_data).unwrap();
            match ls_command_output_type {
                ListCommandOutput::Directory(name) => {
                    let dir = Directory::new(name.clone());
                    traversal_stack
                        .last_mut()
                        .unwrap()
                        .add_content(name, DirectoryContent::Directory(dir));
                }
                ListCommandOutput::File(name, size) => {
                    let file = File::new(name.clone(), size);
                    traversal_stack
                        .last_mut()
                        .unwrap()
                        .add_content(name, DirectoryContent::File(file));
                }
            };
            continue;
        }

        let command = Command::from_str(&line_data).unwrap();
        println!("{command:?}");
        if let Command::ChangeDirectory(ref direction) = command {
            if direction == ".." {
                nested_indicator.pop();
                traversal_stack.pop();
            } else {
                nested_indicator.push('-');
                let dir = traversal_stack
                    .last_mut()
                    .unwrap()
                    .contents
                    .get_mut(direction)
                    .unwrap();
                match dir {
                    DirectoryContent::Directory(dir) => traversal_stack.push(dir),
                    _ => panic!(),
                };
            }
        }
        command_last_read = command;
    }
    println!("The whole thing");
    println!("{starting_directory:?}");

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
