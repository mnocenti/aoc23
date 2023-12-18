use anyhow::anyhow;
use aoc23::*;
use itertools::Itertools;

main!(62, 952408144115);

type Coord = (isize, isize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

struct Instr {
    dir: Dir,
    count: isize,
}

type DigPlan = (Vec<Instr>, Vec<Instr>);

fn parse(input: &str) -> Result<DigPlan> {
    let instructions1 = input
        .lines()
        .map(|l| -> Result<Instr> {
            let mut split = l.split(' ');
            Ok(Instr {
                dir: split.next().ok_or(anyhow!("parse failed"))?.parse()?,
                count: split.next().ok_or(anyhow!("parse failed"))?.parse()?,
            })
        })
        .try_collect()?;
    let instructions2 = input
        .lines()
        .map(|l| -> Result<Instr> {
            let hexa = l.split_once('#').ok_or(anyhow!("parse failed"))?.1;
            Ok(Instr {
                dir: match hexa.chars().nth(5) {
                    Some('0') => Dir::Right,
                    Some('1') => Dir::Down,
                    Some('2') => Dir::Left,
                    Some('3') => Dir::Up,
                    _ => Err(anyhow!("parse failed"))?,
                },
                count: isize::from_str_radix(&hexa[0..5], 16)?,
            })
        })
        .try_collect()?;

    Ok((instructions1, instructions2))
}

fn part1((instructions, _): &DigPlan) -> Result<usize> {
    let polygon = make_polygon(instructions);
    Ok(area(&polygon))
}

fn part2((_, instructions): &DigPlan) -> Result<usize> {
    let polygon = make_polygon(instructions);
    Ok(area(&polygon))
}

fn make_polygon(instructions: &[Instr]) -> Vec<Coord> {
    instructions
        .iter()
        .scan((0, 0), |(x, y), instr| {
            match instr.dir {
                Dir::Up => *y += instr.count,
                Dir::Down => *y -= instr.count,
                Dir::Left => *x -= instr.count,
                Dir::Right => *x += instr.count,
            }
            Some((*x, *y))
        })
        .collect_vec()
}

fn area(polygon: &[(isize, isize)]) -> usize {
    let inside_area = polygon
        .iter()
        .tuple_windows()
        .map(|((x0, y0), (x1, y1))| x0 * y1 - x1 * y0)
        .sum::<isize>()
        .unsigned_abs()
        / 2;
    let distance = |((x0, y0), (x1, y1)): (&Coord, &Coord)| x0.abs_diff(*x1) + y0.abs_diff(*y1);
    let perimeter: usize = polygon.iter().tuple_windows().map(distance).sum::<usize>()
        + distance((polygon.first().unwrap(), polygon.last().unwrap()));
    inside_area + perimeter / 2 + 1
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('U') => Ok(Dir::Up),
            Some('D') => Ok(Dir::Down),
            Some('L') => Ok(Dir::Left),
            Some('R') => Ok(Dir::Right),
            _ => Err(anyhow!("failed to parse direction")),
        }
    }
}
