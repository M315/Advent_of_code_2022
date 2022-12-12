use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; m]; n];

    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost + 1);
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        for dir in directions.iter() {
            if position.0 as i32 + dir.0 < 0
                || position.0 as i32 + dir.0 >= n as i32
                || position.1 as i32 + dir.1 < 0
                || position.1 as i32 + dir.1 >= m as i32
            {
                continue;
            }

            let next = State {
                cost: cost + 1,
                position: (
                    (position.0 as i32 + dir.0) as usize,
                    (position.1 as i32 + dir.1) as usize,
                ),
            };

            if grid[position.0][position.1] + 1 < grid[next.position.0][next.position.1] {
                continue;
            }

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut i: usize = 0;

    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        let mut j: usize = 0;
        for c in line.chars() {
            if c == 'S' {
                start = (i, j);
                row.push(0);
                continue;
            }
            if c == 'E' {
                end = (i, j);
                row.push(26);
                continue;
            }
            row.push(c as u32 - 97);
            j += 1;
        }
        grid.push(row);
        i += 1;
    }

    shortest_path(&grid, start, end)
}

fn shortest_path_2(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> Option<u32> {
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; m]; n];

    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if grid[position.0][position.1] == 0 {
            return Some(cost + 1);
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        for dir in directions.iter() {
            if position.0 as i32 + dir.0 < 0
                || position.0 as i32 + dir.0 >= n as i32
                || position.1 as i32 + dir.1 < 0
                || position.1 as i32 + dir.1 >= m as i32
            {
                continue;
            }

            let next = State {
                cost: cost + 1,
                position: (
                    (position.0 as i32 + dir.0) as usize,
                    (position.1 as i32 + dir.1) as usize,
                ),
            };

            if grid[next.position.0][next.position.1] + 1 < grid[position.0][position.1] {
                continue;
            }

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    None
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut end: (usize, usize) = (0, 0);
    let mut i: usize = 0;

    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        let mut j: usize = 0;
        for c in line.chars() {
            if c == 'S' {
                row.push(0);
                continue;
            }
            if c == 'E' {
                end = (i, j);
                row.push(26);
                continue;
            }
            row.push(c as u32 - 97);
            j += 1;
        }
        grid.push(row);
        i += 1;
    }

    shortest_path_2(&grid, end)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(30));
    }
}
