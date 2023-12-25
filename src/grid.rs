use itertools::Itertools;
use std::{
    fmt::Display,
    ops::{Index, IndexMut, Range},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<Item> {
    pub lines: Vec<Vec<Item>>,
    pub width: usize,
    pub height: usize,
}

pub type ByteGrid = Grid<u8>;

pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn is_reverse(&self, dir: Dir) -> bool {
        match dir {
            Dir::Up => self == &Dir::Down,
            Dir::Down => self == &Dir::Up,
            Dir::Left => self == &Dir::Right,
            Dir::Right => self == &Dir::Left,
        }
    }
}

pub type Neighbor<Item> = (Dir, Option<Item>);

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

impl<Item> Grid<Item>
where
    Item: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Grid<Item> {
        Grid {
            lines: vec![vec![Item::default(); width]; height],
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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.lines.iter_mut().flat_map(|line| line.iter_mut())
    }

    pub fn coords_iter(&self) -> impl Iterator<Item = Coord> {
        (0..self.width).cartesian_product(0..self.height)
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

    pub fn get_neighbors(&self, coord: Coord) -> [(Dir, Option<&Item>); 4] {
        [
            (Dir::Up, self.get_above(coord)),
            (Dir::Down, self.get_below(coord)),
            (Dir::Left, self.get_left_of(coord)),
            (Dir::Right, self.get_right_of(coord)),
        ]
    }

    pub fn get_neighbors_coord(coord: Coord) -> [(Dir, Coord); 4] {
        [
            (Dir::Up, Self::above(coord)),
            (Dir::Down, Self::below(coord)),
            (Dir::Left, Self::left(coord)),
            (Dir::Right, Self::right(coord)),
        ]
    }

    pub fn get_in_dir(&self, coord: Coord, dir: Dir) -> Option<&Item> {
        match dir {
            Dir::Up => self.get_above(coord),
            Dir::Down => self.get_below(coord),
            Dir::Left => self.get_left_of(coord),
            Dir::Right => self.get_right_of(coord),
        }
    }

    pub fn get_above(&self, coord: Coord) -> Option<&Item> {
        self.get(Self::above(coord))
    }
    pub fn get_below(&self, coord: Coord) -> Option<&Item> {
        self.get(Self::below(coord))
    }
    pub fn get_left_of(&self, coord: Coord) -> Option<&Item> {
        self.get(Self::left(coord))
    }
    pub fn get_right_of(&self, coord: Coord) -> Option<&Item> {
        self.get(Self::right(coord))
    }

    pub fn get_cardinally_adjacent_coords(&self, coord: Coord) -> (Coord, Coord, Coord, Coord) {
        (
            Self::above(coord),
            Self::below(coord),
            Self::left(coord),
            Self::right(coord),
        )
    }

    pub fn above((x, y): Coord) -> Coord {
        (x, y.wrapping_sub(1))
    }

    pub fn below((x, y): Coord) -> Coord {
        (x, y + 1)
    }

    pub fn left((x, y): Coord) -> Coord {
        (x.wrapping_sub(1), y)
    }

    pub fn right((x, y): Coord) -> Coord {
        (x + 1, y)
    }

    pub fn get_above_coord(&self, coord: Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord(Self::above(coord))
    }
    pub fn get_below_coord(&self, coord: Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord(Self::below(coord))
    }
    pub fn get_left_coord(&self, coord: Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord(Self::left(coord))
    }
    pub fn get_right_coord(&self, coord: Coord) -> Option<(Coord, &Item)> {
        self.get_with_coord(Self::right(coord))
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

impl<Item> Display for Grid<Item>
where
    Item: Into<char> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for item in line {
                let c: char = item.clone().into();
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
