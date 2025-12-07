advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    let mut reading_ranges = true;

    for line in input.lines() {
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let (start, end) = line.split_once("-")?;
            ranges.push((start.parse::<u64>().ok()?, end.parse::<u64>().ok()?));
        } else {
            ingredients.push(line.parse::<u64>().ok()?);
        }
    }

    let mut fresh = 0;

    for ingredient in ingredients {
        for (start, end) in &ranges {
            if (*start..=*end).contains(&ingredient) {
                fresh += 1;
                break;
            }
        }
    }

    Some(fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-")?;
        ranges.push((start.parse::<u64>().ok()?, end.parse::<u64>().ok()?));
    }

    ranges.sort_by_key(|r| r.0);

    let mut fresh = 0u64;

    let mut fresh_start = ranges[0].0;
    let mut fresh_end = ranges[0].1;

    for &(start, end) in ranges.iter().skip(1) {
        if start > fresh_end + 1 {
            // gap
            fresh += fresh_end - fresh_start + 1;
            fresh_start = start;
            fresh_end = end;
        } else if end > fresh_end {
            // overlap
            fresh_end = end;
        }
    }

    fresh += fresh_end - fresh_start + 1;

    Some(fresh)
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
