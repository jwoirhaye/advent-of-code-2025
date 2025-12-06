advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
    Undefined,
}

impl Op {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
            Op::Undefined => panic!("Cannot apply undefined operator"),
        }
    }
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '*' => Op::Multiply,
            _ => panic!("Unknown operation"),
        }
    }
}

#[derive(Debug, Clone)]
struct Column {
    numbers: Vec<u64>,
    op: Op,
}

fn parse(input: &str) -> Vec<Column> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut cols = vec![
        Column {
            numbers: Vec::new(),
            op: Op::Undefined
        };
        lines[0].len()
    ];

    for line in lines[0..(lines.len() - 1)].iter() {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        for (i, number) in numbers.iter().enumerate() {
            cols[i].numbers.push(number.parse::<u64>().unwrap());
        }
    }

    for line in lines[lines.len() - 1..lines.len()].iter() {
        let ops = line.split_whitespace().collect::<Vec<&str>>();
        for (i, op) in ops.iter().enumerate() {
            cols[i].op = Op::from(op.chars().nth(0).unwrap());
        }
    }
    cols
}

pub fn part_one(input: &str) -> Option<u64> {
    let cols = parse(input);
    let nums: Vec<u64> = cols
        .iter()
        .map(|col| {
            col.numbers
                .iter()
                .copied()
                .reduce(|x, x1| col.op.apply(x, x1))
                .unwrap_or(0)
        })
        .collect();

    Some(nums.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() < 2 {
        return None;
    }

    let width = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| line.len())
        .max()?;

    let height = lines.len() - 1;

    let data_lines: Vec<char> = lines
        .iter()
        .take(height)
        .flat_map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    let op_line: Vec<char> = lines
        .last()?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut ans = 0;
    let mut current_problem = 0;
    let mut current_op = op_line.len().saturating_sub(1);

    for x in (0..width).rev() {
        let number = (0..height)
            .map(|y| data_lines[y * width + x])
            .filter(|&c| c != ' ')
            .try_fold(0u64, |acc, c| c.to_digit(10).map(|d| acc * 10 + d as u64))?;

        if number != 0 {
            current_problem = match op_line.get(current_op) {
                Some(&'+') => current_problem + number,
                Some(&'*') => current_problem.max(1) * number,
                Some(&op) => {
                    eprintln!("Unknown operation: '{}'", op);
                    return None;
                }
                None => current_problem,
            };
        }

        if number == 0 || x == 0 {
            ans += current_problem;
            current_problem = 0;
            current_op = current_op.saturating_sub(1);
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
