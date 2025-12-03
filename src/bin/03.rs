advent_of_code::solution!(3);

fn count_max_voltage(input: &str, get_max_number: fn(&str) -> u64) -> Option<u64> {
    let mut max_voltage = 0;
    for line in input.lines() {
        let number: &str = line.trim();
        let voltage = get_max_number(number);
        max_voltage += voltage;
    }
    Some(max_voltage)
}

fn max_joltage_n(number: &str, n: usize) -> u64 {
    let mut ans = 0;

    let arr = number
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    let mut last_used = 0;

    for i in 0..n {
        let slice_size = arr.len() - (n - 1 - i) - last_used;
        let mut max_value = 0;
        let mut idx_of_max = 0;
        for (i, x) in arr[last_used..(last_used + slice_size)].iter().enumerate() {
            if *x > max_value {
                max_value = *x;
                idx_of_max = i + 1;
            }
        }
        ans = ans * 10 + max_value;
        last_used += idx_of_max;
    }

    ans
}

pub fn part_one(input: &str) -> Option<u64> {
    count_max_voltage(input, |number| {
        max_joltage_n(number, 2)
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    count_max_voltage(input, |number| {
        max_joltage_n(number, 12)
    })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
