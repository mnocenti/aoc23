use aoc23::*;
use std::str::FromStr;

main!(day2, "../inputs/input2.txt");

test_with_example!(day2, "../inputs/example2.txt", 8, 2286);

fn day2(input: &str) -> Result<(usize, usize)> {
    const CUBE_LIMITS: Cubes = Cubes {
        reds: 12,
        greens: 13,
        blues: 14,
    };
    let games = parse_games(input)?;
    let part1 = games
        .iter()
        .filter_map(|game| game.is_possible(&CUBE_LIMITS).then_some(game.id))
        .sum();

    let part2 = games.iter().map(Game::minimal_power).sum();

    Ok((part1, part2))
}

fn parse_games(input: &str) -> Result<Vec<Game>> {
    input.lines().map(Game::from_str).collect()
}

#[derive(Debug)]
struct Game {
    id: usize,
    cubes_sets: Vec<Cubes>,
}

#[derive(Default, Debug)]
struct Cubes {
    reds: usize,
    greens: usize,
    blues: usize,
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
                reds: l.reds.max(r.reds),
                greens: l.greens.max(r.greens),
                blues: l.blues.max(r.blues),
            })
            .power()
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start, rest) = s.split_once(':').ok_or(parse_error(s, "missing ':'"))?;
        let id = start
            .split_once(' ')
            .ok_or(parse_error(s, "missing ' '"))?
            .1
            .parse()?;
        let cubes_sets: Result<_, Self::Err> = rest.split(';').map(Cubes::from_str).collect();
        Ok(Game {
            id,
            cubes_sets: cubes_sets?,
        })
    }
}

impl Cubes {
    fn is_possible(&self, game_limits: &Cubes) -> bool {
        self.reds <= game_limits.reds
            && self.greens <= game_limits.greens
            && self.blues <= game_limits.blues
    }
    fn power(&self) -> usize {
        self.reds * self.greens * self.blues
    }
}

impl FromStr for Cubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split(',').try_fold(Cubes::default(), |mut cubes, s| {
            Ok(
                match s
                    .trim()
                    .split_once(' ')
                    .ok_or(parse_error(s, "missing ' '"))?
                {
                    (n, "red") => {
                        cubes.reds += n.parse::<usize>()?;
                        cubes
                    }
                    (n, "green") => {
                        cubes.greens += n.parse::<usize>()?;
                        cubes
                    }
                    (n, "blue") => {
                        cubes.blues += n.parse::<usize>()?;
                        cubes
                    }
                    _ => Err(parse_error(s, ""))?,
                },
            )
        })
    }
}
