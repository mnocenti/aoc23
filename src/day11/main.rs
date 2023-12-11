use aoc23::{
    grid::{Coord, Grid},
    *,
};
use itertools::Itertools;

main!(374, 82000210);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Tile {
    Empty,
    Galaxy,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Empty,
            b'#' => Tile::Galaxy,
            _ => panic!("bad character"),
        }
    }
}

type Space = Grid<Tile>;

fn parse(input: &str) -> Result<Space> {
    Ok(Space::from_lines_mapped(input, |b| b.into()))
}

fn part1(space: &Space) -> Result<usize> {
    cosmic_expansion(space, 2)
}

fn part2(space: &Space) -> Result<usize> {
    cosmic_expansion(space, 1000000)
}

fn cosmic_expansion(space: &Space, expansion_factor: usize) -> Result<usize> {
    let galaxies = space
        .indexed_iter()
        .filter_map(|(coord, tile)| (*tile == Tile::Galaxy).then_some(coord))
        .collect_vec();

    let expanded_space = get_expanded_space(space);

    Ok(galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .filter(|(coord1, coord2)| coord1 < coord2)
        .map(|(g0, g1)| expanded_distance(g0, g1, expansion_factor, &expanded_space))
        .sum())
}

fn get_expanded_space(space: &Grid<Tile>) -> (Vec<usize>, Vec<usize>) {
    let expanded_rows = space
        .lines
        .iter()
        .enumerate()
        .filter_map(|(y, l)| l.iter().all(|tile| *tile == Tile::Empty).then_some(y))
        .collect_vec();
    let expanded_columns = space
        .columns()
        .enumerate()
        .filter_map(|(x, mut c)| c.all(|tile| *tile == Tile::Empty).then_some(x))
        .collect_vec();
    (expanded_rows, expanded_columns)
}

fn expanded_distance(
    (x0, y0): &Coord,
    (x1, y1): &Coord,
    expansion_factor: usize,
    (expanded_rows, expanded_columns): &(Vec<usize>, Vec<usize>),
) -> usize {
    let col_range = ((*x0).min(*x1))..((*x0).max(*x1));
    let row_range = ((*y0).min(*y1))..((*y0).max(*y1));
    let num_expanded_rows = expanded_rows
        .iter()
        .filter(|x| row_range.contains(x))
        .count();
    let num_expanded_cols = expanded_columns
        .iter()
        .filter(|y| col_range.contains(y))
        .count();
    x0.abs_diff(*x1)
        + y0.abs_diff(*y1)
        + (num_expanded_rows + num_expanded_cols) * (expansion_factor - 1)
}
