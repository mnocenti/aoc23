use aoc23::{grid::Grid, *};
use itertools::Itertools;

main!(405, 400);

#[cfg(windows)]
const TWO_LINES: &str = "\r\n\r\n";
#[cfg(not(windows))]
const TWO_LINES: &str = "\n\n";

type Pattern = Grid<bool>;

type Patterns = Vec<Pattern>;

fn parse(input: &str) -> Result<Patterns> {
    Ok(input
        .split(TWO_LINES)
        .filter(|s| !s.is_empty())
        .map(|s| Pattern::from_lines_mapped(s, |byte| byte == b'#'))
        .collect_vec())
}

fn part1(patterns: &Patterns) -> Result<usize> {
    Ok(patterns.iter().map(find_reflection).sum())
}

fn part2(patterns: &Patterns) -> Result<usize> {
    Ok(patterns.iter().map(find_smudged_reflection).sum())
}

fn find_reflection(pattern: &Pattern) -> usize {
    find_vertical_reflection(pattern)
        .or_else(|| find_horizontal_reflection(pattern).map(|val| 100 * val))
        .unwrap_or(0)
}

fn find_vertical_reflection(pattern: &Pattern) -> Option<usize> {
    let w = pattern.width;
    for x in 1..w {
        let nb_columns = x.min(w - x);
        if (0..nb_columns).all(|c| {
            pattern
                .column(x - c - 1)
                .zip(pattern.column(x + c))
                .all(|(a, b)| a == b)
        }) {
            return Some(x);
        }
    }
    None
}

fn find_horizontal_reflection(pattern: &Pattern) -> Option<usize> {
    let h = pattern.height;
    for y in 1..h {
        let nb_rows = y.min(h - y);
        if (0..nb_rows).all(|r| pattern.lines[y - r - 1] == pattern.lines[y + r]) {
            return Some(y);
        }
    }
    None
}

fn find_smudged_reflection(pattern: &Pattern) -> usize {
    find_vertical_smudged_reflection(pattern)
        .or_else(|| find_horizontal_smudged_reflection(pattern).map(|val| 100 * val))
        .unwrap_or(0)
}

fn find_vertical_smudged_reflection(pattern: &Pattern) -> Option<usize> {
    let w = pattern.width;
    for x in 1..w {
        let nb_columns = x.min(w - x);
        if (0..nb_columns)
            .map(|c| {
                pattern
                    .column(x - c - 1)
                    .zip(pattern.column(x + c))
                    .map(|(a, b)| (a != b) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
            == 1
        {
            return Some(x);
        }
    }
    None
}

fn find_horizontal_smudged_reflection(pattern: &Pattern) -> Option<usize> {
    let h = pattern.height;
    for y in 1..h {
        let nb_rows = y.min(h - y);
        if (0..nb_rows)
            .map(|r| {
                pattern
                    .row(y - r - 1)
                    .zip(pattern.row(y + r))
                    .map(|(a, b)| (a != b) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
            == 1
        {
            return Some(y);
        }
    }
    None
}
