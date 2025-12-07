use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Start,
    Splitter,
}

impl Tile {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Empty),
            'S' => Some(Tile::Start),
            '^' => Some(Tile::Splitter),
            _ => None,
        }
    }
}

fn find_start(grid: &[Vec<Tile>]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Start) {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut number_of_splits = 0;

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).flatten().collect())
        .collect();

    let height = grid.len();

    let mut beams: Vec<(u64, u64)> = vec![];

    if let Some(start) = find_start(&grid) {
        beams.push((start.0 as u64, start.1 as u64));
    } else {
        println!("No start found");
        return None;
    }

    loop {
        let mut new_beams: Vec<(u64, u64)> = vec![];
        for (x, y) in &beams {
            if *y >= (height - 1) as u64 {
                break;
            }
            let tile = &grid[*y as usize + 1][*x as usize];
            match tile {
                Tile::Empty => {
                    new_beams.push((*x, *y + 1));
                }
                Tile::Splitter => {
                    number_of_splits += 1;
                    new_beams.push((*x - 1, *y + 1));
                    new_beams.push((*x + 1, *y + 1));
                }
                _ => {}
            }
        }

        if new_beams.is_empty() {
            break;
        } else {
            new_beams.dedup();
            beams = new_beams;
        }
    }

    Some(number_of_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut number_of_splits = 0;

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).flatten().collect())
        .collect();

    let height = grid.len();

    let mut beams: HashMap<(u64, u64),u64> = HashMap::new();

    if let Some(start) = find_start(&grid) {
        beams.insert((start.0 as u64, start.1 as u64),1);
    } else {
        println!("No start found");
        return None;
    }

    loop {
        let mut new_beams: HashMap<(u64, u64),u64> = HashMap::new();
        for ((x, y),c) in beams.iter() {
            if *y >= (height - 1) as u64 {
                break;
            }
            let tile = &grid[*y as usize + 1][*x as usize];
            match tile {
                Tile::Empty => {
                    *new_beams.entry((*x, *y + 1)).or_default() += *c;
                }
                Tile::Splitter => {
                    number_of_splits += 1;
                    *new_beams.entry((*x - 1, *y + 1)).or_default() += *c;
                    *new_beams.entry((*x + 1, *y + 1)).or_default() += *c;
                }
                _ => {}
            }
        }

        if new_beams.is_empty() {
            break;
        } else {
            beams = new_beams;
        }
    }
    Some(beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
