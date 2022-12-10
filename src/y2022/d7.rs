//! --- Day 7: No Space Left On Device ---
/*
/// File System Entry
#[derive(Debug, PartialEq, Eq)]
enum FsEntry<'a> {
    File(File<'a>),
    Directory(Directory<'a>),
}
#[derive(Debug, PartialEq, Eq)]
struct File<'a> {
    name: &'a str,
    size: usize,
}
#[derive(Debug, PartialEq, Eq)]
struct Directory<'a> {
    name: &'a str,
    size: usize,
    entries: Option<Vec<FsEntry<'a>>>,
}

fn parse_file_entry(entry: &str) -> File {
    let mut parts = entry.split_whitespace();
    let size = parts.next().unwrap().parse::<usize>().unwrap();
    let name = parts.next().unwrap();
    File { name, size }
}

fn parse_directory_entry(entry: &str) -> Directory {
    let mut parts = entry.split_whitespace();
    let name = parts.nth(1).unwrap();
    Directory {
        name,
        size: 0,
        entries: None,
    }
}

fn parse_directory_content(content: &str) -> Vec<FsEntry> {
    content
        .lines()
        .map(|line| {
            if line.starts_with("dir") {
                FsEntry::Directory(parse_directory_entry(line))
            } else {
                FsEntry::File(parse_file_entry(line))
            }
        })
        .collect()
}

pub fn part1(input: String) -> String {
    // split into each command and its output
    let coms = input.split("$ ").collect::<Vec<&str>>();
    // initialize the root directory
    let mut root_dir = Directory {
        name: "/",
        size: 0,
        entries: None,
    };
    let mut current_dir_path: Vec<&mut Directory> = vec![&mut root_dir];
    for com in coms {
        // if the command is a cd command
        if com.starts_with("cd") {
            // split the command into its parts
            let mut parts = com.split_whitespace();
            let dir_name = parts.nth(1).unwrap();
            // if it is root
            if dir_name == "/" {
                // drain all but the root directory
                current_dir_path.drain(1..);
            // if it is '..' (go up one level)
            } else if dir_name == ".." {
                // go up one level by popping the last directory from the path
                current_dir_path.pop();
            // else it must be a directory name, find the directory and push it to the path
            } else {
                // get the current directory and its entries
                // find the directory
                let dir = if let Some(entries) = &mut current_dir_path.last_mut().unwrap().entries {
                    entries
                } else {
                    panic!("Current directory has no entries"); // TODO: assume that the input is valid and add the input name to entries
                }
                .iter_mut()
                .find_map(|entry| match entry {
                    // if it is a directory and its name matches the directory name
                    FsEntry::Directory(dir) => if dir.name == dir_name {Some(dir)} else {None},
                    _ => None,
                })
                .expect("Directory not found");

                // push it to the path
                current_dir_path.push(dir);
            }
        // if the command is a ls command
        } else if com.starts_with("ls") {
            // parse the directory content and add it to the current directory
            let content = com.split_at(3).1;
            let current_dir = current_dir_path.last_mut().unwrap();
            current_dir.entries = Some(parse_directory_content(content));
        } else {
            panic!("Unknown command: {}", com);
        }
    }
}
*/

fn parse_input(input: String) -> Vec<u32> {
    //creates a new, empty vector of unsigned 32-bit integers called tmp_dir.
    let mut active_file_path: Vec<u32> = Vec::new();
    //creates another new, empty vector of unsigned 32-bit integers called directories.
    let mut directories: Vec<u32> = Vec::new();
    //begins a loop that iterates over each line in the input string.
    for line in input.lines() {
        //performs pattern matching on the line. This line does a few things:
        // line.split(' ') splits the line on spaces, returning a Vec of substrings.
        // .collect::<Vec<&str>>() collects the substrings into a new Vec.
        // .as_slice() converts the Vec into a slice, which is needed for the pattern matching in the match expression.
        println!("line: {:?}", line);
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            // checks if the line matches the pattern ["$", "cd", ".."] (which represents the command to move to the parent directory).
            // If the line matches this pattern, then directories.push(tmp_dir.pop().unwrap()) pops the last element from tmp_dir (the current directory) and pushes it onto directories (a list of previously visited directories).
            ["$", "cd", ".."] => {
                let tmp = active_file_path.pop().unwrap();
                println!("  pushing {:?} from active_file_path to directories", tmp);
                directories.push(tmp)
            }
            // if the line matches the pattern ["$", "cd", _] (which represents the command to move to a new directory). If the line matches this pattern, then tmp_dir.push(0) pushes a 0 onto tmp_dir (to represent the new directory that was entered).
            ["$", "cd", name] => {
                println!(
                    "  pushing 0 to active_file_path as new directory {:?}",
                    name
                );
                active_file_path.push(0)
            }
            // if the line matches the pattern [size, _] (where size is any string). If the line matches this pattern, then This code block first tries to parse the size string as an unsigned 32-bit integer using size.parse::<u32>(). If the parse is successful, then the code block tmp_dir.iter_mut().for_each(|n| *n += num) iterates over all elements in tmp_dir, adding the parsed value num to each element in tmp_dir. This effectively increases the size of the current directory by the value of num.
            [size, name] => {
                // Getting rid of "dir ..." and "$ ls" here
                println!(
                    "  trying to parse file {:?} as size with filename: {:?}",
                    size, name
                );
                if let Ok(num) = size.parse::<u32>() {
                    println!(
                        "      success! adding {:?} to all elements in active_file_path",
                        num
                    );
                    active_file_path.iter_mut().for_each(|n| *n += num)
                } else {
                    println!("      failed.");
                }
            }
            [..] => unreachable!(),
        }
    }
    directories.extend(active_file_path);
    directories
}

pub fn part1(input: String) -> String {
    parse_input(input)
        .iter()
        .filter(|&&size| size < 100_000)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let directories = parse_input(input);
    // Get the largest directory, which is the root
    let root = directories.iter().max().unwrap();
    // Get the smallest directory that is larger than the root + 30_000_000 - 70_000_000
    let required = root + 30_000_000 - 70_000_000;
    directories
        .iter()
        .filter(|&&dir| dir >= required)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "95437".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "24933642".to_string());
    }
}
