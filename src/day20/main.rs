use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fmt::Display,
    thread::sleep,
    time::Duration,
};

use anyhow::anyhow;
use aoc23::*;
use colored::{ColoredString, Colorize};
use crossterm_cursor::cursor;
use itertools::Itertools;

main!();

const DISPLAY: bool = false;

type ModuleConf<'a> = HashMap<&'a str, Module<'a>>;

fn parse(input: &str) -> Result<ModuleConf> {
    let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut module_conf: ModuleConf = input
        .lines()
        .map(|l| -> Result<_> {
            let (name, dest_list) = l.split_once("->").ok_or(anyhow!("dests parse error"))?;
            let name = name.trim_start_matches(['%', '&']).trim_end();
            let dests = dest_list.split(',').map(|name| name.trim()).collect_vec();
            for d in &dests {
                match module_inputs.entry(d) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(name);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![name]);
                    }
                }
            }
            let module = match l.chars().next() {
                Some('%') => Module::FlipFlop(FlipFlop {
                    name,
                    mem: false,
                    dests,
                }),
                Some('&') => Module::Conjunction(Conjunction {
                    name,
                    mem: Default::default(),
                    dests,
                }),
                _ => Module::Broadcast(Broadcast { name, dests }),
            };
            Ok((name, module))
        })
        .try_collect()?;
    for (module, inputs) in module_inputs {
        if let Some(Module::Conjunction(conj)) = module_conf.get_mut(module) {
            conj.mem = inputs.into_iter().map(|n| (n, false)).collect()
        }
    }
    Ok(module_conf)
}

#[derive(Debug, Clone)]
struct FlipFlop<'a> {
    name: &'a str,
    mem: bool,
    dests: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Conjunction<'a> {
    name: &'a str,
    mem: HashMap<&'a str, bool>,
    dests: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Broadcast<'a> {
    name: &'a str,
    dests: Vec<&'a str>,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcast(Broadcast<'a>),
}

type SignalQueue<'a> = VecDeque<(&'a str, &'a str, bool)>;

impl<'a> Module<'a> {
    pub fn signal(&mut self, source: &'a str, pulse: bool, signals: &mut SignalQueue<'a>) {
        match self {
            Module::FlipFlop(flipflop) => {
                flipflop.signal(pulse, signals);
            }
            Module::Conjunction(conjunction) => {
                conjunction.signal(source, pulse, signals);
            }
            Module::Broadcast(broadcast) => {
                broadcast.signal(pulse, signals);
            }
        }
    }

    fn is_flipflop(&self) -> bool {
        matches!(self, Module::FlipFlop(_))
    }

    fn is_conjuction(&self) -> bool {
        matches!(self, Module::Conjunction(_))
    }

    fn dests(&self) -> &Vec<&'a str> {
        match self {
            Module::FlipFlop(flipflop) => &flipflop.dests,
            Module::Conjunction(conjunction) => &conjunction.dests,
            Module::Broadcast(broadcast) => &broadcast.dests,
        }
    }

    fn name(&'a self) -> &'a str {
        match self {
            Module::FlipFlop(flipflop) => flipflop.name,
            Module::Conjunction(conjunction) => conjunction.name,
            Module::Broadcast(broadcast) => broadcast.name,
        }
    }

    fn is_high(&self) -> bool {
        match self {
            Module::FlipFlop(flipflop) => flipflop.mem,
            Module::Conjunction(conjunction) => conjunction.mem.values().all(|b| *b),
            Module::Broadcast(_) => false,
        }
    }
}

impl<'a> FlipFlop<'a> {
    fn signal(&mut self, pulse: bool, signals: &mut SignalQueue<'a>) {
        if !pulse {
            self.mem = !self.mem;
            Broadcast::broadcast(self.name, &self.dests, self.mem, signals);
        }
    }
}

impl<'a> Conjunction<'a> {
    fn signal(&mut self, source: &'a str, pulse: bool, signals: &mut SignalQueue<'a>) {
        self.mem.insert(source, pulse);
        Broadcast::broadcast(
            self.name,
            &self.dests,
            !self.mem.values().all(|b| *b),
            signals,
        );
    }
}
impl<'a> Broadcast<'a> {
    fn signal(&self, pulse: bool, signals: &mut SignalQueue<'a>) {
        Self::broadcast(self.name, &self.dests, pulse, signals);
    }
    fn broadcast(name: &'a str, dests: &[&'a str], pulse: bool, signals: &mut SignalQueue<'a>) {
        signals.extend(dests.iter().map(|d| (name, *d, pulse)));
    }
}

fn part1(module_conf: &ModuleConf) -> Result<usize> {
    let mut module_conf = module_conf.clone();
    let (mut high_count, mut low_count) = (0, 0);
    for _ in 0..1000 {
        let (high, low) = push_the_button(&mut module_conf);
        high_count += high;
        low_count += low;
    }
    Ok(high_count * low_count)
}

fn push_the_button(module_conf: &mut ModuleConf) -> (usize, usize) {
    let mut signals = VecDeque::new();
    signals.push_back(("button", "broadcaster", false));
    let (mut high_count, mut low_count) = (0, 0);
    while let Some((source, dest, pulse)) = signals.pop_front() {
        if pulse {
            high_count += 1
        } else {
            low_count += 1
        }
        if let Some(module) = module_conf.get_mut(dest) {
            module.signal(source, pulse, &mut signals);
        }
    }
    (high_count, low_count)
}

#[derive(Debug)]
struct Adder<'a> {
    bits: Vec<&'a str>,
    output: &'a str,
    inverter: &'a str,
}
impl<'a> Adder<'a> {
    // Get the maximum value that can be reached by the adder before it resets
    // (eg the value that triggers the output)
    fn max_val(&self, graph: &ModuleConf) -> usize {
        self.bits
            .iter()
            .rev()
            .map(|b| graph[b].dests().contains(&self.output) as usize)
            .reduce(|acc, b| 2 * acc + b)
            .unwrap()
    }
}

struct Process<'a> {
    graph: ModuleConf<'a>,
    adders: Vec<Adder<'a>>,
    synchronizer: &'a str,
    output: &'a str,
}

fn part2(graph: &ModuleConf) -> Result<usize> {
    // Find the "adders" : groups of flipflops than output to the same conjunction
    let adders: Vec<Adder> = graph["broadcaster"]
        .dests()
        .iter()
        .map(|first_bit| {
            (
                get_chained_flipflops(first_bit, graph),
                find_conjunction_dest(first_bit, graph).unwrap(),
            )
        })
        .map(|(bits, output)| Adder {
            bits,
            output,
            inverter: find_conjunction_dest(output, graph).unwrap(),
        })
        .collect();

    if DISPLAY {
        let synchronizer = find_conjunction_dest(adders[0].inverter, graph).unwrap();
        let mut process = Process {
            graph: graph.clone(),
            adders,
            synchronizer,
            output: "rx",
        };
        let mut cursor = cursor();
        println!("{process}");
        loop {
            sleep(Duration::from_millis(10));
            push_the_button(&mut process.graph);
            cursor.move_up(26)?;
            println!("{process}");
        }
    } else {
        Ok(adders
            .iter()
            .map(|adder| adder.max_val(graph))
            .reduce(num::integer::lcm)
            .unwrap())
    }
}

fn find_conjunction_dest<'a>(source: &str, graph: &'a ModuleConf) -> Option<&'a str> {
    graph[source]
        .dests()
        .iter()
        .copied()
        .find(|d| graph[*d].is_conjuction())
}
fn find_flipflop_dest<'a>(source: &'a Module, graph: &ModuleConf) -> Option<&'a str> {
    source
        .dests()
        .iter()
        .copied()
        .find(|d| graph[*d].is_flipflop())
}

fn get_chained_flipflops<'a>(first_bit: &str, graph: &'a ModuleConf) -> Vec<&'a str> {
    let mut res = Vec::new();
    let mut cur = &graph[first_bit];
    res.push(cur.name());
    while let Some(next) = find_flipflop_dest(cur, graph) {
        cur = &graph[next];
        res.push(cur.name());
    }
    res
}

impl<'a> Display for Process<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let color = |s: &str, module: &str| {
            if self.graph[module].is_high() {
                s.on_yellow()
            } else {
                s.on_bright_black()
            }
        };
        let connected = |source, dest| self.graph[source].dests().contains(&dest);
        let line_ending = |arrow| {
            format!(
                " {} {}   {}",
                arrow,
                color("  ", self.synchronizer),
                color("  ", self.synchronizer)
            )
        };
        let arrow_down_bit = "⇓".blue();
        let arrow_down_conj = "⇓".red();
        let arrow_up = "⇑".red();
        let arrow_left = "⇒".red();
        let no_arrow = ColoredString::from(" ");
        let check_connected = |arrow, source, dest| {
            if connected(source, dest) {
                arrow
            } else {
                &no_arrow
            }
        };
        for (i, adder) in self.adders.iter().enumerate() {
            for bit in &adder.bits {
                write!(f, " {} ", color(bit, bit))?;
            }
            writeln!(f, "{}", line_ending(" "))?;
            for bit in &adder.bits {
                let bit_to_out = check_connected(&arrow_down_bit, *bit, adder.output);
                let out_to_bit = check_connected(&arrow_up, adder.output, *bit);
                write!(f, " {}{} ", bit_to_out, out_to_bit)?;
            }
            writeln!(f, "{}", line_ending(" "))?;
            let adder_width = adder.bits.len() * 4;
            let padding: String = (0..(adder_width / 2 - 1)).map(|_| ' ').collect();
            write!(
                f,
                "{}",
                color(&format!("{padding}{}{padding}", adder.output), adder.output)
            )?;
            writeln!(f, "{}", line_ending(" "))?;
            let out_to_inv = check_connected(&arrow_down_conj, adder.output, adder.inverter);
            write!(f, "{}{}{}", padding, out_to_inv, padding)?;
            writeln!(f, " {}", line_ending(" "))?;
            write!(
                f,
                "{}",
                color(
                    &format!("{padding}{}{padding}", adder.inverter),
                    adder.inverter
                )
            )?;
            if i == self.adders.len() / 2 - 1 {
                writeln!(
                    f,
                    " {} {} {} {}",
                    check_connected(&arrow_left, adder.inverter, self.synchronizer),
                    color(self.synchronizer, self.synchronizer),
                    check_connected(&arrow_left, self.synchronizer, self.output),
                    color(self.output, self.synchronizer)
                )?;
            } else {
                writeln!(
                    f,
                    "{}",
                    line_ending(check_connected(
                        &arrow_left,
                        adder.inverter,
                        self.synchronizer
                    ))
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
