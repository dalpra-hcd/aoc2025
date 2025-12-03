advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut password = 0;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();
        let rot: i32 = match dir {
            "L" => -num,
            "R" => num,
            _ => unreachable!(),
        };

        dial = (dial + rot).rem_euclid(100);
        if dial == 0 {
            password += 1;
        }
    }

    Some(password)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dial = 50;
    let mut password = 0;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();
        let rot: i32 = match dir {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };

        for _ in 0..num {
            dial = (dial + rot).rem_euclid(100);
            if dial == 0 {
                password += 1;
            }
        }
    }

    Some(password)
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
        assert_eq!(result, Some(6));
    }
}
