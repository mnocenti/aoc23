use aoc23::*;

main!(1320, 145);

type InitSeq<'a> = Vec<&'a str>;

fn parse(input: &str) -> Result<InitSeq> {
    Ok(input.trim().split(',').collect())
}

fn part1(init_seq: &InitSeq) -> Result<usize> {
    Ok(init_seq.iter().map(|s| hash(s)).sum())
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal: usize,
}

enum Instr<'a> {
    Set(&'a str, usize),
    Remove(&'a str),
}

type Hashmap<'a> = Vec<Vec<Lens<'a>>>;

fn part2(init_seq: &InitSeq) -> Result<usize> {
    let mut hashmap: Hashmap = vec![Vec::new(); 256];
    let instructions = init_seq.iter().filter_map(|s| {
        s.split_once('=')
            .and_then(|(label, focal)| Some(Instr::Set(label, focal.parse().ok()?)))
            .or_else(|| Some(Instr::Remove(s.trim_end_matches('-'))))
    });
    for instr in instructions {
        match instr {
            Instr::Set(label, focal) => insert(&mut hashmap[hash(label)], label, focal),
            Instr::Remove(label) => remove(&mut hashmap[hash(label)], label),
        }
    }
    Ok(focusing_power(&hashmap))
}

fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |mut acc, b| {
        acc += *b as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn insert<'a>(lens_box: &mut Vec<Lens<'a>>, label: &'a str, focal: usize) {
    match lens_box.iter_mut().find(|lens| lens.label == label) {
        Some(lens) => lens.focal = focal,
        None => lens_box.push(Lens { label, focal }),
    }
}

fn remove(lens_box: &mut Vec<Lens<'_>>, label: &str) {
    if let Some(i) = lens_box.iter_mut().position(|lens| lens.label == label) {
        lens_box.remove(i);
    }
}

fn focusing_power(hashmap: &Hashmap) -> usize {
    hashmap
        .iter()
        .enumerate()
        .flat_map(|(boxn, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(slot, lens)| (boxn + 1) * (slot + 1) * lens.focal)
        })
        .sum()
}
