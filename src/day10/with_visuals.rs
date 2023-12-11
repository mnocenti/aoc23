use anyhow::anyhow;
use aoc23::grid::{ByteGrid, Coord, Grid};
use aoc23::*;

use itertools::Itertools;

use pixel_canvas::{Canvas, Color, Image, RC};

main!();

type Input = ByteGrid;

fn parse(input: &str) -> Result<Input> {
    Ok(ByteGrid::from_lines(input))
}

fn part1(_maze: &Input) -> Result<usize> {
    Ok(0)
}

const TILE_SIZE: usize = 6;

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

    let mut count = 0;
    let mut current_pipes = vec![start_pos; 1];
    let mut scan_coord = (0, 0);
    let mut status = InOutStatus::Outside;
    let mut in_main_loop = false;
    let mut from_below = false;
    let w = maze.width * TILE_SIZE;
    let h = maze.height * TILE_SIZE;
    let canvas = Canvas::new(w, h).title("Day 10").state(()).show_ms(true);
    canvas.render(move |(), image| {
        if count == 0 {
            draw(&maze, image);
        }
        count += 1;

        for _ in 0..10 {
            if !current_pipes.is_empty() {
                let next_pipes = current_pipes
                    .iter()
                    .flat_map(|coord| {
                        get_neighboring_pipes(
                            *coord,
                            &maze,
                            |tile| tile.tile,
                            |tile| tile.status != InOutStatus::MainLoop,
                        )
                    })
                    .collect_vec();
                current_pipes.iter().for_each(|coord| {
                    maze[*coord].status = InOutStatus::MainLoop;
                    draw_tile_at(*coord, &maze[*coord], image);
                });

                current_pipes = next_pipes;
            } else if scan_coord.1 < maze.height {
                let tile = &mut maze[scan_coord];
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
                draw_tile_at(scan_coord, &maze[scan_coord], image);
                scan_coord.0 += 1;
                if scan_coord.0 >= maze.width {
                    scan_coord.0 = 0;
                    scan_coord.1 += 1;
                    status = InOutStatus::Outside;
                    in_main_loop = false;
                    from_below = false;
                }
            }
        }
    });
    Ok(0)
}

fn draw(maze: &Grid<Tile>, image: &mut Image) {
    for point in (0..maze.width).cartesian_product(0..maze.height) {
        draw_tile_at(point, &maze[point], image);
    }
}

fn draw_tile_at((x0, y0): Coord, tile: &Tile, image: &mut Image) {
    let color = tile.status.get_color();
    let height = image.height();
    let (x0, y0) = (x0 * TILE_SIZE, y0 * TILE_SIZE);
    let mut draw = |x: usize, y: usize| {
        image[RC(height - y0 - 2 * y - 1, x0 + 2 * x)] = color;
        image[RC(height - y0 - 2 * y - 1, x0 + 2 * x + 1)] = color;
        image[RC(height - y0 - 2 * y - 2, x0 + 2 * x)] = color;
        image[RC(height - y0 - 2 * y - 2, x0 + 2 * x + 1)] = color;
    };
    match tile.tile {
        b'|' => {
            draw(1, 0);
            draw(1, 1);
            draw(1, 2);
        }
        b'J' => {
            draw(1, 0);
            draw(1, 1);
            draw(0, 1);
        }
        b'L' => {
            draw(1, 0);
            draw(1, 1);
            draw(2, 1);
        }
        b'7' => {
            draw(0, 1);
            draw(1, 1);
            draw(1, 2);
        }
        b'F' => {
            draw(2, 1);
            draw(1, 1);
            draw(1, 2);
        }
        b'-' => {
            draw(0, 1);
            draw(1, 1);
            draw(2, 1);
        }
        _ => (),
    }
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

    const UNKNOWN_COLOR: Color = Color {
        r: 0xA0,
        g: 0xA0,
        b: 0xA0,
    };
    const INSIDE_COLOR: Color = Color {
        r: 0xFF,
        g: 0x20,
        b: 0x20,
    };
    const OUTSIDE_COLOR: Color = Color {
        r: 0x40,
        g: 0x40,
        b: 0xF0,
    };
    const MAINLOOP_COLOR: Color = Color {
        r: 0xD0,
        g: 0xD0,
        b: 0x40,
    };

    fn get_color(&self) -> Color {
        match *self {
            InOutStatus::Unknown => Self::UNKNOWN_COLOR,
            InOutStatus::Inside => Self::INSIDE_COLOR,
            InOutStatus::Outside => Self::OUTSIDE_COLOR,
            InOutStatus::MainLoop => Self::MAINLOOP_COLOR,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Tile {
    tile: u8,
    status: InOutStatus,
}
