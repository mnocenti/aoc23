
aoc23::main!(day1, "../inputs/input1.txt");

aoc23::test_with_example!(day1_2, "../inputs/example1.txt", 281);

//aoc23::test_with_example!(day1, "../inputs/example1.txt", 13, 140);
const DIGITS : [(usize, &str); 9] = [(1, "one"), (2,"two"), (3,"three"), (4,"four"), (5,"five"), (6,"six"), (7,"seven"), (8,"eight"), (9,"nine")];

fn find_first_digit(s : &str) -> Option<usize> {
    DIGITS.iter()
        .filter_map(|(i,w)| Some((s.find(&i.to_string())?.min(s.find(w)?), i)))
        .min_by(|a,b| a.0.cmp(&b.0)).map(|(_,i)| *i)
}
fn find_last_digit(s : &str) -> Option<usize> {
    DIGITS.iter()
        .filter_map(|(i,w)| Some((s.rfind(&i.to_string())?.max(s.rfind(w)?), i)))
        .max_by(|a,b| a.0.cmp(&b.0)).map(|(_,i)| *i)
}

fn day1(input: &str) -> aoc23::MyResult<(usize, usize)> {
    let part1 = input
        .lines()
        .filter(|s| !s.is_empty()).map(|s: &str| { let mut s = String::from(s); s.retain(|c| c.is_ascii_digit()); (String::from(s.chars().next().unwrap()) + &String::from(s.chars().last().unwrap()) ).parse::<usize>().unwrap() }).sum();
    
    Ok((part1,day1_2(input)?))
}

fn day1_2(input: &str) -> aoc23::MyResult<usize> {
    Ok(input.lines().filter_map(|s| Some(10*find_first_digit(s)? + find_last_digit(s)?)).sum())
}