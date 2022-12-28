use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (i64, i64, HashMap<char, HashSet<(i64, i64)>>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut map: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();
    let n: usize = grid.len();
    let m: usize = grid[0].len();

    map.insert('>', HashSet::new());
    map.insert('<', HashSet::new());
    map.insert('v', HashSet::new());
    map.insert('^', HashSet::new());

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '.' && grid[i][j] != '#' {
                (*(map.get_mut(&grid[i][j]).unwrap())).insert((i as i64, j as i64));
            }
        }
    }

    (n as i64, m as i64, map)
}

fn move_arrows(n: i64, m: i64, set: HashSet<(i64, i64)>, dir: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut new_set: HashSet<(i64, i64)> = HashSet::new();

    for pos in set {
        let mut new_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if new_pos.0 == 0 {
            new_pos.0 = n - 2;
        }
        if new_pos.0 == n - 1 {
            new_pos.0 = 1;
        }
        if new_pos.1 == 0 {
            new_pos.1 = m - 2;
        }
        if new_pos.1 == m - 1 {
            new_pos.1 = 1;
        }

        new_set.insert(new_pos);
    }

    new_set
}

fn update_map(
    n: i64,
    m: i64,
    map: HashMap<char, HashSet<(i64, i64)>>,
) -> HashMap<char, HashSet<(i64, i64)>> {
    let mut new_map: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();

    for (key, set) in map {
        match key {
            '>' => {
                new_map.insert('>', move_arrows(n, m, set, (0, 1)));
            }
            '<' => {
                new_map.insert('<', move_arrows(n, m, set, (0, -1)));
            }
            'v' => {
                new_map.insert('v', move_arrows(n, m, set, (1, 0)));
            }
            '^' => {
                new_map.insert('^', move_arrows(n, m, set, (-1, 0)));
            }
            _ => panic!("Unexpected direction"),
        }
    }

    new_map
}

fn move_explo(
    n: i64,
    m: i64,
    e: HashSet<(i64, i64)>,
    map: &HashMap<char, HashSet<(i64, i64)>>,
) -> HashSet<(i64, i64)> {
    let directions: Vec<(i64, i64)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)];
    let mut new_explo: HashSet<(i64, i64)> = HashSet::new();

    for &pos in e.iter() {
        for &dir in directions.iter() {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            let mut valid_pos = true;

            for obs in map.values() {
                if obs.contains(&new_pos) {
                    valid_pos = false;
                    break;
                }
            }

            if !valid_pos {
                continue;
            }

            if new_pos.0 > 0 && new_pos.0 < n - 1 && new_pos.1 > 0 && new_pos.1 < m - 1 {
                new_explo.insert(new_pos);
            }

            if new_pos.0 == 0 && new_pos.1 == 1 {
                new_explo.insert(new_pos);
            }

            if new_pos.0 == n - 1 && new_pos.1 == m - 2 {
                new_explo.insert(new_pos);
            }
        }
    }

    return new_explo;
}

fn bfs(
    n: i64,
    m: i64,
    times: i64,
    start: (i64, i64),
    target: (i64, i64),
    mut map: HashMap<char, HashSet<(i64, i64)>>,
) -> Option<u64> {
    let mut round: u64 = 0;
    let mut e: HashSet<(i64, i64)> = HashSet::new();

    e.insert(start);

    while !e.contains(&target) {
        map = update_map(n, m, map);
        e = move_explo(n, m, e, &map);
        round += 1;
    }

    if times > 1 {
        e = HashSet::new();
        e.insert(target);

        while !e.contains(&start) {
            map = update_map(n, m, map);
            e = move_explo(n, m, e, &map);
            round += 1;
        }

        e = HashSet::new();
        e.insert(start);

        while !e.contains(&target) {
            map = update_map(n, m, map);
            e = move_explo(n, m, e, &map);
            round += 1;
        }
    }

    Some(round)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (n, m, map) = parse(input);
    bfs(n, m, 1, (0, 1), (n - 1, m - 2), map)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (n, m, map) = parse(input);
    bfs(n, m, 2, (0, 1), (n - 1, m - 2), map)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
