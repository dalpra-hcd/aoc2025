advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_ouput = 0;

    for line in input.lines() {
        total_ouput += largest_joltage(line, 2);
    }

    Some(total_ouput)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_ouput = 0;

    for line in input.lines() {
        total_ouput += largest_joltage(line, 12);
    }

    Some(total_ouput)
}

fn largest_joltage(bank: &str, n_bat: u32) -> u64 {
    match n_bat {
        0 => 0,
        n => do_largest_joltage(bank, n),
    }
}

fn do_largest_joltage(bank: &str, n_bat: u32) -> u64 {
    let bytes = bank.as_bytes();
    let len = bank.len() - (n_bat as usize) + 1;
    let mut max = 0;
    let mut idx = 0;

    for (i, b) in bytes[0..len].iter().enumerate() {
        let d = (b - b'0') as u64;

        if d > max {
            max = d;
            idx = i;
        }
    }

    max *= 10_u64.pow(n_bat - 1);
    max + largest_joltage(&bank[idx + 1..], n_bat - 1)
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
