advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let operations: Vec<&str> = lines.pop()?.split_ascii_whitespace().collect();
    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        })
        .collect();

    let grand_total: u64 = operations
        .iter()
        .enumerate()
        .map(|(idx, op)| match *op {
            "+" => numbers.iter().map(|row| row[idx]).sum::<u64>(),
            "*" => numbers.iter().map(|row| row[idx]).product(),
            _ => unreachable!(),
        })
        .sum();

    Some(grand_total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
