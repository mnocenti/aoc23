use std::{collections::HashMap, ops::Range};

use anyhow::anyhow;
use aoc23::*;
use itertools::Itertools;

main!(19114, 167409079868000);

#[cfg(windows)]
const TWO_LINES: &str = "\r\n\r\n";
#[cfg(not(windows))]
const TWO_LINES: &str = "\n\n";

#[apply(parse_ordered!)]
#[delim(TWO_LINES)]
#[derive(Default)]
struct XMASSystem {
    #[parse()]
    workflows: Workflows,
    #[parse(collect(lines))]
    parts: Vec<Part>,
}

#[derive(Default)]
struct Workflows(HashMap<String, Vec<Rule>>);

impl FromStr for Workflows {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Workflows(
            s.lines()
                .map(|line| -> Result<_> {
                    let (name, rules) = line
                        .split_once('{')
                        .ok_or(anyhow!("workflow parse error"))?;
                    Ok((
                        String::from(name),
                        parse_collect(&rules[0..rules.len() - 1], ',')?,
                    ))
                })
                .try_collect()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    LessThan,
    GreaterThan,
}

struct Rule {
    which: u8,
    comp: Comparison,
    threshold: usize,
    dest: String,
}
impl Rule {
    fn check(&self, part: &Part) -> bool {
        let value = match self.which {
            b'x' => part.x,
            b'm' => part.m,
            b'a' => part.a,
            b's' => part.s,
            _ => panic!("unknown part category"),
        };
        match self.comp {
            Comparison::LessThan => value < self.threshold,
            Comparison::GreaterThan => value > self.threshold,
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((check, dest)) = s.split_once(':') {
            let threshold: usize = check[2..].parse()?;
            let comparison = check.as_bytes()[1];
            Ok(Rule {
                which: check.as_bytes()[0],
                comp: if comparison == b'<' {
                    Comparison::LessThan
                } else {
                    Comparison::GreaterThan
                },
                threshold,
                dest: String::from(dest),
            })
        } else {
            // x > 0 always true
            Ok(Rule {
                which: b'x',
                comp: Comparison::GreaterThan,
                threshold: 0,
                dest: String::from(s),
            })
        }
    }
}

#[apply(parse_ordered!)]
#[delim(',')]
#[derive(Default)]
struct Part {
    #[parse(re("\\{x=([0-9]+)"))]
    x: usize,
    #[parse(re("m=([0-9]+)"))]
    m: usize,
    #[parse(re("a=([0-9]+)"))]
    a: usize,
    #[parse(re("s=([0-9]+)}"))]
    s: usize,
}

impl Part {
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse(input: &str) -> Result<XMASSystem> {
    input.parse()
}

fn part1(system: &XMASSystem) -> Result<usize> {
    Ok(system
        .parts
        .iter()
        .filter(|part| validate(part, &system.workflows.0))
        .map(Part::rating)
        .sum())
}

fn validate(part: &Part, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut next = "in";
    while !["A", "R"].contains(&next) {
        let rules = &workflows[next];
        next = &rules
            .iter()
            .find(|r| r.check(part))
            .expect("no rule matches ?")
            .dest;
    }
    next == "A"
}

type XMASRange = (Range<usize>, Range<usize>, Range<usize>, Range<usize>);
const FULL_RANGE: Range<usize> = 1..4001;

fn part2(system: &XMASSystem) -> Result<usize> {
    let all_possible_parts = (FULL_RANGE, FULL_RANGE, FULL_RANGE, FULL_RANGE);
    let workflows = &system.workflows.0;
    Ok(count_accepted(all_possible_parts, workflows, "in"))
}

fn count_accepted(
    mut range: XMASRange,
    workflows: &HashMap<String, Vec<Rule>>,
    next: &str,
) -> usize {
    let mut count = 0;
    for rule in &workflows[next] {
        let (included_range, remaining_range) = rule.check_range(&range);
        if !range_is_empty(&included_range) {
            if rule.dest == "A" {
                count += count_combinations(&included_range);
            } else if rule.dest != "R" {
                count += count_accepted(included_range, workflows, &rule.dest);
            }
        }
        range = remaining_range;
        if range_is_empty(&range) {
            break;
        }
    }
    count
}

fn range_is_empty(range: &XMASRange) -> bool {
    range.0.is_empty() || range.1.is_empty() || range.2.is_empty() || range.3.is_empty()
}

fn count_combinations(range: &XMASRange) -> usize {
    range.0.len() * range.1.len() * range.2.len() * range.3.len()
}

impl Rule {
    fn check_range(&self, range: &XMASRange) -> (XMASRange, XMASRange) {
        let mut included_range = range.clone();
        let mut excluded_range = range.clone();
        let (range_to_include, range_to_exclude) = match self.which {
            b'x' => (&mut included_range.0, &mut excluded_range.0),
            b'm' => (&mut included_range.1, &mut excluded_range.1),
            b'a' => (&mut included_range.2, &mut excluded_range.2),
            b's' => (&mut included_range.3, &mut excluded_range.3),
            _ => panic!("unknown part category"),
        };
        match self.comp {
            Comparison::LessThan => {
                *range_to_include =
                    range_to_include.start..self.threshold.min(range_to_include.end);
                *range_to_exclude =
                    (self.threshold.max(range_to_exclude.start))..range_to_exclude.end;
            }
            Comparison::GreaterThan => {
                *range_to_include =
                    ((self.threshold + 1).max(range_to_include.start))..range_to_include.end;
                *range_to_exclude =
                    range_to_exclude.start..((self.threshold + 1).min(range_to_exclude.end));
            }
        }
        (included_range, excluded_range)
    }
}
