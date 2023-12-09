use aoc23::*;
use itertools::Itertools;

main!(114, 2);

type Input = Vec<Vec<isize>>;

fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse().expect("expected integer"))
                .collect()
        })
        .collect())
}

fn part1(report: &Input) -> Result<isize> {
    Ok(report.iter().map(|history| extrapolate(history)).sum())
}

fn part2(report: &Input) -> Result<isize> {
    Ok(report.iter().map(|history| extrapolate_back(history)).sum())
}

fn extrapolate(history: &[isize]) -> isize {
    let diffs = get_diffs(history);
    let diff_sum = diffs.iter().filter_map(|l| l.last()).sum();
    diff_sum
}

fn extrapolate_back(history: &[isize]) -> isize {
    let diffs = get_diffs(history);
    let diff_sum: isize = diffs.iter().rev().fold(0, |acc, list| list[0] - acc);
    diff_sum
}

fn get_diffs(history: &[isize]) -> Vec<Vec<isize>> {
    let mut diffs: Vec<Vec<isize>> = Vec::new();
    diffs.push(Vec::from(history));
    let mut last = diffs.last().unwrap();
    while !last.iter().all_equal() {
        diffs.push(last.iter().tuple_windows().map(|(a, b)| b - a).collect());
        last = diffs.last().unwrap();
    }
    diffs
}
