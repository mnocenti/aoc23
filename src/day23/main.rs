use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use anyhow::anyhow;
use aoc23::{
    grid::{Coord, Dir, Grid},
    *,
};
use itertools::Itertools;

main!(94, 154);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

type Trail = Grid<Tile>;

fn parse(input: &str) -> Result<Trail> {
    Ok(Trail::from_lines_mapped(input, |b| match b {
        b'.' => Tile::Path,
        b'#' => Tile::Forest,
        b'^' => Tile::Slope(Dir::Up),
        b'v' => Tile::Slope(Dir::Down),
        b'>' => Tile::Slope(Dir::Right),
        b'<' => Tile::Slope(Dir::Left),
        _ => panic!("trail parse error"),
    }))
}

fn part1(trail: &Trail) -> Result<usize> {
    let mut graph = Graph::from(trail, true);
    Ok(graph.longest_icy_hike())
}

fn part2(trail: &Trail) -> Result<usize> {
    let graph = Graph::from(trail, false);
    graph
        .longest_dry_hike((1, 0), (trail.width - 2, trail.height - 1))
        .ok_or(anyhow!("couldn't find max distance"))
}

#[derive(Debug, Default)]
struct Node {
    distance_from_start: usize,
    nb_sources: usize,
    sources: Vec<Coord>,
    dests: Vec<(usize, Coord)>,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<Coord, Node>,
    order: Vec<Coord>,
}

struct Intersection {
    distance: usize,
    coord: Coord,
    nb_sources: usize,
    paths: Vec<Dir>,
}

impl Graph {
    fn from(trail: &Trail, icy: bool) -> Graph {
        let start = (1, 0);
        let mut graph = Graph::default();
        graph.nodes.insert(start, Node::default());
        graph.order.push(start);
        let mut paths = vec![(Dir::Down, start)];
        let mut next_paths = Vec::new();

        while !paths.is_empty() {
            for (dir, coord) in &paths {
                let node = graph.nodes.entry(*coord).or_default();
                if icy && node.sources.len() < node.nb_sources {
                    // not ready to go down this path yet
                    next_paths.push((*dir, *coord));
                    continue;
                }
                let intersection = get_next_intersection(trail, *coord, *dir, 0, icy);
                if !node
                    .dests
                    .contains(&(intersection.distance, intersection.coord))
                {
                    node.dests.push((intersection.distance, intersection.coord));
                }
                let inter_node = graph.nodes.entry(intersection.coord).or_default();
                inter_node.nb_sources = intersection.nb_sources;
                inter_node.sources.push(*coord);
                if icy && inter_node.nb_sources == inter_node.sources.len() {
                    graph.order.push(intersection.coord);
                }
                if !icy
                    && !inter_node
                        .dests
                        .iter()
                        .contains(&(intersection.distance, *coord))
                {
                    inter_node.dests.push((intersection.distance, *coord));
                }
                if inter_node.sources.len() == 1 {
                    next_paths.extend(
                        intersection
                            .paths
                            .into_iter()
                            .map(|dir| (dir, intersection.coord)),
                    );
                }
            }
            paths.clear();
            swap(&mut paths, &mut next_paths);
        }
        graph
    }

    fn longest_icy_hike(&mut self) -> usize {
        for coord in &self.order {
            let node = &self.nodes[coord];
            let distance = node.distance_from_start;
            for (dist, dest_coord) in node.dests.iter().copied().collect_vec() {
                let dest = self.nodes.entry(dest_coord).or_default();
                if dest.distance_from_start < distance + dist {
                    dest.distance_from_start = distance + dist;
                }
            }
        }

        self.nodes[self.order.last().unwrap()].distance_from_start
    }

    fn longest_dry_hike(&self, start: Coord, goal: Coord) -> Option<usize> {
        let mut seen = HashSet::new();
        self.max_distance(start, goal, &mut seen)
    }

    fn max_distance(&self, start: Coord, goal: Coord, seen: &mut HashSet<Coord>) -> Option<usize> {
        if seen.contains(&start) {
            return None;
        }
        if start == goal {
            return Some(0);
        }
        seen.insert(start);
        let max_distance = self.nodes[&start]
            .dests
            .iter()
            .filter_map(|(distance, dest)| {
                self.max_distance(*dest, goal, seen)
                    .map(|dist| dist + distance)
            })
            .max();
        seen.remove(&start);
        max_distance
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

fn get_next_intersection(
    trail: &Grid<Tile>,
    coord: Coord,
    dir: Dir,
    distance: usize,
    icy: bool,
) -> Intersection {
    let next_coord = get_next(coord, dir);
    if let Some(tile) = trail.get(coord) {
        assert!(tile != &Tile::Forest, "pathing error at {next_coord:?}");
    } else {
        panic!("coord {next_coord:?} out of bounds !");
    }
    let neighbors = Trail::get_neighbors_coord(next_coord);
    let paths = neighbors
        .iter()
        .filter(|(next_dir, coord)| {
            !next_dir.is_reverse(dir) && trail.get(*coord).is_some_and(|t| (t != &Tile::Forest))
        })
        .collect_vec();
    if paths.len() == 1 {
        let (path_dir, _) = paths[0];
        get_next_intersection(trail, next_coord, *path_dir, distance + 1, icy)
    } else if paths.is_empty() {
        // end
        Intersection {
            distance: distance + 1,
            coord: next_coord,
            nb_sources: 1,
            paths: Vec::new(),
        }
    } else if icy {
        let nb_sources = neighbors
            .iter()
            .filter(|(dir, coord)| {
                trail.get(*coord).is_some_and(|tile| {
                    get_slope_dir(tile).is_some_and(|slope_dir| dir.is_reverse(slope_dir))
                })
            })
            .count();
        let allowed_dirs = paths
            .iter()
            .filter(|(dir, coord)| {
                trail.get(*coord).is_some_and(|tile| {
                    get_slope_dir(tile)
                        .map(|slope_dir| !dir.is_reverse(slope_dir))
                        .unwrap_or(true)
                })
            })
            .map(|(dir, _)| *dir)
            .collect();
        Intersection {
            distance: distance + 1,
            coord: next_coord,
            nb_sources,
            paths: allowed_dirs,
        }
    } else {
        Intersection {
            distance: distance + 1,
            coord: next_coord,
            nb_sources: neighbors
                .iter()
                .filter(|(_, coord)| trail.get(*coord).is_some_and(|t| (t != &Tile::Forest)))
                .count(),
            paths: paths.iter().map(|(dir, _)| *dir).collect(),
        }
    }
}

fn get_slope_dir(tile: &Tile) -> Option<Dir> {
    if let Tile::Slope(dir) = tile {
        Some(*dir)
    } else {
        None
    }
}
