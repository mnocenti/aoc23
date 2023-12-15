use std::{collections::HashMap, fmt::Display};

use aoc23::{
    grid::{Coord, Grid},
    *,
};
use colored::Colorize;

main!(136, 64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Empty,
    Square,
    Round,
}

type Rocks = Grid<Rock>;

fn parse(input: &str) -> Result<Rocks> {
    Ok(Rocks::from_lines(input))
}

fn part1(rocks: &Rocks) -> Result<usize> {
    let mut rocks = rocks.clone();
    tilt(&mut rocks, Dir::North);
    Ok(rocks
        .indexed_iter()
        .map(|(coord, _)| north_load(&rocks, coord))
        .sum())
}

fn part2(rocks: &Rocks) -> Result<usize> {
    let mut rocks = rocks.clone();
    let mut history = HashMap::new();

    let mut n = 0;
    while history.entry(uuid(&rocks)).or_insert(n) == &n {
        spin_cycle(&mut rocks);
        n += 1;
    }

    let loop_start = history.get(&uuid(&rocks)).unwrap();
    let loop_end = n;
    println!("Found loop from {loop_start} to {loop_end}");

    let pos_in_loop = (1000000000 - loop_start) % (loop_end - loop_start);

    for _ in 0..pos_in_loop {
        spin_cycle(&mut rocks);
    }

    Ok(rocks
        .indexed_iter()
        .map(|(coord, _)| north_load(&rocks, coord))
        .sum())
}

impl From<u8> for Rock {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Rock::Empty,
            b'#' => Rock::Square,
            b'O' => Rock::Round,
            _ => panic!("expected rock shape"),
        }
    }
}

fn north_load(rocks: &Rocks, (x, y): Coord) -> usize {
    match rocks[(x, y)] {
        Rock::Round => rocks.height - y,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    West,
    South,
    East,
}

fn spin_cycle(rocks: &mut Rocks) {
    tilt(rocks, Dir::North);
    tilt(rocks, Dir::West);
    tilt(rocks, Dir::South);
    tilt(rocks, Dir::East);
}

fn tilt(rocks: &mut Rocks, dir: Dir) {
    let round_rock_coords: Vec<_> = match dir {
        Dir::North | Dir::West => rocks
            .indexed_iter()
            .filter_map(|(coord, rock)| (*rock == Rock::Round).then_some(coord))
            .collect(),
        Dir::South | Dir::East => rocks
            .indexed_iter()
            .rev()
            .filter_map(|(coord, rock)| (*rock == Rock::Round).then_some(coord))
            .collect(),
    };
    for coord in round_rock_coords {
        let dest = roll(rocks, coord, dir);
        rocks[coord] = Rock::Empty;
        rocks[dest] = Rock::Round;
    }
}

fn roll(rocks: &Rocks, coord: Coord, dir: Dir) -> Coord {
    let mut coord = (coord.0 as isize, coord.1 as isize);
    let incr = match dir {
        Dir::North => (0, -1),
        Dir::West => (-1, 0),
        Dir::South => (0, 1),
        Dir::East => (1, 0),
    };
    let mut nb_round_rocks = 0;
    coord = add(coord, incr);
    let bounds_x = 0..(rocks.width as isize);
    let bounds_y = 0..(rocks.height as isize);
    while bounds_x.contains(&coord.0) && bounds_y.contains(&coord.1) {
        match rocks[(coord.0 as usize, coord.1 as usize)] {
            Rock::Square => break,
            Rock::Round => nb_round_rocks += 1,
            Rock::Empty => (),
        }
        coord = add(coord, incr);
    }
    (
        (coord.0 - incr.0 * (nb_round_rocks + 1)) as usize,
        (coord.1 - incr.1 * (nb_round_rocks + 1)) as usize,
    )
}

fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn uuid(rocks: &Rocks) -> Vec<usize> {
    rocks
        .iter()
        .fold((Vec::new(), 0), |(mut acc, n), rock| {
            if n % 30 == 0 {
                acc.push(0);
            }
            let val = acc.last_mut().unwrap();
            let rock_val = match *rock {
                Rock::Empty => 0,
                Rock::Square => 1,
                Rock::Round => 2,
            };
            *val = *val * 3 + rock_val;
            (acc, n + 1)
        })
        .0
}

struct RockDisplay<'a>(&'a Rocks);

impl<'a> Display for RockDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let rocks = self.0;
        for line in &rocks.lines {
            for rock in line {
                let display = match *rock {
                    Rock::Empty => ".".color(colored::Color::BrightBlack),
                    Rock::Round => "O".yellow(),
                    Rock::Square => "#".blue(),
                };
                write!(f, "{}", display)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
