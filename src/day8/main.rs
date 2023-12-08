use std::collections::HashMap;

use anyhow::anyhow;
use aoc23::*;

main!("example1.txt", 6, "example2.txt", 6);
type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;
type Input<'a> = (Vec<Dir>, Network<'a>);

const PAREN: &[char] = &['(', ')'];

#[derive(Clone, Copy)]
enum Dir {
    Left,
    Right,
}

fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .ok_or(anyhow!("bad input"))?
        .chars()
        .map(|c| if c == 'L' { Dir::Left } else { Dir::Right })
        .collect();
    let network = lines
        .skip(1)
        .filter_map(|s| s.split_once(" = ("))
        .filter_map(|(s, lr)| Some((s, lr.trim_matches(PAREN).split_once(", ")?)))
        .collect();
    Ok((directions, network))
}

fn part1((directions, network): &Input) -> Result<usize> {
    let mut current = "AAA";
    let mut count = 0;
    while current != "ZZZ" {
        let dir = directions[count % directions.len()];
        current = advance(current, dir, network);
        count += 1;
    }
    Ok(count)
}

fn part2((directions, network): &Input) -> Result<usize> {
    let starting_nodes = network.keys().copied().filter(|node| node.ends_with('A'));
    let loop_values = starting_nodes.map(|node| count_until_exit(node, network, directions));
    loop_values
        .reduce(num::integer::lcm)
        .ok_or(anyhow!("no exits?"))
}

fn advance<'a>(current: &'a str, dir: Dir, network: &Network<'a>) -> &'a str {
    match dir {
        Dir::Left => network[current].0,
        Dir::Right => network[current].1,
    }
}
fn count_until_exit<'a>(
    mut current: &'a str,
    network: &Network<'a>,
    directions: &Vec<Dir>,
) -> usize {
    let mut count = 0;
    let nb_directions = directions.len();
    while !current.ends_with('Z') {
        let dir = directions[count % nb_directions];
        current = advance(current, dir, network);
        count += 1;
    }
    count
}
