use itertools::Itertools;
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<Item> {
    pub lines: Vec<Vec<Item>>,
    pub width: usize,
    pub height: usize,
}

pub type ByteGrid = Grid<u8>;

pub type Coord = (usize, usize);

impl<Item> Grid<Item>
where
    Item: From<u8>,
{
    pub fn from_lines(input: &str) -> Self {
        let lines = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.as_bytes().iter().map(|c| (*c).into()).collect_vec())
            .collect_vec();
        let width = lines[0].len();
        let height = lines.len();
        Self {
            lines,
            width,
            height,
        }
    }
}

impl<Item> Grid<Item> {
    pub fn row(
        &self,
        y: usize,
    ) -> impl DoubleEndedIterator<Item = &Item> + ExactSizeIterator<Item = &Item> {
        self.lines[y].iter()
    }

    pub fn rows(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &Item>> {
        (0..self.height).map(|y| self.row(y))
    }

    pub fn column(
        &self,
        x: usize,
    ) -> impl DoubleEndedIterator<Item = &Item> + ExactSizeIterator<Item = &Item> {
        (0..self.height).map(move |y| &self.lines[y][x])
    }

    pub fn columns(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &Item>> {
        (0..self.width).map(|x| self.column(x))
    }

    pub fn from_lines_mapped(input: &str, f: impl Fn(u8) -> Item) -> Grid<Item> {
        let lines = input
            .lines()
            .map(|l| l.as_bytes().iter().copied().map(&f).collect_vec())
            .collect_vec();
        let width = lines[0].len();
        let height = lines.len();
        Grid {
            lines,
            width,
            height,
        }
    }

    pub fn mapped<MappedItem>(&self, f: impl Fn(&Item) -> MappedItem) -> Grid<MappedItem> {
        Grid::<MappedItem> {
            lines: self
                .lines
                .iter()
                .map(|l| l.iter().map(&f).collect_vec())
                .collect_vec(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn into_mapped<MappedItem>(self, f: impl Fn(Item) -> MappedItem) -> Grid<MappedItem> {
        Grid::<MappedItem> {
            lines: self
                .lines
                .into_iter()
                .map(|l| l.into_iter().map(&f).collect_vec())
                .collect_vec(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn insert_line(&mut self, y: usize, line: Vec<Item>) {
        assert!(line.len() == self.width);
        self.lines.insert(y, line);
        self.height += 1;
    }

    pub fn insert_column(&mut self, x: usize, column: Vec<Item>) {
        assert!(column.len() == self.height);
        for (y, item) in column.into_iter().enumerate() {
            self.lines[y].insert(x, item);
        }
        self.width += 1;
    }

    pub fn adjacent_to(&self, (x, y): Coord) -> impl Iterator<Item = &Item> {
        let x0 = (x as isize - 1).max(0) as usize;
        let y0 = (y as isize - 1).max(0) as usize;
        let xneighbors = x0..((x + 2).min(self.width));
        let yneighbors = y0..((y + 2).min(self.height));
        xneighbors
            .cartesian_product(yneighbors)
            .filter_map(move |(xi, yi)| (xi != x || yi != y).then_some(&self[(xi, yi)]))
    }

    pub fn adjacent_to_range(
        &self,
        xrange: &Range<usize>,
        y: usize,
    ) -> impl Iterator<Item = &Item> {
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

    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.lines.iter().flat_map(|line| line.iter())
    }

    pub fn indexed_iter(&self) -> impl DoubleEndedIterator<Item = (Coord, &Item)> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| ((x, y), c)))
    }

    /// Returns the values of the tiles in (up, down, left, right) directions
    pub fn get_cardinally_adjacent_tiles(
        &self,
        coord: Coord,
    ) -> (Option<&Item>, Option<&Item>, Option<&Item>, Option<&Item>) {
        (
            self.get_above(coord),
            self.get_below(coord),
            self.get_left_of(coord),
            self.get_right_of(coord),
        )
    }

    pub fn get_above(&self, (x, y): Coord) -> Option<&Item> {
        self.get((x, y.wrapping_sub(1)))
    }
    pub fn get_below(&self, (x, y): Coord) -> Option<&Item> {
        self.get((x, y + 1))
    }
    pub fn get_left_of(&self, (x, y): Coord) -> Option<&Item> {
        self.get((x.wrapping_sub(1), y))
    }
    pub fn get_right_of(&self, (x, y): Coord) -> Option<&Item> {
        self.get((x + 1, y))
    }

    pub fn get_above_coord(&self, (x, y): Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord((x, y.wrapping_sub(1)))
    }
    pub fn get_below_coord(&self, (x, y): Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord((x, y + 1))
    }
    pub fn get_left_coord(&self, (x, y): Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord((x.wrapping_sub(1), y))
    }
    pub fn get_right_coord(&self, (x, y): Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord((x + 1, y))
    }

    pub fn get(&self, (x, y): Coord) -> Option<&Item> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self[(x, y)])
        }
    }

    pub fn get_mut(&mut self, (x, y): Coord) -> Option<&mut Item> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self[(x, y)])
        }
    }

    pub fn get_with_coord(&self, (x, y): Coord) -> Option<(Coord, &Item)> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(((x, y), &self[(x, y)]))
        }
    }
}

impl<Item> Index<Coord> for Grid<Item> {
    type Output = Item;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.lines[y][x]
    }
}

impl<Item> IndexMut<Coord> for Grid<Item> {
    fn index_mut(&mut self, (x, y): Coord) -> &mut Self::Output {
        &mut self.lines[y][x]
    }
}
