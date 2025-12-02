use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let str_intervals = input.trim().split(",");
    let mut count = 0;
    for str_interval in str_intervals {
        let (start_str, end_str) = str_interval.split_once("-")?;
        let start: u64 = start_str.parse().ok()?;
        let end: u64 = end_str.parse().ok()?;
        for value in start..=end {
            let digits = value.ilog10().div_ceil(2);
            let mask = u64::pow(10, digits);
            if value % mask == value / mask {
                count += value;
            }
        }
    }
    Some(count)
}

fn is_in_ranges(value: u64, ranges: &[(u64, u64)]) -> bool {
    for &(start, end) in ranges {
        if value >= start && value <= end {
            return true;
        }
    }
    false
}

fn explore_motifs(
    prefix: u64,
    len_p: u32,
    min_val: u64,
    max_val: u64,
    max_digits: u32,
    ranges: &[(u64, u64)],
    seen: &mut HashSet<u64>,
    sum: &mut u64,
) {
    if len_p > max_digits / 2 {
        return;
    }

    if len_p > 0 {
        let pow_len = 10u64.pow(len_p);
        let mut value = prefix * pow_len + prefix;
        let mut digits_value = len_p * 2;

        if digits_value > max_digits || value > max_val {
            return;
        }

        while digits_value <= max_digits && value <= max_val {
            if value >= min_val && is_in_ranges(value, ranges) {
                if seen.insert(value) {
                    *sum += value;
                }
            }

            if digits_value + len_p > max_digits {
                break;
            }
            value = value * pow_len + prefix;
            digits_value += len_p;
        }
    }

    if len_p == max_digits / 2 {
        return;
    }

    for d in 0..=9 {
        if len_p == 0 && d == 0 {
            continue;
        }

        let new_prefix = prefix * 10 + d as u64;
        explore_motifs(
            new_prefix,
            len_p + 1,
            min_val,
            max_val,
            max_digits,
            ranges,
            seen,
            sum,
        );
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges = Vec::new();

    for str_interval in input.trim().split(',') {
        let (start_str, end_str) = str_interval.split_once('-')?;
        let start: u64 = start_str.parse().ok()?;
        let end: u64 = end_str.parse().ok()?;
        ranges.push((start, end));
    }

    if ranges.is_empty() {
        return Some(0);
    }

    let mut min_val = u64::MAX;
    let mut max_val = 0u64;
    for &(start, end) in &ranges {
        if start < min_val {
            min_val = start;
        }
        if end > max_val {
            max_val = end;
        }
    }

    let max_digits: u32 = if max_val == 0 {
        1
    } else {
        max_val.ilog10() + 1
    };

    let mut seen = HashSet::new();
    let mut sum = 0u64;

    explore_motifs(
        0, 0, min_val, max_val, max_digits, &ranges, &mut seen, &mut sum,
    );

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
