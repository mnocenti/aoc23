use std::ops::Range;

use aoc23::*;
use itertools::Itertools;
use ndarray::Array2;

main!(day3, "../inputs/input3.txt");

test_with_example!(day3, "../inputs/example3.txt", 4361, 467835);

fn day3(input: &str) -> Result<(usize, usize)> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.len();
    let height = lines[0].len();
    let flat: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();
    let numbers: Vec<_> = lines
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
    let schematic = Array2::from_shape_vec((width, height), flat)?;

    let part1 = numbers
        .iter()
        .filter_map(|n| is_next_to_symbol(&schematic, &n.xrange, n.y).then_some(n.value))
        .sum();

    let gears = schematic
        .indexed_iter()
        .filter_map(|((y, x), &c)| (c == '*').then_some((x, y)));

    let part2 = gears
        .filter_map(|(x, y)| -> Option<usize> {
            let adjacent_nums = numbers
                .iter()
                .filter_map(|n| n.is_adjacent(x, y).then_some(n.value))
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
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        self.y.abs_diff(y) <= 1
            && (self.xrange.contains(&(x + 1)) || (x > 0 && self.xrange.contains(&(x - 1))))
    }
}

fn is_next_to_symbol(schematic: &Array2<char>, xrange: &Range<usize>, y: usize) -> bool {
    let x0 = (xrange.start as isize - 1).max(0) as usize;
    let y0 = (y as isize - 1).max(0) as usize;
    let xneighbors = x0..((xrange.end + 1).min(schematic.shape()[0]));
    let yneighbors = y0..((y + 2).min(schematic.shape()[1]));
    yneighbors
        .cartesian_product(xneighbors)
        .any(|coord| is_symbol(schematic[coord]))
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}
