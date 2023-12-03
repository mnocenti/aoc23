use std::str::FromStr;

pub use anyhow::Result;
use thiserror::Error;

pub mod grid;

#[macro_export]
macro_rules! main {
    () => {
        fn main() -> anyhow::Result<()> {
            let parsed = parse(include_str!("input.txt"))?;
            let p1 = part1(&parsed)?;
            println!("part1: {}", p1);
            let p2 = part2(&parsed)?;
            println!("part2: {}", p2);
            Ok(())
        }
    };
    ($part1_expected:expr, $part2_expected:expr) => {
        main!();
        test_with_example!($part1_expected, $part2_expected);
    };
    ($path1:literal, $part1_expected:expr, $path2:literal, $part2_expected:expr) => {
        main!();
        test_with_example!($path1, $part1_expected, $path2, $part2_expected);
    };
}

#[macro_export]
macro_rules! test_with_example {
    ($part1_expected:expr, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn part1() -> anyhow::Result<()> {
                let parsed = super::parse(include_str!("example.txt"))?;
                let res = super::part1(&parsed)?;
                assert_eq!(res, $part1_expected);
                Ok(())
            }
            #[test]
            fn part2() -> anyhow::Result<()> {
                let parsed = super::parse(include_str!("example.txt"))?;
                let res = super::part2(&parsed)?;
                assert_eq!(res, $part2_expected);
                Ok(())
            }
        }
    };
    ($path1:literal, $part1_expected:expr, $path2:literal, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn part1() -> anyhow::Result<()> {
                let parsed = super::parse(include_str!($path1))?;
                let part1 = super::part1(&parsed)?;
                assert_eq!(part1, $part1_expected);
                Ok(())
            }
            #[test]
            fn part2() -> anyhow::Result<()> {
                let parsed = super::parse(include_str!($path2))?;
                let part2 = super::part2(&parsed)?;
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

pub fn parse_collect<Item, T: FromIterator<Item>>(
    s: &str,
    delim: char,
) -> Result<T, <Item as FromStr>::Err>
where
    Item: FromStr,
{
    s.split(delim).map(|st| st.parse()).collect()
}

pub fn collect_lines<Item, T: FromIterator<Item>>(s: &str) -> Result<T, <Item as FromStr>::Err>
where
    Item: FromStr,
{
    s.lines().map(|st| st.parse()).collect()
}

#[macro_export]
macro_rules! set_matching_member {
    ($t:ident, $s:ident, $member:ident : $regex:literal) => {{
        static RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($regex).unwrap());
        if let Some(cap) = RE.captures($s) {
            $t.$member = cap.get(1).unwrap().as_str().parse()?;
            return Ok($t);
        }
    }};
}
#[macro_export]
macro_rules! parse_matching {
    ($string:ident, $delim:literal, $T:ty { $($member:ident : $regex:tt),+ }) => {
        $string.split($delim).try_fold(<$T>::default(), |mut t, s| {
            $(
                set_matching_member!(t, s, $member : $regex);
            )*
            #[allow(unreachable_code)]
            Err(aoc23::parse_error($string, ""))?
        })
    };
}
#[macro_export]
macro_rules! impl_fromstr_matching {
    (delim : $delim:literal, $T:ty { $($member:ident : $regex:tt),+ $(,)?}) => {
        impl FromStr for $T {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self> {
                parse_matching!(s, $delim, $T { $($member : $regex),* })
            }
        }
    }
}

#[macro_export]
macro_rules! set_member_ordered {
    ($t:ident, $s:ident, $member:ident : { collect $delim:literal}) => {{
        $t.$member = aoc23::parse_collect($s, $delim)?;
    }};
    ($t:ident, $s:ident, $member:ident : $regex:literal) => {{
        static RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($regex).unwrap());
        if let Some(cap) = RE.captures($s) {
            $t.$member = cap.get(1).unwrap().as_str().parse()?;
        } else {
            return Err(aoc23::parse_error(
                $s,
                &format!("failed to match regex {}", $regex),
            ))?;
        }
    }};
}
#[macro_export]
macro_rules! parse_ordered {
    ($string:ident, $delim:literal, $T:ty { $($member:ident : $regex:tt),+ }) => {{
        let mut t = <$T>::default();
        let mut split = $string.split($delim);
        $({
            let s = split.next().unwrap();
            set_member_ordered!(t, s, $member : $regex);
        })*
        if split.next().is_some() {
            Err(aoc23::parse_error($string, "too much delimiters"))?
        }
        Ok(t)
    }};
}
#[macro_export]
macro_rules! impl_fromstr_ordered {
    (delim : $delim:literal, $T:ty { $($member:ident : $regex:tt),+ $(,)?}) => {
        impl FromStr for $T {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self> {
                parse_ordered!(s, $delim, $T { $($member : $regex),* })
            }
        }
    }
}
