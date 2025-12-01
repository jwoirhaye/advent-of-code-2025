advent_of_code::solution!(1);
const DIAL_NUMBER: u64 = 100;

fn parse_line(line: &str) -> Option<(i64, u64)> {
    let line = line.trim();
    let (direction, distance) = line.split_at(1);
    let distance: u64 = distance.parse().ok()?;

    let delta: i64 = match direction {
        "L" => -(distance as i64),
        "R" => distance as i64,
        _ => return None,
    };

    Some((delta, distance))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut count = 0;

    for line in input.lines() {
        let (delta, _) = parse_line(line)?;
        position = (position + delta).rem_euclid(DIAL_NUMBER as i64);
        if position == 0 {
            count += 1;
        }
    }

    Some(count)
}

fn count_zero_crossings(position: u64, delta: i64, distance: u64) -> u64 {
    if distance == 0 {
        return 0;
    }

    let first_hit = if delta > 0 {
        if position == 0 { DIAL_NUMBER } else { DIAL_NUMBER - position }
    } else {
        if position == 0 { DIAL_NUMBER } else { position }
    };

    if first_hit > distance {
        0
    } else {
        1 + (distance - first_hit) / DIAL_NUMBER
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut count = 0;

    for line in input.lines() {
        let (delta, distance) = parse_line(line)?;
        count += count_zero_crossings(position, delta, distance);
        position = (position as i64 + delta).rem_euclid(DIAL_NUMBER as i64) as u64;
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
