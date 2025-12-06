advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let accessible_cnt = count_accessible(grid);
    Some(accessible_cnt)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

fn count_accessible(grid: Vec<Vec<char>>) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible_cnt = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let mut neighbour_cnt = 0;

                for (dr, dc) in directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    let mr = rows as isize;
                    let mc = cols as isize;

                    if nr >= 0
                        && nr < mr
                        && nc >= 0
                        && nc < mc
                        && grid[nr as usize][nc as usize] == '@'
                    {
                        neighbour_cnt += 1;
                    }
                }

                if neighbour_cnt < 4 {
                    accessible_cnt += 1;
                }
            }
        }
    }

    accessible_cnt
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
        assert_eq!(result, None);
    }
}
