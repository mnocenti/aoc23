use std::ops::Range;

use aoc23::{grid::ByteGrid, *};
use itertools::Itertools;

main!(4361, 467835);

type Input = (ByteGrid, Vec<Number>);

fn parse(input: &str) -> Result<Input> {
    let grid = ByteGrid::from_lines(input);
    let numbers = get_numbers(&grid)?;
    Ok((grid, numbers))
}

fn part1((grid, numbers): &Input) -> Result<usize> {
    Ok(numbers
        .iter()
        .filter_map(|n| {
            grid.adjacent_to_range(&n.xrange, n.y)
                .any(|&c| is_symbol(c))
                .then_some(n.value)
        })
        .sum())
}

fn part2((grid, numbers): &Input) -> Result<usize> {
    let gears = grid
        .indexed_iter()
        .filter_map(|(coord, &c)| (c == b'*').then_some(coord));

    Ok(gears
        .filter_map(|coord| -> Option<usize> {
            let adjacent_nums = numbers
                .iter()
                .filter_map(|n| n.is_adjacent(coord).then_some(n.value))
                .collect_vec();
            (adjacent_nums.len() == 2).then_some(adjacent_nums.iter().product())
        })
        .sum())
}

fn get_numbers(grid: &ByteGrid) -> Result<Vec<Number>> {
    grid.lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            split_numbers_indices(line)
                .map(move |(x0, sub)| -> Result<Number> { Number::parse(sub, y, x0) })
        })
        .try_collect()
}

fn addr_of(s: &[u8]) -> usize {
    s.as_ptr() as usize
}

fn split_numbers_indices(s: &[u8]) -> impl Iterator<Item = (usize, &[u8])> {
    s.split(|c: &u8| !c.is_ascii_digit())
        .filter_map(move |sub| (!sub.is_empty()).then_some((addr_of(sub) - addr_of(s), sub)))
}

#[derive(Debug)]
struct Number {
    value: usize,
    y: usize,
    xrange: Range<usize>,
}

impl Number {
    fn parse(sub: &[u8], y: usize, x0: usize) -> Result<Number> {
        let value = std::str::from_utf8(sub)?.parse()?;
        Ok(Number {
            value,
            y,
            xrange: x0..(x0 + sub.len()),
        })
    }
    fn is_adjacent(&self, (x, y): (usize, usize)) -> bool {
        self.y.abs_diff(y) <= 1
            && (self.xrange.contains(&(x + 1)) || (x > 0 && self.xrange.contains(&(x - 1))))
    }
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}
