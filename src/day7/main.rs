use aoc23::*;
use itertools::Itertools;

main!(6440, 5905);
type Input = Vec<Hand>;

#[apply(parse_ordered!)]
#[delim(' ')]
#[derive(Debug, Default, Clone)]
struct Hand {
    #[parse()]
    hand: String,
    #[parse()]
    bid: usize,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse(input: &str) -> Result<Input> {
    collect_lines(input)
}

fn part(hands: &Input, joker: u8) -> Result<usize> {
    Ok(hands
        .iter()
        .sorted_by_cached_key(|hand| hand.compare_key(joker))
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .sum())
}

fn part1(hands: &Input) -> Result<usize> {
    part(hands, b' ')
}

fn part2(hands: &Input) -> Result<usize> {
    part(hands, b'J')
}

impl Hand {
    fn compare_key(&self, joker: u8) -> (Type, (u8, u8, u8, u8, u8)) {
        (
            self.hand_type(joker),
            self.hand
                .bytes()
                .map(|b| Self::card_rank(b, joker))
                .next_tuple()
                .unwrap(),
        )
    }

    fn hand_type(&self, joker: u8) -> Type {
        let joker_count = self.hand.bytes().filter(|b| *b == joker).count();
        let mut counts = self
            .hand
            .bytes()
            .filter(|b| *b != joker)
            .sorted()
            .group_by(|&b| b)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev();
        match (
            counts.next().or(Some(0)).map(|n| n + joker_count),
            counts.next(),
        ) {
            (Some(5), _) => Type::FiveOfAKind,
            (Some(4), _) => Type::FourOfAKind,
            (Some(3), Some(2)) => Type::FullHouse,
            (Some(3), _) => Type::ThreeOfAKind,
            (Some(2), Some(2)) => Type::TwoPair,
            (Some(2), _) => Type::OnePair,
            _ => Type::HighCard,
        }
    }

    fn card_rank(b: u8, joker: u8) -> u8 {
        match b {
            n if n == joker => 0,
            b'T' => 10,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            n => n - b'0',
        }
    }
}
