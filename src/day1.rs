use aoc23::*;
main!(day1_1, day1_2, "../inputs/input1.txt");

test_with_example!(
    day1_1,
    "../inputs/example1_1.txt",
    142,
    day1_2,
    "../inputs/example1_2.txt",
    281
);

fn day1_1(input: &str) -> Result<u32> {
    let digit_at = |s: &str, i| s.chars().nth(i)?.to_digit(10);
    Ok(input
        .lines()
        .filter_map(|s: &str| {
            let first = s.find(|c: char| c.is_ascii_digit())?;
            let last = s.rfind(|c: char| c.is_ascii_digit())?;
            Some(10 * digit_at(s, first)? + digit_at(s, last)?)
        })
        .sum())
}

fn day1_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .filter_map(|s| Some(10 * first_digit(s)? + last_digit(s)?))
        .sum())
}

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_digit(s: &str) -> Option<usize> {
    search_digit(|pattern: &str| s.find(pattern), true)
}

fn last_digit(s: &str) -> Option<usize> {
    search_digit(|pattern: &str| s.rfind(pattern), false)
}

fn search_digit(search: impl Fn(&str) -> Option<usize>, is_first: bool) -> Option<usize> {
    let digits_pos = DIGITS
        .iter()
        .enumerate()
        .chain(SPELLED_DIGITS.iter().enumerate())
        .filter_map(|(i, word)| Some((search(word)?, i + 1)));
    if is_first {
        digits_pos.min_by_key(|(pos, _)| *pos).map(|(_, i)| i)
    } else {
        digits_pos.max_by_key(|(pos, _)| *pos).map(|(_, i)| i)
    }
}
