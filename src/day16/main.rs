use std::fmt::Display;

use anyhow::anyhow;
use aoc23::{
    grid::{Coord, Grid},
    *,
};
use bitflags::bitflags;
use colored::Colorize;

main!(46, 51);

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    struct BeamDir: u8 {
        const None  = 0b0000;
        const Up    = 0b0001;
        const Left  = 0b0010;
        const Down  = 0b0100;
        const Right = 0b1000;
    }
}

#[derive(Debug, Clone)]
struct Tile {
    tile: u8,
    beam: BeamDir,
}

type Cave = Grid<Tile>;

fn parse(input: &str) -> Result<Cave> {
    Ok(Cave::from_lines(input))
}

fn part1(cave: &Cave) -> Result<usize> {
    let mut cave = cave.clone();

    add_beam(&mut cave, (0, 0), BeamDir::Right);

    Ok(energized_tiles(&mut cave))
}

fn part2(cave: &Cave) -> Result<usize> {
    let all_starting_points = ((0..cave.width).map(|x| ((x, 0), BeamDir::Down)))
        .chain((0..cave.height).map(|y| ((0, y), BeamDir::Right)))
        .chain((0..cave.width).map(|x| ((x, cave.height - 1), BeamDir::Up)))
        .chain((0..cave.height).map(|y| ((cave.width - 1, y), BeamDir::Left)));

    all_starting_points
        .map(|(coord, dir)| {
            let mut cave = cave.clone();
            add_beam(&mut cave, coord, dir);
            energized_tiles(&mut cave)
        })
        .max()
        .ok_or(anyhow!("oopsie"))
}

fn energized_tiles(cave: &mut Grid<Tile>) -> usize {
    cave.iter()
        .filter(|tile| tile.beam != BeamDir::None)
        .count()
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        Tile {
            tile: value,
            beam: BeamDir::None,
        }
    }
}

fn add_beam(cave: &mut Grid<Tile>, coord: Coord, dir: BeamDir) {
    let tile = cave.get_mut(coord);
    if tile.is_none() {
        // out of bounds
        return;
    }
    let tile = tile.unwrap();
    if tile.beam.contains(dir) {
        // already processed this beam
        return;
    }
    tile.beam |= dir;
    match (tile.tile, dir) {
        (b'.', _)
        | (b'|', BeamDir::Up)
        | (b'|', BeamDir::Down)
        | (b'-', BeamDir::Left)
        | (b'-', BeamDir::Right) => add_beam_from(cave, coord, dir),
        (b'|', _) => {
            add_beam_from(cave, coord, BeamDir::Up);
            add_beam_from(cave, coord, BeamDir::Down)
        }
        (b'-', _) => {
            add_beam_from(cave, coord, BeamDir::Left);
            add_beam_from(cave, coord, BeamDir::Right)
        }
        (mirror, _) => add_beam_from(cave, coord, reflect(dir, mirror)),
    }
}

fn flip(dir: BeamDir) -> BeamDir {
    match dir {
        BeamDir::Up => BeamDir::Down,
        BeamDir::Left => BeamDir::Right,
        BeamDir::Down => BeamDir::Up,
        BeamDir::Right => BeamDir::Left,
        _ => panic!("flip requires a single direction, got {dir:?}"),
    }
}

fn add_beam_from(cave: &mut Grid<Tile>, coord: Coord, dir: BeamDir) {
    add_beam(cave, get_next(coord, dir), dir)
}

fn reflect(dir: BeamDir, mirror: u8) -> BeamDir {
    let dir = if mirror == b'\\' { flip(dir) } else { dir };
    match dir {
        BeamDir::Up => BeamDir::Right,
        BeamDir::Left => BeamDir::Down,
        BeamDir::Down => BeamDir::Left,
        BeamDir::Right => BeamDir::Up,
        _ => panic!("reflect requires a single direction, got {dir:?}"),
    }
}

fn get_next((x, y): Coord, dir: BeamDir) -> Coord {
    match dir {
        BeamDir::Up => (x, y.wrapping_sub(1)),
        BeamDir::Left => (x.wrapping_sub(1), y),
        BeamDir::Down => (x, y + 1),
        BeamDir::Right => (x + 1, y),
        _ => panic!("get_next requires a single direction, got {dir:?}"),
    }
}

struct CaveDisplay<'a>(&'a Cave);

impl<'a> Display for CaveDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let cave = self.0;
        for line in &cave.lines {
            for tile in line {
                let display = match tile.beam {
                    BeamDir::None => String::from(tile.tile as char).bright_black(),
                    _ => String::from(tile.tile as char).yellow(),
                };
                write!(f, "{}", display)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
