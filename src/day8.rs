use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
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

impl<'a, T> DoubleEndedIterator for ColumnView<'a, T>
where
    T: Copy + Debug,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it
            .next_back()
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

fn visibility_predicate(
    mut it: impl Iterator<Item = usize>,
    height: usize,
    default: usize,
) -> usize {
    it.find_position(|&s| s >= height)
        .map(|(pos, _val)| pos + 1)
        .unwrap_or(default)
}

fn visibility_score(input: &[Vec<usize>], row: usize, col: usize) -> usize {
    let height = input[row][col];
    let left_slice = &input[row][0..col];
    let left = visibility_predicate(left_slice.iter().copied().rev(), height, left_slice.len());

    let right_slice = &input[row][col + 1..];
    let right = visibility_predicate(right_slice.iter().copied(), height, right_slice.len());

    let up_it = ColumnView::new(input, col, 0, row).rev();
    let up = visibility_predicate(up_it, height, row);

    let down_it = ColumnView::new(input, col, row + 1, input.len());
    let down = visibility_predicate(down_it, height, input.len() - (row + 1));

    left * right * up * down
}

#[aoc(day8, part2)]
fn part2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            (0..row.len()).map(move |col_idx| visibility_score(input, row_idx, col_idx))
        })
        .max()
        .unwrap()
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

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 8);
    }
}
