use std::mem::swap;

use aoc23::*;

use aoc23::grid::{Coord, Grid};

main!();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TileType {
    GardenPlot,
    Rock,
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    reached: bool,
}

type Land = Grid<Tile>;

fn parse(input: &str) -> Result<Land> {
    Ok(Land::from_lines_mapped(input, |c| Tile {
        tile_type: match c {
            b'.' | b'S' => TileType::GardenPlot,
            b'#' => TileType::Rock,
            _ => panic!("parse error"),
        },
        reached: c == b'S',
    }))
}

fn part1(land: &Land) -> Result<usize> {
    let steps = if land.width <= 12 { 6 } else { 64 };
    let mut current = land.clone();
    let mut next = land.clone();
    for _ in 0..steps {
        step_once(&mut current, &mut next);
    }
    Ok(current.iter().filter(|tile| tile.reached).count())
}

fn step_once(current: &mut Land, next: &mut Land) {
    next.iter_mut().for_each(|tile| tile.reached = false);

    for coord in current
        .indexed_iter()
        .filter_map(|(coord, tile)| tile.reached.then_some(coord))
    {
        let (up, down, left, right) = next.get_cardinally_adjacent_coords(coord);
        let mut mark_reached = |coord| {
            if let Some(tile) = next.get_mut(coord) {
                if tile.tile_type != TileType::Rock {
                    tile.reached = true;
                }
            }
        };
        mark_reached(up);
        mark_reached(down);
        mark_reached(left);
        mark_reached(right);
    }

    swap(current, next);
}

const STEPS: usize = 26501365;
const MAP_SIZE: usize = 131;
const NB_FULL_MAPS: usize = (STEPS - MAP_SIZE / 2) / MAP_SIZE; // 202 300

// mostly solved geometrically, see day21.png 😄
fn part2(land: &Land) -> Result<usize> {
    let mut map = land.mapped(|tile| tile.tile_type);
    block_unreachable_spots(&mut map);
    let full_even = count_garden_plots(&map, true, |_| true);
    let full_odd = count_garden_plots(&map, false, |_| true);
    let bot_right_even = count_garden_plots(&map, true, |(x, y)| 260 - x - y < 65);
    let bot_left_even = count_garden_plots(&map, true, |(x, y)| 130 - y + x < 65);
    let top_left_even = count_garden_plots(&map, true, |(x, y)| x + y < 65);
    let top_right_even = count_garden_plots(&map, true, |(x, y)| 130 - x + y < 65);
    let bot_right_odd = count_garden_plots(&map, false, |(x, y)| 260 - x - y < 65);
    let bot_left_odd = count_garden_plots(&map, false, |(x, y)| 130 - y + x < 65);
    let top_left_odd = count_garden_plots(&map, false, |(x, y)| x + y < 65);
    let top_right_odd = count_garden_plots(&map, false, |(x, y)| 130 - x + y < 65);

    let total = full_even * NB_FULL_MAPS * NB_FULL_MAPS
        + full_odd * (NB_FULL_MAPS - 1) * (NB_FULL_MAPS - 1)
        + bot_right_even * NB_FULL_MAPS
        + bot_left_even * NB_FULL_MAPS
        + top_left_even * NB_FULL_MAPS
        + top_right_even * NB_FULL_MAPS
        + (full_odd - top_right_odd) * (NB_FULL_MAPS - 1)
        + (full_odd - top_left_odd) * (NB_FULL_MAPS - 1)
        + (full_odd - bot_left_odd) * (NB_FULL_MAPS - 1)
        + (full_odd - bot_right_odd) * (NB_FULL_MAPS - 1)
        + (full_odd - top_left_odd - top_right_odd)
        + (full_odd - top_left_odd - bot_left_odd)
        + (full_odd - bot_left_odd - bot_right_odd)
        + (full_odd - bot_right_odd - top_right_odd);
    Ok(total)
}

fn block_unreachable_spots(map: &mut Grid<TileType>) {
    for coord in map.coords_iter() {
        if let (
            Some(TileType::Rock),
            Some(TileType::Rock),
            Some(TileType::Rock),
            Some(TileType::Rock),
        ) = map.get_cardinally_adjacent_tiles(coord)
        {
            map[coord] = TileType::Rock
        }
    }
}

fn count_garden_plots(
    map: &Grid<TileType>,
    even: bool,
    predicate: impl Fn(Coord) -> bool,
) -> usize {
    map.indexed_iter()
        .filter(|(_, tile)| **tile == TileType::GardenPlot)
        .filter(|((x, y), _)| (*x + *y) % 2 == (1 - even as usize))
        .filter(|(coord, _)| predicate(*coord))
        .count()
}

impl From<TileType> for char {
    fn from(value: TileType) -> Self {
        match value {
            TileType::GardenPlot => '.',
            TileType::Rock => '#',
        }
    }
}
