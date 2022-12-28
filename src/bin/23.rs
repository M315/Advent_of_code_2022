use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashSet<(i64, i64)> {
    let mut elves: HashSet<(i64, i64)> = HashSet::new();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                elves.insert((i as i64, j as i64));
            }
        }
    }

    return elves;
}

fn propose(round: usize, elves: &HashSet<(i64, i64)>) -> HashMap<(i64, i64), (i64, i64)> {
    let mut proposal: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    let dir: Vec<Vec<(i64, i64)>> = vec![
        vec![(-1, 0), (-1, 1), (-1, -1)],
        vec![(1, 0), (1, 1), (1, -1)],
        vec![(0, -1), (1, -1), (-1, -1)],
        vec![(0, 1), (1, 1), (-1, 1)],
    ];

    for &elve in elves.iter() {
        let mut made_move: u64 = 0;
        let mut prop: (i64, i64) = (0, 0);
        for k in 0..4 {
            let i: usize = (round + k).rem_euclid(4);
            let mut valid_move: bool = true;
            for j in 0..3 {
                if elves.contains(&(elve.0 + dir[i][j].0, elve.1 + dir[i][j].1)) {
                    valid_move = false;
                }
            }
            if valid_move && made_move == 0 {
                prop = (elve.0 + dir[i][0].0, elve.1 + dir[i][0].1);
            }
            if valid_move {
                made_move += 1;
            }
        }
        if made_move == 0 || made_move == 4 {
            proposal.insert(elve, elve);
        } else {
            proposal.insert(elve, prop);
        }
    }

    return proposal;
}

fn next_round(round: usize, elves: &HashSet<(i64, i64)>) -> (bool, HashSet<(i64, i64)>) {
    let proposal: HashMap<(i64, i64), (i64, i64)> = propose(round, elves);
    let mut count: HashMap<(i64, i64), u64> = HashMap::new();
    let mut new_elves: HashSet<(i64, i64)> = HashSet::new();
    let mut changes: u64 = 0;

    for &new_pos in proposal.values() {
        count
            .entry(new_pos)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for (curr_pos, new_pos) in proposal {
        if count[&new_pos] == 1 && curr_pos != new_pos {
            changes += 1;
        }
        if count[&new_pos] == 1 {
            new_elves.insert(new_pos);
        } else {
            new_elves.insert(curr_pos);
        }
    }

    if changes == 0 {
        return (true, new_elves);
    }

    return (false, new_elves);
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut elves: HashSet<(i64, i64)> = parse(input);
    let mut round: usize = 0;

    loop {
        let (result, new_elves) = next_round(round, &elves);
        elves = new_elves;
        round += 1;
        if result || round == 10 {
            break;
        }
    }

    let mut max_x: i64 = i64::MIN;
    let mut min_x: i64 = i64::MAX;
    let mut max_y: i64 = i64::MIN;
    let mut min_y: i64 = i64::MAX;
    let n: i64 = elves.len() as i64;

    for elve in elves {
        max_x = max_x.max(elve.0);
        min_x = min_x.min(elve.0);
        max_y = max_y.max(elve.1);
        min_y = min_y.min(elve.1);
    }

    Some(((max_x + 1 - min_x) * (max_y + 1 - min_y)) - n)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut elves: HashSet<(i64, i64)> = parse(input);
    let mut round: usize = 0;

    loop {
        let (result, new_elves) = next_round(round, &elves);
        elves = new_elves;
        round += 1;
        if result {
            return Some(round as u64);
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
