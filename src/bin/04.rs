advent_of_code::solution!(4);

#[derive(Debug,Eq,PartialEq)]
enum Tile {
    Empty,
    Full
}

impl Tile{
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Empty),
            '@' => Some(Tile::Full),
            _ => None,
        }
    }
}

fn count_non_empty_neighbors8(grid: &Vec<Vec<Tile>>, x: usize, y: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0;

    let directions: [(isize, isize); 8] = [
        (0, -1),
        (0,  1),
        (-1, 0),
        (1,  0),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];

    for (dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
            if grid[ny as usize][nx as usize] != Tile::Empty {
                count += 1;
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ans = 0;

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).flatten().collect())
        .collect();

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == Tile::Full {
                let neighbors = count_non_empty_neighbors8(&grid, x, y);
                if neighbors < 4 {
                    ans += 1;
                }
            }
        }

    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ans = 0;

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).flatten().collect())
        .collect();

    let mut nb_of_rolls = 0;

    loop {
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                if grid[y][x] == Tile::Full {
                    let neighbors = count_non_empty_neighbors8(&grid, x, y);
                    if neighbors < 4 {
                        nb_of_rolls += 1;
                        grid[y][x] = Tile::Empty;
                    }
                }
            }
        }

        if nb_of_rolls == 0 {
            break;
        }else {
            ans += nb_of_rolls;
            nb_of_rolls = 0;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
