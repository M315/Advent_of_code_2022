use std::cmp::{max, min};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 1000]; 500];
    let mut max_depth = 0;

    for line in input.lines() {
        let path: Vec<Vec<usize>> = line
            .split(" -> ")
            .map(|x| {
                x.trim()
                    .split(',')
                    .map(|y| y.trim().parse::<usize>().expect("Can't parse to usize"))
                    .collect::<Vec<usize>>()
            })
            .collect();

        for i in 0..path.len() - 1 {
            if path[i][0] == path[i + 1][0] {
                let start: usize = min(path[i][1], path[i + 1][1]);
                let end: usize = max(path[i][1], path[i + 1][1]) + 1;
                for j in start..end {
                    grid[j][path[i][0]] = '#';
                }
            } else {
                let start: usize = min(path[i][0], path[i + 1][0]);
                let end: usize = max(path[i][0], path[i + 1][0]) + 1;
                for j in start..end {
                    grid[path[i][1]][j] = '#';
                }
            }
            max_depth = max(max_depth, path[i][1]);
            max_depth = max(max_depth, path[i + 1][1]);
        }
    }

    grid.truncate(max_depth + 3);

    grid
}

fn simulate(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i == grid.len() - 1 {
        return true;
    }

    if grid[i + 1][j] == '.' {
        return simulate(grid, i + 1, j);
    } else if grid[i + 1][j - 1] == '.' {
        return simulate(grid, i + 1, j - 1);
    } else if grid[i + 1][j + 1] == '.' {
        return simulate(grid, i + 1, j + 1);
    } else {
        grid[i][j] = 'o';
        if i == 0 {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = parse_input(input);
    let mut grands: u32 = 0;

    while !simulate(&mut grid, 0, 500) {
        grands += 1;
    }

    Some(grands)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = parse_input(input);
    let mut grands: u32 = 1;
    let n: usize = grid.len();

    // Insert the floor
    for j in 0..grid[0].len() {
        grid[n - 1][j] = '#';
    }

    while !simulate(&mut grid, 0, 500) {
        grands += 1;
    }

    Some(grands)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
