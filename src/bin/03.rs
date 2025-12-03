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

pub fn part_one(input: &str) -> Option<u64> {
    count_max_voltage(input, |number| {
        let mut c = 0;
        let mut c2 = 0;

        let arr = number
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        for i in 0..arr.len() - 1 {
            if arr[i] > c {
                c = arr[i];
                c2 = 0;
            }
            if arr[i + 1] > c2 {
                c2 = arr[i + 1];
            }
        }
        c * 10 + c2
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    count_max_voltage(input, |number| {

        println!("{}","-".repeat(50));

        let mut ans = 0;

        let arr = number
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();


        let mut last_used = 0;

        let n = 12;

        for i in 0..n {
            println!("arr: {:?}, 11 - i: {} ,last_used: {}", arr.len(),11-i, last_used);
            let slice_size = arr.len() - (n - 1 - i) - last_used;
            println!("{}",slice_size);
            let mut max_value = 0;
            let mut idx_of_max = 0;
            println!("---------------------");
            for (i,x) in arr[last_used..(last_used+slice_size)].iter().enumerate() {
                println!("i : {}, x: {}",i,*x);
                if *x > max_value {
                    println!("toto {}",*x);
                    max_value = *x;
                    idx_of_max = (i + 1) as usize;
                }
            }
            ans = ans*10 + max_value;
            last_used += idx_of_max;

        }

        ans
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
