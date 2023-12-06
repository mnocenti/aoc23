use std::{ops::Range, str::FromStr};

use anyhow::anyhow;
use aoc23::*;
use itertools::Itertools;

main!(35, 46);

#[cfg(windows)]
const LINE_FEED: &str = "\r\n";
#[cfg(not(windows))]
const LINE_FEED: &str = "\n";

fn parse(input: &str) -> Result<Almanac> {
    input.parse()
}

fn part1(almanac: &Almanac) -> Result<usize> {
    almanac
        .seeds
        .iter()
        .map(|&seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |acc, mapping| mapping.map(acc))
        })
        .min()
        .ok_or(anyhow!("no seeds ???"))
}

fn part2(almanac: &Almanac) -> Result<usize> {
    let seeds = almanac
        .seeds
        .iter()
        .tuples()
        .map(|(start, length)| *start..(*start + *length))
        .collect_vec();

    Ok(almanac
        .maps
        .iter()
        .fold(seeds, |acc, mapping| mapping.map_range(acc))
        .iter()
        .min_by_key(|r| r.start)
        .ok_or(anyhow!("no seeds ???"))?
        .start)
}

#[derive(Debug, Default, Clone)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Mapping>,
}

#[apply(parse_ordered!)]
#[delim(LINE_FEED)]
#[derive(Debug, Default, Clone)]
struct Mapping {
    #[parse()]
    name: String,
    #[parse(collect(remaining))]
    ranges: Vec<MappedRange>,
}

#[derive(Debug, Default, Clone)]
struct MappedRange {
    source: Range<usize>,
    dest: Range<usize>,
}

impl Mapping {
    fn map(&self, source: usize) -> usize {
        match self
            .ranges
            .binary_search_by_key(&source, |r| r.source.start)
        {
            Ok(idx) => self.ranges[idx].dest.start, // match start of range
            Err(0) => source,                       // lower than first mapped element
            Err(idx) => {
                let r = &self.ranges[idx - 1];
                if r.source.end >= source {
                    source + r.dest.start - r.source.start
                } else {
                    source
                }
            }
        }
    }
    fn map_range(&self, sources: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut res = Vec::with_capacity(sources.len());
        for source in sources {
            // find first intersting destination range
            let mut idx = match self
                .ranges
                .binary_search_by_key(&source.start, |r| r.source.start)
            {
                Ok(idx) => idx,      // match start of range
                Err(0) => 0,         // lower than first mapped element
                Err(idx) => idx - 1, // maybe in range idx-1
            };
            let mut remaining = source.end - source.start;
            while remaining > 0 {
                let source_start = source.end - remaining;
                let in_current_range = self.ranges.get(idx).and_then(|r| {
                    (r.source.start <= source_start && r.source.end > source_start).then_some(r)
                });
                if let Some(r) = in_current_range {
                    // the start of the remaining range is inside the dest range
                    let first_dest = source_start + r.dest.start - r.source.start;
                    let len = (r.dest.end - first_dest).min(remaining);
                    res.push(first_dest..(first_dest + len));
                    remaining -= len;
                } else if let Some(r) = self.ranges.get(idx + 1) {
                    // the start of the remaining range is lower than the next range.
                    // Map the region not matching any range to itself.
                    let len = (r.dest.start - source_start).min(remaining);
                    res.push(source_start..(source_start + len));
                    remaining -= len;
                } else {
                    // No more dest ranges, map all remaining sources to themselves
                    res.push(source_start..source.end);
                    remaining = 0;
                }
                idx += 1;
            }
        }
        res
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps_delim = String::from(LINE_FEED) + LINE_FEED;
        let (seeds, maps) = s.split_once(&maps_delim).unwrap();
        let mut almanac = Almanac {
            seeds: parse_collect(seeds.split_once(':').unwrap().1, ' ')?,
            maps: parse_collect_str(maps, &maps_delim)?,
        };
        for m in almanac.maps.iter_mut() {
            m.ranges.sort_by_key(|r| r.source.start)
        }
        Ok(almanac)
    }
}

impl FromStr for MappedRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ');
        let (dest, source, length) = split
            .map(|s| s.parse().expect("expected integer for MappedRange"))
            .next_tuple()
            .ok_or(parse_error(s, "expected 3 values"))?;
        Ok(MappedRange {
            source: source..(source + length),
            dest: dest..(dest + length),
        })
    }
}
