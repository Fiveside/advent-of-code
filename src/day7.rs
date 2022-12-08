use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Inode = usize;

struct FileSystem {
    files: Vec<File>,
}

impl FileSystem {
    fn new() -> Self {
        let root = File {
            name: "/".to_string(),
            typ: FileType::Folder(vec![]),
        };
        Self { files: vec![root] }
    }

    fn root_inode(&self) -> Inode {
        return 0 as Inode;
    }

    fn add_file(&mut self, parent: Inode, name: &str, size: usize) -> Inode {
        let child = File {
            name: name.to_string(),
            typ: FileType::File(size),
        };
        return self.add(parent, child);
    }

    fn add_folder(&mut self, parent: Inode, name: &str) -> Inode {
        let child = File {
            name: name.to_string(),
            typ: FileType::Folder(vec![]),
        };
        return self.add(parent, child);
    }

    fn add(&mut self, parent: Inode, child: File) -> Inode {
        self.files.push(child);
        let child_inode = self.files.len() - 1 as Inode;

        let p = self.files.get_mut(parent as usize).unwrap();
        match p.typ {
            FileType::File(_) => panic!("Attempted to add a child to a regular file"),
            FileType::Folder(ref mut children) => children.push(child_inode),
        }

        return child_inode;
    }

    fn get(&self, that: Inode) -> &File {
        self.files.get(that).unwrap()
    }
}

#[derive(Debug)]
enum FileType {
    // file size
    File(usize),

    // indicies into files vector representing direct descendents
    Folder(Vec<Inode>),
}

#[derive(Debug)]
struct File {
    name: String,
    typ: FileType,
}

impl File {
    fn get_child_inode(&self, fs: &FileSystem, name: &str) -> Inode {
        if let FileType::Folder(ref children) = self.typ {
            for child_inode in children {
                let child = fs.get(*child_inode);
                if child.name == name {
                    return *child_inode;
                }
            }
            panic!("no children match that name!");
        } else {
            panic!("Attempted to find children for a file, not a folder");
        }
    }

    fn get_size(&self, fs: &FileSystem) -> usize {
        match self.typ {
            FileType::File(x) => x,
            FileType::Folder(ref children) => children
                .iter()
                .map(|x| fs.get(*x).get_size(fs))
                .fold(0, |x, y| x + y),
        }
    }

    fn is_folder(&self) -> bool {
        match self.typ {
            FileType::Folder(_) => true,
            _ => false,
        }
    }
}

#[aoc_generator(day7)]
fn generator(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let root_inode = fs.root_inode();

    let updir = Regex::new(r#"\$ cd \.\."#).unwrap();
    let downdir = Regex::new(r#"\$ cd (\w+)"#).unwrap();
    let rootnav = Regex::new(r#"\$ cd /"#).unwrap();
    let ls = Regex::new(r#"\$ ls"#).unwrap();
    let file_entry = Regex::new(r#"(\d+) (\w+)"#).unwrap();
    let folder_entry = Regex::new(r#"dir (\w+)"#).unwrap();

    let mut dirstack = vec![root_inode];

    for line in input.lines() {
        if rootnav.is_match(line) {
            dirstack.truncate(1);
        } else if updir.is_match(line) {
            dirstack.pop().expect("directory stack is empty!");
        } else if let Some(caps) = downdir.captures(line) {
            let folder_name = caps.get(1).unwrap().as_str();
            let current_folder = fs.get(*dirstack.last().unwrap());
            let next_folder = current_folder.get_child_inode(&fs, folder_name);
            dirstack.push(next_folder);
        } else if ls.is_match(line) {
            // Nothing to do here
        } else if let Some(caps) = file_entry.captures(line) {
            let file_size = caps.get(1).unwrap().as_str().parse().unwrap();
            let file_name = caps.get(2).unwrap().as_str();
            let parent_inode = *dirstack.last().unwrap();

            fs.add_file(parent_inode, file_name, file_size);
        } else if let Some(caps) = folder_entry.captures(line) {
            let file_name = caps.get(1).unwrap().as_str();
            let parent_inode = *dirstack.last().unwrap();
            fs.add_folder(parent_inode, file_name);
        } else {
            panic!("Unrecognized statement: {}", line);
        }
    }

    return fs;
}

#[aoc(day7, part1)]
fn part1(fs: &FileSystem) -> usize {
    let folders: Vec<&File> = fs.files.iter().filter(|x| x.is_folder()).collect();
    folders
        .iter()
        .map(|f| f.get_size(fs))
        .filter(|&size| size <= 100_000)
        .fold(0, |x, y| x + y)
}

#[aoc(day7, part2)]
fn part2(fs: &FileSystem) -> usize {
    let total_space = 70_000_000;
    let used_space = fs.files.get(0).unwrap().get_size(fs);
    let free_space = total_space - used_space;
    let to_free = 30_000_000 - free_space;

    let folders: Vec<_> = fs.files.iter().filter(|f| f.is_folder()).collect();
    folders
        .into_iter()
        .map(|f| f.get_size(fs))
        .filter(|&size| size >= to_free)
        .fold(usize::MAX, std::cmp::min)
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

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 24933642);
    }
}
