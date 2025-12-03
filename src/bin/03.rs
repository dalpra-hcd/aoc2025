advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_ouput = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut largest = 0;

        for (i, d1) in bytes.iter().enumerate() {
            let d1 = (d1 - b'0') as u64;

            for d2 in bytes.iter().take(len).skip(i + 1) {
                let d2 = (d2 - b'0') as u64;
                let joltage = d1 * 10 + d2;

                if joltage > largest {
                    largest = joltage;
                }
            }
        }

        total_ouput += largest;
    }

    Some(total_ouput)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
