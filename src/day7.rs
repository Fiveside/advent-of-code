use aoc_runner_derive::{aoc, aoc_generator};

struct Day7;

#[aoc_generator(day7)]
fn generator(input: &str) -> Day7 {
    unimplemented!()
}

#[aoc(day7, part1)]
fn part1(input: &Day7) -> usize {
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
