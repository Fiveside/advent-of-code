use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Debug;

#[derive(Debug)]
struct ColumnView<'a, T>
where
    T: Copy + Debug,
{
    view: &'a [Vec<T>],
    colnum: usize,
    it: std::ops::Range<usize>,
}

impl<'a, T> ColumnView<'a, T>
where
    T: Copy + Debug,
{
    fn new(view: &'a [Vec<T>], colnum: usize, start: usize, end: usize) -> Self {
        Self {
            view,
            colnum,
            it: start..end,
        }
    }
}

impl<'a, T> Iterator for ColumnView<'a, T>
where
    T: Copy + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it
            .next()
            .and_then(|x| self.view[x].get(self.colnum).map(|x| *x))
    }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Vec<usize>]) -> usize {
    let mut visible_trees = (input.len() * 2) + ((input[0].len() - 2) * 2);
    for (row_index, row) in input[..input.len() - 1].iter().enumerate().skip(1) {
        for (col_index, test_tree) in row[..row.len() - 1].iter().enumerate().skip(1) {
            // search left
            if !input[row_index][..col_index]
                .iter()
                .any(|&x| x >= *test_tree)
            {
                // all shorter than our designated tree, thus this tree can be
                // seen from the outside.
                visible_trees += 1;
                continue;
            }

            // search right
            if !input[row_index][col_index + 1..]
                .iter()
                .any(|&x| x >= *test_tree)
            {
                visible_trees += 1;
                continue;
            }

            // search up
            let mut up_view = ColumnView::new(input, col_index, 0, row_index);
            if !up_view.any(|x| x >= *test_tree) {
                visible_trees += 1;
                continue;
            }

            // search down
            let mut down_view = ColumnView::new(input, col_index, row_index + 1, input.len());
            if !down_view.any(|x| x >= *test_tree) {
                visible_trees += 1;
                continue;
            }
        }
    }

    return visible_trees;
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 21)
    }
}
