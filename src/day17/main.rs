use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use aoc23::{
    grid::{Coord, Grid},
    *,
};

main!(102, 94);

#[derive(Debug, Clone)]
struct Tile {
    cost: u32,
    min_distance: [u32; 4], // min distance for each direction
}

type Map = Grid<Tile>;

fn parse(input: &str) -> Result<Map> {
    Ok(Map::from_lines_mapped(input, |b| Tile {
        cost: (b - b'0') as u32,
        min_distance: [u32::MAX; 4],
    }))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    distance: u32,
    coord: Coord,
    dir: Dir,
}

type Graph = BinaryHeap<Node>;

fn part1(map: &Map) -> Result<u32> {
    find_path(map, 1, 3)
}

fn part2(map: &Map) -> Result<u32> {
    find_path(map, 4, 10)
}

fn find_path(map: &Map, min_straight: u8, max_straight: u8) -> Result<u32> {
    let mut map = map.clone();
    let destination = (map.width - 1, map.height - 1);
    let mut graph = Graph::new();
    graph.push(Node {
        distance: 0,
        coord: (0, 0),
        dir: Dir::Left,
    });
    graph.push(Node {
        distance: 0,
        coord: (0, 0),
        dir: Dir::Up,
    });
    while !graph.peek().is_some_and(|node| node.coord == destination) {
        let node = graph.pop().expect("I'm lost");
        if node.distance > map[node.coord].min_distance[node.dir as usize] {
            continue;
        }
        add_nodes_from(
            &mut graph,
            &mut map,
            &node,
            rotate_clockwise(node.dir),
            min_straight,
            max_straight,
        );
        add_nodes_from(
            &mut graph,
            &mut map,
            &node,
            rotate_counter_clockwise(node.dir),
            min_straight,
            max_straight,
        );
    }
    Ok(graph.pop().unwrap().distance)
}

fn add_nodes_from(
    graph: &mut Graph,
    map: &mut Map,
    node: &Node,
    dir: Dir,
    min_straight: u8,
    max_straight: u8,
) {
    let mut next = node.coord;
    let mut total_distance = node.distance;
    for i in 0..max_straight {
        next = get_next(next, dir);
        if let Some(tile) = map.get_mut(next) {
            total_distance += tile.cost;
            let min_distance = &mut tile.min_distance[dir as usize];
            if i + 1 >= min_straight && total_distance < *min_distance {
                *min_distance = total_distance;
                graph.push(Node {
                    distance: total_distance,
                    coord: next,
                    dir,
                });
            }
        } else {
            break;
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

fn rotate_clockwise(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    }
}

fn rotate_counter_clockwise(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right,
        Dir::Left => Dir::Down,
        Dir::Right => Dir::Up,
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.distance).cmp(&Reverse(other.distance))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
