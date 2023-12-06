
use std::collections::HashSet;
use std::str::FromStr;

use aoc23::*;
use itertools::Itertools;

main!(288, 71503);
type Input = (Vec<(usize,usize)>, (usize,usize));

fn parse(input: &str) -> Result<Input> {
    let (time,distance) =  input.split_once('\n').ok_or(parse_error(input, "expected 2 lines"))?;
    let part1_input = time.split_whitespace().skip(1).map(|s| s.parse().unwrap()).zip(distance.split_whitespace().skip(1).map(|s| s.parse().unwrap())).collect_vec();
    let part2_input = (time.replace(' ', "").split_once(':').unwrap().1.trim().parse()?, distance.replace(' ', "").split_once(':').unwrap().1.trim().parse()?);
    Ok((part1_input, part2_input))
}

fn part1(races : &Input) -> Result<usize> {
    Ok(races.0.iter().map(|&(t,d)|nb_wining_ways(t,d)).product())
}

fn part2(race : &Input) -> Result<usize> {
    Ok(nb_wining_ways(race.1.0,race.1.1))
}

fn nb_wining_ways(time : usize, distance : usize) -> usize {
    (1..time).map(|t| (time-t)*t).filter(|d| *d > distance).count()
}
