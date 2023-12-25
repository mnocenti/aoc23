use aoc23::*;

main!(21, 525152);

#[apply(parse_ordered!)]
#[delim(' ')]
#[derive(Debug, Default)]
struct Row {
    #[parse()]
    row: String,
    #[parse(collect(','))]
    groups: Vec<usize>,
}

type Springs = Vec<Row>;

fn parse(input: &str) -> Result<Springs> {
    collect_lines(input)
}

fn part1(springs: &Springs) -> Result<usize> {
    Ok(springs.iter().map(Row::count_arrangements).sum())
}

fn part2(springs: &Springs) -> Result<usize> {
    Ok(springs
        .iter()
        .map(|row| row.unfold())
        .map(|row| row.count_arrangements())
        .sum())
}

impl Row {
    pub fn count_arrangements(&self) -> usize {
        Self::arrangements(self.row.as_bytes(), 0, &self.groups)
    }

    fn arrangements(row: &[u8], current_group_length: usize, groups: &[usize]) -> usize {
        if row.is_empty() {
            if groups.is_empty() && current_group_length == 0 {
                return 1;
            } else if groups.len() == 1 && current_group_length == groups[0] {
                return 1;
            } else {
                // invalid arrangement
                return 0;
            }
        }
        let mut count = 0;
        count += if [b'.', b'?'].contains(&row[0]) {
            let can_close_group = current_group_length == 0
                || (!groups.is_empty() && current_group_length == groups[0]);
            if !can_close_group {
                // invalid arrangement
                0
            } else if current_group_length == 0 {
                Self::arrangements(&row[1..], 0, groups)
            } else if groups.is_empty() {
                // invalid arrangement: no enough groups
                0
            } else {
                Self::arrangements(&row[1..], 0, &groups[1..])
            }
        } else {
            0
        };
        count += if [b'#', b'?'].contains(&row[0]) {
            if groups.is_empty() {
                0
            } else if current_group_length == 0 {
                // start a new group
                Self::arrangements(&row[1..], 1, groups)
            } else if current_group_length + 1 > groups[0] {
                // group is too big
                0
            } else {
                Self::arrangements(&row[1..], current_group_length + 1, groups)
            }
        } else {
            0
        };
        count
    }

    fn unfold(&self) -> Self {
        Row {
            row: vec![self.row.clone(); 5].join("?"),
            groups: self.groups.repeat(5),
        }
    }
}
