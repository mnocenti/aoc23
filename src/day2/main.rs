use aoc23::*;
use std::str::FromStr;

main!(8, 2286);

fn parse(input: &str) -> Result<Vec<Game>> {
    collect_lines(input)
}

fn part1(games: &[Game]) -> Result<usize> {
    const CUBE_LIMITS: Cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    Ok(games
        .iter()
        .filter_map(|game| game.is_possible(&CUBE_LIMITS).then_some(game.id))
        .sum())
}

fn part2(games: &[Game]) -> Result<usize> {
    Ok(games.iter().map(Game::minimal_power).sum())
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    cubes_sets: Vec<Cubes>,
}

impl_fromstr_ordered!(
    delim: ':',
    Game {
        id: "Game ([0-9]+)",
        cubes_sets : {collect ';'},
    }
);

#[derive(Default, Debug)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl_fromstr_matching!(
    delim: ',',
    Cubes {
        red: "([0-9]+) red",
        green: "([0-9]+) green",
        blue: "([0-9]+) blue",
    }
);

impl Game {
    fn is_possible(&self, game_limits: &Cubes) -> bool {
        self.cubes_sets
            .iter()
            .all(|cubes| cubes.is_possible(game_limits))
    }

    fn minimal_power(&self) -> usize {
        self.cubes_sets
            .iter()
            .fold(Cubes::default(), |l, r| Cubes {
                red: l.red.max(r.red),
                green: l.green.max(r.green),
                blue: l.blue.max(r.blue),
            })
            .power()
    }
}

impl Cubes {
    fn is_possible(&self, game_limits: &Cubes) -> bool {
        self.red <= game_limits.red
            && self.green <= game_limits.green
            && self.blue <= game_limits.blue
    }
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}
