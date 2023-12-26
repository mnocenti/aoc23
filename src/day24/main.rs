use aoc23::*;
use itertools::Itertools;

main!(2, 47);

#[apply(parse_ordered!)]
#[delim(", ")]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Coord {
    #[parse(trim)]
    x: f32,
    #[parse(trim)]
    y: f32,
    #[parse(trim)]
    z: f32,
}

#[apply(parse_ordered!)]
#[delim(" @ ")]
#[derive(Debug, Default, PartialEq, PartialOrd)]
struct Hailstone {
    #[parse()]
    pos: Coord,
    #[parse()]
    vel: Coord,
}

fn parse(input: &str) -> Result<Vec<Hailstone>> {
    collect_lines(input)
}

fn part1(hailstones: &[Hailstone]) -> Result<usize> {
    let test_area = if hailstones.len() < 10 {
        // example
        7f32..27f32
    } else {
        // real input
        200000000000000f32..400000000000000f32
    };
    let in_test_area = |x, y| test_area.contains(&x) && test_area.contains(&y);
    let lines = hailstones
        .iter()
        .map(|hailstone| (hailstone, Line2D::from(hailstone)))
        .collect_vec();
    Ok(lines
        .iter()
        .permutations(2)
        .filter(|l| l[0] < l[1])
        .filter(|l| match intersection(l[0], l[1]) {
            IntersectionPoint::Point(x, y) => in_test_area(x, y),
            IntersectionPoint::AllPoints => true,
            IntersectionPoint::None => false,
        })
        .count())
}

fn part2(_hailstones: &[Hailstone]) -> Result<usize> {
    Ok(0)
}

#[derive(Debug, PartialEq, PartialOrd)]
// y = mx + p
struct Line2D {
    m: f32,
    p: f32,
}

impl From<&Hailstone> for Line2D {
    fn from(h: &Hailstone) -> Self {
        let m = h.vel.y / h.vel.x;
        let p = h.pos.y - m * h.pos.x;
        Line2D { m, p }
    }
}

enum IntersectionPoint {
    None,
    Point(f32, f32),
    AllPoints,
}

fn intersection(
    (h1, l1): &(&Hailstone, Line2D),
    (h2, l2): &(&Hailstone, Line2D),
) -> IntersectionPoint {
    if l1.m == l2.m {
        // parallel lines
        // intersect only if they are the same line
        if l1.p == l2.p {
            IntersectionPoint::AllPoints
        } else {
            IntersectionPoint::None
        }
    } else {
        let x = (l2.p - l1.p) / (l1.m - l2.m);
        let y = l1.m * x + l1.p;
        if in_past(y, h1) || in_past(y, h2) {
            IntersectionPoint::None
        } else {
            IntersectionPoint::Point(x, y)
        }
    }
}

fn in_past(y: f32, h: &Hailstone) -> bool {
    (y - h.pos.y).signum() != h.vel.y.signum()
}
