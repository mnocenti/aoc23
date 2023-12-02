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
    let games: Vec<Game> = collect_lines(input)?;
    let part1 = games
        .iter()
        .filter_map(|game| game.is_possible(&CUBE_LIMITS).then_some(game.id))
        .sum();

    let part2 = games.iter().map(Game::minimal_power).sum();

    Ok((part1, part2))
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
    reds: usize,
    greens: usize,
    blues: usize,
}

impl_fromstr_matching!(
    delim: ',',
    Cubes {
        reds: "([0-9]+) red",
        greens: "([0-9]+) green",
        blues: "([0-9]+) blue",
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
                reds: l.reds.max(r.reds),
                greens: l.greens.max(r.greens),
                blues: l.blues.max(r.blues),
            })
            .power()
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
