//! --- Day 7: No Space Left On Device ---
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
    Directory { name, entries: None }
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
    let coms = input.split("$ ").collect::<Vec<&str>>();
    let mut root = Directory { name: "/", entries: None };
    let mut current_dir_path: Vec<&mut Directory> = vec![&mut root];
    for com in coms {
        if com.starts_with("cd") {
            let mut parts = com.split_whitespace();
            let dir_name = parts.nth(1).unwrap();
            if dir_name == "/" { // return to root
                current_dir_path.drain(1..);
            } else if dir_name == ".." { // go up one level
                current_dir_path.pop();
            } else { // go down one level, find the directory and push it to the path
                let mut current_dir = current_dir_path.last_mut().unwrap();
                let entries = current_dir.entries.as_mut().unwrap();
                let dir = entries
                    .iter_mut()
                    .find(|entry| match entry {
                        FsEntry::Directory(dir) => dir.name == dir_name,
                        _ => false,
                    })
                    .unwrap();
                let dir = match dir {
                    FsEntry::Directory(dir) => dir,
                    _ => panic!("Cannot cd into a file"),
                };
                let dir = current_dir_path.drain(..).find(|x| *x == dir).unwrap();
                current_dir_path.push(dir);
            }
        } else if com.starts_with("ls") {
            let content = com.split_at(3).1;
            let current_dir = current_dir_path.last_mut().unwrap();
            current_dir.entries = Some(parse_directory_content(content));
        }
        else {
            panic!("Unknown command: {}", com);
        }
    }
    unimplemented!()
}

pub fn part2(input: String) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = 
"$ cd /
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
        assert_eq!(part2(INPUT.to_string()), "".to_string());
        unimplemented!();
    }
}
