advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut dial = 50;
    let mut password = 0;

    for line in input.lines() {
        let (direction, num) = line.split_at(1);
        let mut num = num.parse::<i32>().ok()?;

        if direction == "L" {
            num *= -1;
        }

        dial = (dial + num).rem_euclid(100);
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
        let (direction, num) = line.split_at(1);
        let mut num = num.parse::<i32>().ok()?;

        if direction == "L" {
            num *= -1;
        }

        if num >= 0 {
            password += (dial + num) / 100;
        } else {
            let reversed = (100 - dial) % 100;
            password += (reversed - num) / 100;
        }

        dial = (dial + num).rem_euclid(100);
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
