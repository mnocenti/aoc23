use std::fmt::Display;

use anyhow::anyhow;
use aoc23::grid::{ByteGrid, Coord, Grid};
use aoc23::*;
use colored::{ColoredString, Colorize};
use itertools::Itertools;

main!("example1.txt", 8, "example2.txt", 4);

type Input = ByteGrid;

fn parse(input: &str) -> Result<Input> {
    Ok(ByteGrid::from_lines(input))
}

fn part1(maze: &Input) -> Result<usize> {
    let start_pos = maze
        .indexed_iter()
        .find(|(_, &tile)| tile == b'S')
        .ok_or(anyhow!("no start pos"))?
        .0;
    let start_pipe = determine_starting_pipe_shape(maze, start_pos);
    let mut maze = (*maze).clone();
    maze[start_pos] = start_pipe;
    let mut current_pipes = vec![start_pos; 1];
    let mut steps = 0;
    while !current_pipes.is_empty() {
        let next_pipes = current_pipes
            .iter()
            .flat_map(|coord| {
                get_neighboring_pipes(*coord, &maze, |tile| *tile, |tile| tile != b'.')
            })
            .collect_vec();
        current_pipes
            .into_iter()
            .for_each(|coord| maze[coord] = b'.');
        current_pipes = next_pipes;
        steps += 1;
    }

    Ok(steps - 1)
}

fn get_neighboring_pipes<Item: Clone>(
    coord: (usize, usize),
    maze: &Grid<Item>,
    get_tile: impl Fn(&Item) -> u8,
    predicate: impl Fn(Item) -> bool,
) -> impl Iterator<Item = Coord> {
    let copied = |opt: Option<(Coord, &Item)>| opt.map(|(coord, val)| (coord, (*val).clone()));
    let candidates = match get_tile(&maze[coord]) {
        b'|' => [
            copied(maze.get_above_coord(coord)),
            copied(maze.get_below_coord(coord)),
        ],
        b'-' => [
            copied(maze.get_left_coord(coord)),
            copied(maze.get_right_coord(coord)),
        ],
        b'J' => [
            copied(maze.get_above_coord(coord)),
            copied(maze.get_left_coord(coord)),
        ],
        b'L' => [
            copied(maze.get_above_coord(coord)),
            copied(maze.get_right_coord(coord)),
        ],
        b'7' => [
            copied(maze.get_below_coord(coord)),
            copied(maze.get_left_coord(coord)),
        ],
        b'F' => [
            copied(maze.get_below_coord(coord)),
            copied(maze.get_right_coord(coord)),
        ],
        _ => panic!("unexpected tile for pipe"),
    };
    candidates.into_iter().filter_map(move |coord| match coord {
        Some((c, val)) if predicate(val.clone()) => Some(c),
        _ => None,
    })
}

fn determine_starting_pipe_shape(maze: &ByteGrid, start_pos: (usize, usize)) -> u8 {
    match maze.get_cardinally_adjacent_tiles(start_pos) {
        (Some(up), Some(down), _, _) if connects_down(up) && connects_up(down) => b'|',
        (Some(up), _, Some(left), _) if connects_down(up) && connects_right(left) => b'J',
        (Some(up), _, _, Some(right)) if connects_down(up) && connects_left(right) => b'L',
        (_, Some(down), Some(left), _) if connects_up(down) && connects_right(left) => b'7',
        (_, Some(down), _, Some(right)) if connects_up(down) && connects_left(right) => b'F',
        (_, _, Some(left), Some(right)) if connects_right(left) && connects_left(right) => b'-',
        _ => panic!("no possible pipe for start position"),
    }
}

fn connects_up(tile: &u8) -> bool {
    [b'|', b'L', b'J'].contains(tile)
}
fn connects_down(tile: &u8) -> bool {
    [b'|', b'7', b'F'].contains(tile)
}
fn connects_left(tile: &u8) -> bool {
    [b'-', b'7', b'J'].contains(tile)
}
fn connects_right(tile: &u8) -> bool {
    [b'-', b'L', b'F'].contains(tile)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum InOutStatus {
    #[default]
    Unknown,
    Inside,
    Outside,
    MainLoop,
}
impl InOutStatus {
    fn flipped(&self) -> InOutStatus {
        match self {
            InOutStatus::Inside => InOutStatus::Outside,
            InOutStatus::Outside => InOutStatus::Inside,
            s => *s,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Tile {
    tile: u8,
    status: InOutStatus,
}

fn part2(maze: &Input) -> Result<usize> {
    let start_pos = maze
        .indexed_iter()
        .find(|(_, &tile)| tile == b'S')
        .ok_or(anyhow!("no start pos"))?
        .0;
    let start_pipe = determine_starting_pipe_shape(maze, start_pos);
    let mut maze = maze.mapped(|byte| Tile {
        tile: *byte,
        status: InOutStatus::Unknown,
    });
    maze[start_pos] = Tile {
        tile: start_pipe,
        status: InOutStatus::MainLoop,
    };
    color_main_loop(&mut maze, start_pos);
    color_inside_outside(&mut maze);
    println!("{}", MazeDisplayer(&maze));
    Ok(maze
        .lines()
        .iter()
        .map(|l| {
            l.iter()
                .filter(|tile| tile.status == InOutStatus::Inside)
                .count()
        })
        .sum())
}

fn color_main_loop(maze: &mut grid::Grid<Tile>, start_pos: (usize, usize)) {
    let mut current_pipes = vec![start_pos; 1];
    while !current_pipes.is_empty() {
        let next_pipes = current_pipes
            .iter()
            .flat_map(|coord| {
                get_neighboring_pipes(
                    *coord,
                    maze,
                    |tile| tile.tile,
                    |tile| tile.status != InOutStatus::MainLoop,
                )
            })
            .collect_vec();
        current_pipes
            .into_iter()
            .for_each(|coord| maze[coord].status = InOutStatus::MainLoop);
        current_pipes = next_pipes;
    }
}

fn color_inside_outside(maze: &mut grid::Grid<Tile>) {
    for line in &mut maze.lines {
        let mut status = InOutStatus::Outside;
        let mut in_main_loop = false;
        let mut from_below = false;
        for tile in line {
            if tile.status == InOutStatus::MainLoop {
                if !in_main_loop && !connects_left(&tile.tile) {
                    in_main_loop = connects_right(&tile.tile);
                    if in_main_loop {
                        from_below = connects_down(&tile.tile);
                    } else {
                        status = status.flipped();
                    }
                } else if in_main_loop && !connects_right(&tile.tile) {
                    in_main_loop = false;
                    if from_below != connects_down(&tile.tile) {
                        status = status.flipped();
                    }
                }
            } else {
                tile.status = status;
            }
        }
    }
}

struct MazeDisplayer<'a>(&'a Grid<Tile>);

impl<'a> Display for MazeDisplayer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maze = self.0;
        let display_char = |byte| {
            String::from(match byte {
                b'F' => '┌',
                b'J' => '┘',
                b'7' => '┐',
                b'L' => '└',
                b'|' => '│',
                b'-' => '─',
                b => b as char,
            })
        };
        let color = |status, s: String| match status {
            InOutStatus::Unknown => ColoredString::from(s),
            InOutStatus::Inside => s.red(),
            InOutStatus::Outside => s.blue(),
            InOutStatus::MainLoop => s.yellow(),
        };
        for line in &maze.lines {
            for tile in line {
                write!(f, "{}", color(tile.status, display_char(tile.tile)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
