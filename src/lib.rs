use std::fs::File;
use std::io::{prelude::*, BufReader};

pub type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn read_lines(path: &str) -> MyResult<impl Iterator<Item = String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(Result::unwrap))
}

#[macro_export]
macro_rules! main {
    ($func:ident, $path:literal) => {
        fn main() -> aoc23::MyResult<()> {
            let (part1, part2) = $func(include_str!($path))?;
            println!("part1: {}", part1);
            println!("part2: {}", part2);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! test_with_example {
    ($func:ident, $path:literal, $expected:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn example() -> aoc23::MyResult<()> {
                let res = $func(include_str!($path))?;
                assert_eq!(res, $expected);
                Ok(())
            }
        }
    };
    ($func:ident, $path:literal, $part1_expected:expr, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn example() -> aoc23::MyResult<()> {
                let (part1, part2) = $func(include_str!($path))?;
                assert_eq!(part1, $part1_expected);
                assert_eq!(part2, $part2_expected);
                Ok(())
            }
        }
    };
    ($func1:ident, $path1:literal, $part1_expected:expr, $func2:ident, $path2:literal, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn example1() -> aoc23::MyResult<()> {
                let part1 = $func1(include_str!($path1))?;
                assert_eq!(part1, $part1_expected);
                Ok(())
            }
            #[test]
            fn example2() -> aoc23::MyResult<()> {
                let part2 = $func2(include_str!($path2))?;
                assert_eq!(part2, $part2_expected);
                Ok(())
            }
        }
    };
}
