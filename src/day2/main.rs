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

#[apply(parse_ordered!)]
#[delim(':')]
#[derive(Debug, Default)]
struct Game {
    #[parse(re("Game ([0-9]+)"))]
    id: usize,
    #[parse(collect(';'))]
    cubes_sets: Vec<Cubes>,
}

#[apply(parse_matching!)]
#[delim(',')]
#[derive(Default, Debug)]
struct Cubes {
    #[parse("([0-9]+) red")]
    red: usize,
    #[parse("([0-9]+) green")]
    green: usize,
    #[parse("([0-9]+) blue")]
    blue: usize,
}

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
