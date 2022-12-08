use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct FileSystem {
    files: Vec<File>,
    root: usize,
}

enum FileType {
    // file size
    File(usize),

    // indicies into files vector representing direct descendents
    Folder(Vec<usize>),
}

struct File {
    name: String,
    typ: FileType,
}

impl File {
    fn add_child(&mut self, that: File) {
        match self.typ {
            FileType::File(_size) => panic!("tried to add a child to a file, not a folder"),
            FileType::Folder(ref mut children) => children.push(that),
        }
    }

    fn get_child_mut(&self, name: &str) -> Option<&mut File> {
        match self.typ {
            FileType::File(_size) => panic!("Tried to get child of a file, not a folder"),
            FileType::Folder(ref mut children) => {
                children.iter_mut().filter(|c| c.name == name).next()
            }
        }
    }
}

#[aoc_generator(day7)]
fn generator(input: &str) -> File {
    let mut root = File {
        name: "/".to_string(),
        typ: FileType::Folder(vec![]),
    };

    let updir = Regex::new(r#"\$ cd \.\."#).unwrap();
    let downdir = Regex::new(r#"\$ cd (\w+)"#).unwrap();
    let rootnav = Regex::new(r#"\$ cd \\"#).unwrap();
    let ls = Regex::new(r#"\$ ls"#).unwrap();
    let file_entry = Regex::new(r#"(\d+) (\w+)"#).unwrap();
    let filder_entry = Regex::new(r#"dir (\w+)"#).unwrap();

    let mut dirstack = vec![&mut root];

    for line in input.lines() {
        if rootnav.is_match(line) {
            dirstack.truncate(1);
        } else if updir.is_match(line) {
            dirstack.pop().expect("directory stack is empty!");
        } else if let Some(caps) = downdir.captures(line) {
            let folder = dirstack
                .last()
                .unwrap()
                .get_child_mut(caps.get(1).unwrap().as_str())
                .unwrap();
            dirstack.push(folder);
        }
    }

    return root;
}

#[aoc(day7, part1)]
fn part1(input: &File) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "$ cd /
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
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 95437);
    }
}
