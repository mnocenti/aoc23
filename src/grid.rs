use itertools::Itertools;
use std::ops::{Index, Range};

pub struct ByteGrid<'a> {
    lines: Vec<&'a str>,
    pub width: usize,
    pub height: usize,
}

type Coord = (usize, usize);

impl<'a> ByteGrid<'a> {
    pub fn from_lines(input: &'a str) -> ByteGrid<'a> {
        let lines: Vec<&'a str> = input.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        ByteGrid {
            lines,
            width,
            height,
        }
    }

    pub fn lines(&self) -> &Vec<&'a str> {
        &self.lines
    }

    pub fn adjacent_to(&'a self, (x, y): Coord) -> impl Iterator<Item = &'a u8> {
        let x0 = (x as isize - 1).max(0) as usize;
        let y0 = (y as isize - 1).max(0) as usize;
        let xneighbors = x0..((x + 2).min(self.width));
        let yneighbors = y0..((y + 2).min(self.height));
        xneighbors
            .cartesian_product(yneighbors)
            .filter_map(move |(xi, yi)| (xi != x || yi != y).then_some(&self[(xi, yi)]))
    }

    pub fn adjacent_to_range(
        &'a self,
        xrange: &Range<usize>,
        y: usize,
    ) -> impl Iterator<Item = &'a u8> {
        let xrange = xrange.clone();
        let x0 = (xrange.start as isize - 1).max(0) as usize;
        let y0 = (y as isize - 1).max(0) as usize;
        let xneighbors = x0..((xrange.end + 1).min(self.width));
        let yneighbors = y0..((y + 2).min(self.height));
        xneighbors
            .cartesian_product(yneighbors)
            .filter_map(move |(xi, yi)| {
                (!xrange.contains(&xi) || yi != y).then_some(&self[(xi, yi)])
            })
    }

    pub fn indexed_iter(&'a self) -> impl Iterator<Item = (Coord, &'a u8)> {
        self.lines.iter().enumerate().flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, c)| ((x, y), c))
        })
    }
}

impl<'a> Index<Coord> for ByteGrid<'a> {
    type Output = u8;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.lines()[y].as_bytes()[x]
    }
}
