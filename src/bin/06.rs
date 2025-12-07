advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut homework: Vec<&str> = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            homework.push(line);
        }
    }

    let operations: Vec<&str> = homework.last()?.split_ascii_whitespace().collect();
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    for line in homework.iter().take(homework.len() - 1) {
        let ns: Vec<u64> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        numbers.push(ns);
    }

    let mut grand_total = 0u64;
    for (idx, op) in operations.iter().enumerate() {
        let problem: u64 = match *op {
            "+" => {
                let mut sum = 0u64;
                for row in &numbers {
                    sum += row[idx];
                }
                sum
            }
            "*" => {
                let mut prod = 1u64;
                for row in &numbers {
                    prod *= row[idx];
                }
                prod
            }
            _ => unreachable!(),
        };
        grand_total += problem;
    }

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
