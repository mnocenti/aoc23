use std::{collections::HashMap, fmt::Display, mem::swap};

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
    let mut tilted_north = rocks.clone();
    tilt(rocks, &mut tilted_north, Dir::North);
    Ok(tilted_north
        .indexed_iter()
        .map(|(coord, _)| north_load(&tilted_north, coord))
        .sum())
}

fn part2(rocks: &Rocks) -> Result<usize> {
    let mut rocks = rocks.clone();
    let mut previous = rocks.clone();

    let mut history = HashMap::new();

    let mut n = 0;
    while !history.contains_key(&rocks) {
        history.insert(rocks.clone(), n);
        swap(&mut previous, &mut rocks);
        spin_cycle(&mut previous, &mut rocks);
        n += 1;
    }

    let loop_start = history.get(&rocks).unwrap();
    let loop_end = n;
    println!("Found loop from {loop_start} to {loop_end}");

    let pos_in_loop = (1000000000 - loop_start) % (loop_end - loop_start);

    for _ in 0..pos_in_loop {
        swap(&mut previous, &mut rocks);
        spin_cycle(&mut previous, &mut rocks);
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

fn spin_cycle(from: &mut Rocks, to: &mut Rocks) {
    tilt(from, to, Dir::North);
    tilt(to, from, Dir::West);
    tilt(from, to, Dir::South);
    tilt(to, from, Dir::East);
    swap(from, to);
}

fn tilt(from: &Rocks, to: &mut Rocks, dir: Dir) {
    remove_round_rocks(to);
    let round_rock_coords = from
        .indexed_iter()
        .filter_map(|(coord, rock)| (*rock == Rock::Round).then_some(coord));
    for coord in round_rock_coords {
        let dest = roll(from, coord, dir);
        to[dest] = Rock::Round;
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

fn remove_round_rocks(rocks: &mut Rocks) {
    for l in rocks.lines.iter_mut() {
        for rock in l.iter_mut() {
            if *rock == Rock::Round {
                *rock = Rock::Empty;
            }
        }
    }
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
