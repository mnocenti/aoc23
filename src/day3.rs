use std::ops::Range;

use aoc23::{grid::ByteGrid, *};
use itertools::Itertools;

main!(day3, "../inputs/input3.txt");

test_with_example!(day3, "../inputs/example3.txt", 4361, 467835);

fn day3(input: &str) -> Result<(usize, usize)> {
    let grid = ByteGrid::from_lines(input);
    let numbers: Vec<_> = grid
        .lines()
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            split_numbers_indices(line).map(move |(x0, sub)| -> Result<Number> {
                let value = sub.parse()?;
                Ok(Number {
                    value,
                    y,
                    xrange: x0..(x0 + sub.len()),
                })
            })
        })
        .try_collect()?;

    let part1 = numbers
        .iter()
        .filter_map(|n| {
            grid.adjacent_to_range(&n.xrange, n.y)
                .any(|&c| is_symbol(c))
                .then_some(n.value)
        })
        .sum();

    let gears = grid
        .indexed_iter()
        .filter_map(|(coord, &c)| (c == b'*').then_some(coord));

    let part2 = gears
        .filter_map(|coord| -> Option<usize> {
            let adjacent_nums = numbers
                .iter()
                .filter_map(|n| n.is_adjacent(coord).then_some(n.value))
                .collect_vec();
            (adjacent_nums.len() == 2).then_some(adjacent_nums.iter().product())
        })
        .sum();

    Ok((part1, part2))
}

fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}

fn split_numbers_indices(s: &str) -> impl Iterator<Item = (usize, &str)> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter_map(move |sub| (!sub.is_empty()).then_some((addr_of(sub) - addr_of(s), sub)))
}

#[derive(Debug)]
struct Number {
    value: usize,
    y: usize,
    xrange: Range<usize>,
}

impl Number {
    fn is_adjacent(&self, (x, y): (usize, usize)) -> bool {
        self.y.abs_diff(y) <= 1
            && (self.xrange.contains(&(x + 1)) || (x > 0 && self.xrange.contains(&(x - 1))))
    }
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}
