use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let ids = ids.lines().map(|id| id.parse().unwrap()).collect();

    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse(input);

    Some(
        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse(input);
    ranges.sort_by_key(|r| *r.start());
    let mut ans = 0;
    let mut last_end_used = 0;

    for range in ranges {
        let start_used = (*range.start()).max(last_end_used);
        ans += (range.end()+1).saturating_sub(start_used);
        last_end_used = (*range.end()+1).max(last_end_used );
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
