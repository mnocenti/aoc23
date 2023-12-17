use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

use aoc23::{
    grid::{ByteGrid, Coord},
    *,
};

main!(102, 94);

type Map = ByteGrid;

fn parse(input: &str) -> Result<Map> {
    Ok(Map::from_lines_mapped(input, |b| b - b'0'))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    distance: usize,
    coord: Coord,
    dir: Dir,
}

struct Graph {
    by_distance: BinaryHeap<Node>,
    by_coord: HashMap<Coord, Vec<Node>>,
}

fn part1(map: &Map) -> Result<usize> {
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
        add_nodes_from(&mut graph, map, &node, rotate_clockwise(node.dir), 1, 3);
        add_nodes_from(
            &mut graph,
            map,
            &node,
            rotate_counter_clockwise(node.dir),
            1,
            3,
        );
    }
    Ok(graph.pop().unwrap().distance)
}

fn part2(map: &Map) -> Result<usize> {
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
        add_nodes_from(&mut graph, map, &node, rotate_clockwise(node.dir), 4, 10);
        add_nodes_from(
            &mut graph,
            map,
            &node,
            rotate_counter_clockwise(node.dir),
            4,
            10,
        );
    }
    Ok(graph.pop().unwrap().distance)
}

fn add_nodes_from(
    graph: &mut Graph,
    map: &Map,
    node: &Node,
    dir: Dir,
    min_straight: u8,
    max_straight: u8,
) {
    let mut next = node.coord;
    let mut total_distance = node.distance;
    for i in 0..max_straight {
        next = get_next(next, dir);
        if let Some(distance) = map.get(next) {
            total_distance += *distance as usize;
            if i + 1 >= min_straight {
                graph.push_if_better(Node {
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

impl Graph {
    fn new() -> Graph {
        Graph {
            by_distance: BinaryHeap::new(),
            by_coord: HashMap::new(),
        }
    }

    fn push(&mut self, node: Node) {
        self.by_coord
            .entry(node.coord)
            .and_modify(|nodes| nodes.push(node.clone()))
            .or_insert(vec![node.clone()]);
        self.by_distance.push(node);
    }

    fn peek(&self) -> Option<&Node> {
        self.by_distance.peek()
    }

    fn pop(&mut self) -> Option<Node> {
        if let Some(node) = self.by_distance.pop() {
            if let Some(nodes) = self.by_coord.get_mut(&node.coord) {
                if let Some(pos) = nodes.iter().position(|n| *n == node) {
                    nodes.remove(pos);
                }
            }
            Some(node)
        } else {
            None
        }
    }

    fn push_if_better(&mut self, node: Node) {
        if let Some(nodes) = self.by_coord.get(&node.coord) {
            if nodes
                .iter()
                .any(|n| n.dir == node.dir && n.distance <= node.distance)
            {
                return;
            }
        }
        self.push(node);
    }
}
