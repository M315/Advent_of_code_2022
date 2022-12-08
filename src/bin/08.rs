fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = parse_input(input);
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let mut seen: Vec<Vec<u32>> = vec![vec![0; m]; n];

    seen[0][0] = 1;
    seen[0][m - 1] = 1;
    seen[n - 1][0] = 1;
    seen[n - 1][m - 1] = 1;

    for i in 1..n - 1 {
        seen[i][0] = 1;
        seen[i][m - 1] = 1;
        // Look right
        let mut max_height = grid[i][0];
        for j in 1..m {
            if max_height < grid[i][j] {
                seen[i][j] = 1;
                max_height = grid[i][j];
            }
        }
        // Look left
        let mut max_height = grid[i][m - 1];
        for j in 1..m {
            if max_height < grid[i][m - 1 - j] {
                seen[i][m - 1 - j] = 1;
                max_height = grid[i][m - 1 - j];
            }
        }
    }

    for j in 1..m - 1 {
        seen[0][j] = 1;
        seen[n - 1][j] = 1;
        // Look down
        let mut max_height = grid[0][j];
        for i in 1..n {
            if max_height < grid[i][j] {
                seen[i][j] = 1;
                max_height = grid[i][j];
            }
        }
        // Look up
        let mut max_height = grid[n - 1][j];
        for i in 1..n {
            if max_height < grid[n - 1 - i][j] {
                seen[n - 1 - i][j] = 1;
                max_height = grid[n - 1 - i][j];
            }
        }
    }

    return Some(seen.into_iter().map(|x| x.into_iter().sum::<u32>()).sum());
}

fn get_score(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let n: usize = grid.len();
    let m: usize = grid[0].len();

    //Look up
    let mut up: u32 = 0;
    for i in 1..n {
        up += 1;
        if x - i == 0 || grid[x - i][y] >= grid[x][y] {
            break;
        }
    }
    //Look down
    let mut down: u32 = 0;
    for i in 1..n {
        down += 1;
        if x + i == n - 1 || grid[x + i][y] >= grid[x][y] {
            break;
        }
    }

    //Look right
    let mut right: u32 = 0;
    for j in 1..m {
        right += 1;
        if y + j == m - 1 || grid[x][y + j] >= grid[x][y] {
            break;
        }
    }

    //Look left
    let mut left: u32 = 0;
    for j in 1..m {
        left += 1;
        if y - j == 0 || grid[x][y - j] >= grid[x][y] {
            break;
        }
    }

    return up * down * right * left;
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = parse_input(input);
    let mut max_score: u32 = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            let score: u32 = get_score(i, j, &grid);
            max_score = std::cmp::max(max_score, score);
        }
    }

    return Some(max_score);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
