use std::collections::HashSet;
use std::str::FromStr;

use aoc23::*;
use itertools::Itertools;

main!(13, 30);
type Input = Vec<Card>;

fn parse(input: &str) -> Result<Input> {
    collect_lines(input)
}

fn part1(cards: &[Card]) -> Result<usize> {
    Ok(cards.iter().map(|c| c.numbers.score()).sum())
}

fn part2(cards: &[Card]) -> Result<usize> {
    let mut cards_counts = cards.iter().map(|_| 1).collect_vec();
    for (i, card) in cards.iter().enumerate() {
        let winning_numbers = card.numbers.count_winning();
        for next in (card.id)..(card.id + winning_numbers) {
            cards_counts[next] += cards_counts[i]
        }
    }
    Ok(cards_counts.iter().sum())
}

#[apply(parse_ordered!)]
#[delim(':')]
#[derive(Debug, Default, Clone)]
struct Card {
    #[parse(re("Card +([0-9]+)"))]
    id: usize,
    #[parse()]
    numbers: Numbers,
}

#[apply(parse_ordered!)]
#[delim('|')]
#[derive(Debug, Default, Clone)]
struct Numbers {
    #[parse(collect(' '))]
    winning: HashSet<isize>,
    #[parse(collect(' '))]
    present: HashSet<isize>,
}

impl Numbers {
    fn score(&self) -> usize {
        let count = self.count_winning();
        if count == 0 {
            0
        } else {
            2usize.pow(count as u32 - 1)
        }
    }

    fn count_winning(&self) -> usize {
        self.winning.intersection(&self.present).count()
    }
}
