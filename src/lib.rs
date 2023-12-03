use std::str::FromStr;

pub use anyhow::Result;
use thiserror::Error;

pub mod grid;

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
            #[test]
            fn $func() -> anyhow::Result<()> {
                let res = super::$func(include_str!($path))?;
                assert_eq!(res, $expected);
                Ok(())
            }
        }
    };
    ($func:ident, $path:literal, $part1_expected:expr, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn $func() -> anyhow::Result<()> {
                let (part1, part2) = super::$func(include_str!($path))?;
                assert_eq!(part1, $part1_expected);
                assert_eq!(part2, $part2_expected);
                Ok(())
            }
        }
    };
    ($func1:ident, $path1:literal, $part1_expected:expr, $func2:ident, $path2:literal, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn $func1() -> anyhow::Result<()> {
                let part1 = super::$func1(include_str!($path1))?;
                assert_eq!(part1, $part1_expected);
                Ok(())
            }
            #[test]
            fn $func2() -> anyhow::Result<()> {
                let part2 = super::$func2(include_str!($path2))?;
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
