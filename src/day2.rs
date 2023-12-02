use std::str::FromStr;

aoc23::main!(day2, "../inputs/input2.txt");

aoc23::test_with_example!(day2, "../inputs/example2.txt", 8, 2286);

#[derive(Default, Debug)]
struct Handful {
    reds: usize,
    greens: usize,
    blues: usize,
}

impl Handful {
    fn possible(&self, game_limits: &Handful) -> bool {
        self.reds <= game_limits.reds
            && self.greens <= game_limits.greens
            && self.blues <= game_limits.blues
    }
    fn power(&self) -> usize {
        self.reds * self.greens * self.blues
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    cubes: Vec<Handful>,
}

impl Game {
    fn minimal_power(&self) -> usize {
        self.cubes
            .iter()
            .fold(Handful::default(), |l, r| Handful {
                reds: l.reds.max(r.reds),
                greens: l.greens.max(r.greens),
                blues: l.blues.max(r.blues),
            })
            .power()
    }
}

fn day2(input: &str) -> aoc23::MyResult<(usize, usize)> {
    const CUBE_LIMITS: Handful = Handful {
        reds: 12,
        greens: 13,
        blues: 14,
    };
    let games = parse_games(input)?;
    let part1 = games
        .iter()
        .filter_map(|game| {
            game.cubes
                .iter()
                .all(|handful| handful.possible(&CUBE_LIMITS))
                .then_some(game.id)
        })
        .sum();

    let part2 = games.iter().map(Game::minimal_power).sum();

    Ok((part1, part2))
}

fn parse_error(string: &str, err: &str) -> String {
    format!("Failed to parse '{}': {}", string, err)
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_error = |string, err| format!("Failed to parse {}: {}", string, err);
        let (start, rest) = s.split_once(':').ok_or(parse_error(s, "missing ':'"))?;
        let id = start
            .split_once(' ')
            .ok_or(parse_error(s, "missing ' '"))?
            .1
            .parse()?;
        let cubes: Result<_, Self::Err> = rest.split(';').map(Handful::from_str).collect();
        Ok(Game { id, cubes: cubes? })
    }
}
impl FromStr for Handful {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',').try_fold(Handful::default(), |mut handful, s| {
            Ok(match s.trim().split_once(' ').ok_or("missing ' '")? {
                (n, "red") => {
                    handful.reds += n.parse::<usize>()?;
                    handful
                }
                (n, "green") => {
                    handful.greens += n.parse::<usize>()?;
                    handful
                }
                (n, "blue") => {
                    handful.blues += n.parse::<usize>()?;
                    handful
                }
                _ => Err(parse_error(s, ""))?,
            })
        })
    }
}

fn parse_games(input: &str) -> aoc23::MyResult<Vec<Game>> {
    input.lines().map(Game::from_str).collect()
}
