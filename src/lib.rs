pub use anyhow::Result;
use thiserror::Error;

#[macro_export]
macro_rules! main {
    ($func:ident, $path:literal) => {
        fn main() -> anyhow::Result<()> {
            let (part1, part2) = $func(include_str!($path))?;
            println!("part1: {}", part1);
            println!("part2: {}", part2);
            Ok(())
        }
    };
    ($func1:ident, $func2:ident, $path:literal) => {
        fn main() -> anyhow::Result<()> {
            let input = include_str!($path);
            let part1 = $func1(input)?;
            println!("part1: {}", part1);
            let part2 = $func2(input)?;
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
            fn example() -> anyhow::Result<()> {
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
            fn example() -> anyhow::Result<()> {
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
            fn example1() -> anyhow::Result<()> {
                let part1 = $func1(include_str!($path1))?;
                assert_eq!(part1, $part1_expected);
                Ok(())
            }
            #[test]
            fn example2() -> anyhow::Result<()> {
                let part2 = $func2(include_str!($path2))?;
                assert_eq!(part2, $part2_expected);
                Ok(())
            }
        }
    };
}

#[derive(Error, Debug)]
#[error("Failed to parse '{text}': {err}")]
pub struct ParseError {
    text: String,
    err: String,
}

pub fn parse_error(text: &str, err: &str) -> ParseError {
    ParseError {
        text: String::from(text),
        err: String::from(err),
    }
}
