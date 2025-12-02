advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let line = input.lines().next()?;

    let ranges = parse_ranges(line);

    let invalid_ids_sum: u64 = ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|id| is_repeated_twice(&id.to_string()))
        .sum();

    Some(invalid_ids_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_ranges(line: &str) -> Vec<(u64, u64)> {
    line.split(',')
        .filter_map(|range| {
            let (start, end) = range.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect()
}

fn is_repeated_twice(id: &str) -> bool {
    let len = id.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let (first, second) = id.split_at(len / 2);
    first == second
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
        assert_eq!(result, None);
    }
}
