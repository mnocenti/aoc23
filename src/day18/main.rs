use std::fmt::Display;

use anyhow::anyhow;
use aoc23::{
    grid::{Coord, Grid},
    *,
};

main!(62, 0);

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    #[default]
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[apply(parse_ordered!)]
#[delim(' ')]
#[derive(Debug, Default)]
struct Instr {
    #[parse()]
    dir: Dir,
    #[parse()]
    count: usize,
    #[parse(re("\\((#[0-9a-f]+)\\)"))]
    color: Color,
}

type DigPlan = Vec<Instr>;

fn parse(input: &str) -> Result<DigPlan> {
    collect_lines(input)
}

#[derive(Debug, Default, Clone)]
struct Tile {
    dug: bool,
    color: Option<Color>,
}

fn part1(plan: &DigPlan) -> Result<usize> {
    let mut map = dig_edges(plan);
    //println!("{}", MapDisplay(&map));
    dig_inside(&mut map);
    Ok(map.iter().filter(|tile| tile.dug).count())
}

fn dig_edges(plan: &Vec<Instr>) -> Grid<Tile> {
    let (start_point, width, height) = get_bounding_box(plan);
    let mut map = Grid::new(width, height);
    dig(&mut map, start_point, None);
    let mut coord = start_point;
    for instr in plan {
        for _ in 0..instr.count {
            coord = get_next(coord, instr.dir);
            dig(&mut map, coord, Some(instr.color));
        }
    }
    map
}

fn dig_inside(map: &mut Grid<Tile>) {
    for y in 0..map.height {
        let mut inside = false;
        let mut in_wall = false;
        let mut enters_from_below = false;
        for x in 0..map.width {
            let is_dug = |x, y| map.get((x, y)).map(|t| t.dug).unwrap_or(false);
            let tile = &map[(x, y)];
            if tile.dug {
                if !in_wall {
                    in_wall = true;
                    enters_from_below = is_dug(x, y + 1);
                    if !is_dug(x + 1, y) {
                        inside = !inside;
                    }
                }
            } else {
                if in_wall {
                    let exits_from_below = is_dug(x - 1, y + 1);
                    if enters_from_below != exits_from_below {
                        inside = !inside;
                    }
                }
                in_wall = false;
                map[(x, y)].dug = inside;
            }
            //println!("{}", MapDisplay(map));
        }
    }
}

fn get_next((x, y): Coord, dir: Dir) -> Coord {
    match dir {
        Dir::Up => (x, y.wrapping_sub(1)),
        Dir::Left => (x.wrapping_sub(1), y),
        Dir::Down => (x, y + 1),
        Dir::Right => (x + 1, y),
    }
}

fn dig(map: &mut Grid<Tile>, coord: (usize, usize), color: Option<Color>) {
    let tile = &mut map[coord];
    tile.dug = true;
    tile.color = color;
}

fn get_bounding_box(plan: &[Instr]) -> (Coord, usize, usize) {
    let (_, (u, d, l, r)) = plan.iter().fold(
        ((0, 0), (0, 0, 0, 0)),
        |(mut coord, (mut u, mut d, mut l, mut r)), instr| {
            match instr.dir {
                Dir::Up => {
                    coord.1 -= instr.count as isize;
                    u = u.min(coord.1)
                }
                Dir::Down => {
                    coord.1 += instr.count as isize;
                    d = d.max(coord.1)
                }
                Dir::Left => {
                    coord.0 -= instr.count as isize;
                    l = l.min(coord.0)
                }
                Dir::Right => {
                    coord.0 += instr.count as isize;
                    r = r.max(coord.0)
                }
            }
            (coord, (u, d, l, r))
        },
    );
    let start_point = ((-l) as usize, (-u) as usize);
    (
        start_point,
        l.abs_diff(0) + r.abs_diff(0) + 1,
        u.abs_diff(0) + d.abs_diff(0) + 1,
    )
}

fn part2(_plan: &DigPlan) -> Result<usize> {
    Ok(0)
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('U') => Ok(Dir::Up),
            Some('D') => Ok(Dir::Down),
            Some('L') => Ok(Dir::Left),
            Some('R') => Ok(Dir::Right),
            _ => Err(anyhow!("failed to parse direction")),
        }
    }
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color {
            r: u8::from_str_radix(&s[1..3], 16)?,
            g: u8::from_str_radix(&s[3..5], 16)?,
            b: u8::from_str_radix(&s[5..7], 16)?,
        })
    }
}

struct MapDisplay<'a>(&'a Grid<Tile>);

impl<'a> Display for MapDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let map = self.0;
        for line in &map.lines {
            for tile in line {
                let c = if tile.dug { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
