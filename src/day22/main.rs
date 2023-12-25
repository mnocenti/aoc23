use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use aoc23::*;
use itertools::Itertools;
use std::ops::Range;

main!(5, 7);

fn parse(input: &str) -> Result<Vec<Brick>> {
    let mut res: Vec<Brick> = collect_lines(input)?;
    res.iter_mut().for_each(|brick| {
        if brick.start.z > brick.end.z {
            swap(&mut brick.start, &mut brick.end)
        }
    });
    res.sort_by_key(|brick| brick.start.z);
    Ok(res)
}

fn part1(bricks: &Vec<Brick>) -> Result<usize> {
    let mut bricks = (*bricks).clone();
    fall(&mut bricks);
    let graph = make_support_graph(&bricks);
    Ok(count_safe_bricks(&graph))
}

fn part2(bricks: &Vec<Brick>) -> Result<usize> {
    let mut bricks = (*bricks).clone();
    fall(&mut bricks);
    let graph = make_support_graph(&bricks);
    Ok(chain_reaction_potential(&graph))
}

#[apply(parse_ordered!)]
#[delim(',')]
#[derive(Debug, Default, Clone)]
struct Coord {
    #[parse()]
    x: usize,
    #[parse()]
    y: usize,
    #[parse()]
    z: usize,
}

#[apply(parse_ordered!)]
#[delim('~')]
#[derive(Debug, Default, Clone)]
struct Brick {
    #[parse()]
    start: Coord,
    #[parse()]
    end: Coord,
}

fn abs_range(a: usize, b: usize) -> Range<usize> {
    (a.min(b))..(a.max(b) + 1)
}

impl Brick {
    fn bottom_cubes(&self) -> impl Iterator<Item = Coord> {
        let xr = abs_range(self.start.x, self.end.x);
        let yr = abs_range(self.start.y, self.end.y);
        let z = self.start.z;
        xr.cartesian_product(yr)
            .map(move |(x, y)| Coord { x, y, z })
    }

    fn top_cubes(&self) -> impl Iterator<Item = Coord> {
        let xr = abs_range(self.start.x, self.end.x);
        let yr = abs_range(self.start.y, self.end.y);
        let z = self.end.z;
        xr.cartesian_product(yr)
            .map(move |(x, y)| Coord { x, y, z })
    }

    fn z_at(&self, x: usize, y: usize) -> Option<usize> {
        self.top_cubes()
            .filter_map(|coord| (coord.x == x && coord.y == y).then_some(coord.z))
            .next()
    }

    fn has_cube_at(&self, x: usize, y: usize) -> bool {
        abs_range(self.start.x, self.end.x).contains(&x)
            && abs_range(self.start.y, self.end.y).contains(&y)
    }

    fn set_z(&mut self, z: usize) {
        let height = self.end.z - self.start.z;
        self.start.z = z;
        self.end.z = z + height;
    }
}

fn fall(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let z = bricks[i]
            .bottom_cubes()
            .flat_map(|coord| {
                bricks
                    .iter()
                    .take(i)
                    .filter_map(move |lower_brick| lower_brick.z_at(coord.x, coord.y))
            })
            .max()
            .unwrap_or(0)
            + 1;
        bricks[i].set_z(z);
    }
}

#[derive(Debug, Default)]
struct SupportNode {
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

fn make_support_graph(bricks: &[Brick]) -> HashMap<usize, SupportNode> {
    let mut graph: HashMap<usize, SupportNode> = HashMap::new();
    for (i, brick_to_remove) in bricks.iter().enumerate() {
        // get bricks supported by this one
        let supported_by_this = bricks
            .iter()
            .enumerate()
            .skip(i + 1)
            .filter(|(_, brick)| brick.start.z == brick_to_remove.end.z + 1)
            .filter_map(|(j, brick)| {
                brick_to_remove
                    .top_cubes()
                    .any(|coord| brick.has_cube_at(coord.x, coord.y))
                    .then_some(j)
            })
            .collect_vec();
        for supported_brick in supported_by_this.iter() {
            graph
                .entry(*supported_brick)
                .or_default()
                .supported_by
                .push(i);
        }
        graph.entry(i).or_default().supports = supported_by_this;
    }
    graph
}

fn count_safe_bricks(graph: &HashMap<usize, SupportNode>) -> usize {
    graph
        .values()
        .filter(|node| {
            node.supports
                .iter()
                .all(|brick_idx| graph[brick_idx].supported_by.len() > 1)
        })
        .count()
}

fn chain_reaction_potential(graph: &HashMap<usize, SupportNode>) -> usize {
    graph
        .keys()
        .map(|brick_idx| get_nb_falling_bricks(*brick_idx, graph))
        .sum()
}

fn get_nb_falling_bricks(brick_idx: usize, graph: &HashMap<usize, SupportNode>) -> usize {
    let mut removed = HashSet::new();
    let mut next = HashSet::new();
    next.insert(brick_idx);
    while !next.is_empty() {
        next = next.difference(&removed).copied().collect();
        removed.extend(next.iter());
        next = next
            .into_iter()
            .flat_map(|brick_idx| {
                let node = &graph[&brick_idx];
                node.supports
                    .iter()
                    .filter(|b| {
                        graph[b]
                            .supported_by
                            .iter()
                            .all(|support| removed.contains(support))
                    })
                    .copied()
            })
            .collect();
    }
    removed.len() - 1
}
