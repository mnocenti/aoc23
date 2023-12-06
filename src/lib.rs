use std::str::FromStr;

pub use anyhow::Result;
use clap::{Parser, ValueEnum};
pub use macro_rules_attribute::apply;
use thiserror::Error;

pub mod grid;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    part: Option<Part>,
}

pub fn get_cli_args() -> Args {
    Args::parse()
}

pub fn aoc_main(
    args: Args,
    part1: impl Fn() -> Result<()>,
    part2: impl Fn() -> Result<()>,
) -> Result<()> {
    match args.part {
        Some(Part::Part1) => part1(),
        Some(Part::Part2) => part2(),
        None => {
            part1()?;
            part2()
        }
    }
}

#[macro_export]
macro_rules! main {
    () => {
        fn main() -> anyhow::Result<()> {
            let args = aoc23::get_cli_args();
            let parsed = parse(include_str!("input.txt"))?;
            aoc_main(
                args,
                || {
                    let p1 = part1(&parsed)?;
                    println!("part1: {}", p1);
                    Ok(())
                },
                || {
                    let p2 = part2(&parsed)?;
                    println!("part2: {}", p2);
                    Ok(())
                },
            )
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
    s.split(delim)
        .filter(|s| !s.is_empty())
        .map(|st| st.parse())
        .collect()
}

pub fn parse_collect_str<Item, T: FromIterator<Item>>(
    s: &str,
    delim: &str,
) -> Result<T, <Item as FromStr>::Err>
where
    Item: FromStr,
{
    s.split(delim)
        .filter(|s| !s.is_empty())
        .map(|st| st.parse())
        .collect()
}

pub fn collect_lines<Item, T: FromIterator<Item>>(s: &str) -> Result<T, <Item as FromStr>::Err>
where
    Item: FromStr,
{
    s.lines().map(|st| st.parse()).collect()
}

#[macro_export]
macro_rules! set_field_ordered {
    ($t:ident, $split:ident, $member:ident ()) => {{
        let s = $split.next().expect("not enough delimiters");
        $t.$member = s.parse()?;
    }};
    ($t:ident, $split:ident, $member:ident (collect(remaining))) => {{
        $t.$member = $split.map(|s| s.parse()).try_collect()?;
    }};
    ($t:ident, $split:ident, $member:ident (collect($delim:literal))) => {{
        let s = $split.next().expect("not enough delimiters");
        $t.$member = aoc23::parse_collect(s, $delim)?;
    }};
    ($t:ident, $split:ident, $member:ident (re($regex:literal))) => {{
        let s = $split.next().expect("not enough delimiters");
        static RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($regex).unwrap());
        if let Some(cap) = RE.captures(s) {
            $t.$member = cap.get(1).unwrap().as_str().parse()?;
        } else {
            return Err(aoc23::parse_error(
                s,
                &format!("failed to match regex {}", $regex),
            ))?;
        }
    }};
}

#[macro_export]
macro_rules! parse_ordered {(
    #[delim($delim:expr)]
    $(#[$struct_meta:meta])*
    $struct_vis:vis
    struct $StructName:ident {
        $(
            #[parse $field_parser:tt]
            $(#[$field_meta:meta])*
            $field_vis:vis
            $field_name:ident : $field_ty:ty
        ),* $(,)?
    }
) => (
    // Generate the struct definition we have been given
    $(#[$struct_meta])*
    $struct_vis
    struct $StructName {
        $(
            $(#[$field_meta])*
            $field_vis $field_name: $field_ty,
        )*
    }
    // Generate an implementation of FromStr
    impl FromStr for $StructName {
        type Err = anyhow::Error;

        fn from_str(string: &str) -> Result<Self> {
            let mut t = <$StructName>::default();
            let mut split = string.split($delim);
            $({
                set_field_ordered!(t, split, $field_name $field_parser);
            })*
            Ok(t)
        }
    }
)}

#[macro_export]
macro_rules! set_matching_field {
    ($t:ident, $s:ident, $member:ident ($regex:literal)) => {{
        static RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($regex).unwrap());
        if let Some(cap) = RE.captures($s) {
            $t.$member = cap.get(1).unwrap().as_str().parse()?;
            return Ok($t);
        }
    }};
}

#[macro_export]
macro_rules! parse_matching {(
    #[delim($delim:literal)]
    $(#[$struct_meta:meta])*
    $struct_vis:vis
    struct $StructName:ident {
        $(
            #[parse $field_parser:tt]
            $(#[$field_meta:meta])*
            $field_vis:vis // this visibility will be applied to the getters instead
            $field_name:ident : $field_ty:ty
        ),* $(,)?
    }
) => (
    // First, generate the struct definition we have been given
    $(#[$struct_meta])*
    $struct_vis
    struct $StructName {
        $(
            $(#[$field_meta])*
            $field_vis $field_name: $field_ty,
        )*
    }
    impl FromStr for $StructName {
        type Err = anyhow::Error;

        fn from_str(string: &str) -> Result<Self> {
            string.split($delim).try_fold(<$StructName>::default(), |mut t, s| {
                $(
                    set_matching_field!(t, s, $field_name $field_parser);
                )*
                #[allow(unreachable_code)]
                Err(aoc23::parse_error(string, ""))?
            })
        }
    }
)}
