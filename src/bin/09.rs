fn make_move_h(h: &mut (i32, i32), dir: &str) {
    let mut d: (i32, i32) = (0, 0);
    match dir {
        "R" => {
            d = (0, 1);
        }
        "L" => {
            d = (0, -1);
        }
        "U" => {
            d = (-1, 0);
        }
        "D" => {
            d = (1, 0);
        }
        _ => {
            panic!("Wrong direction");
        }
    }

    h.0 += d.0;
    h.1 += d.1;
}

fn make_move_t(grid: &mut Vec<Vec<u32>>, h: (i32, i32), t: &mut (i32, i32), trace: bool) {
    if (h.0 - t.0).abs() <= 1 && (h.1 - t.1).abs() <= 1 {
        return;
    }

    t.0 += (h.0 - t.0).signum();
    t.1 += (h.1 - t.1).signum();
    if t.0 < 0 || t.1 < 0 {
        panic!("Out of bounds");
    }

    if trace {
        grid[t.0 as usize][t.1 as usize] = 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    let mut h: (i32, i32) = (500, 500);
    let mut t: (i32, i32) = (500, 500);

    grid[t.0 as usize][t.1 as usize] = 1;

    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let dir: &str = line[0];
        let steps: usize = line[1].parse::<usize>().unwrap();

        for _ in 0..steps {
            make_move_h(&mut h, dir);
            make_move_t(&mut grid, h, &mut t, true);
        }
    }

    Some(
        grid.into_iter()
            .map(|row| row.into_iter().sum::<u32>())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    let mut h: (i32, i32) = (500, 500);
    let mut t: Vec<(i32, i32)> = vec![(500, 500); 9];

    grid[h.0 as usize][h.1 as usize] = 1;

    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let dir: &str = line[0];
        let steps: usize = line[1].parse::<usize>().unwrap();

        for _ in 0..steps {
            make_move_h(&mut h, dir);
            make_move_t(&mut grid, h, &mut t[0], false);
            for i in 1..9 {
                make_move_t(&mut grid, t[i - 1], &mut t[i], i == 8);
            }
        }
    }

    Some(
        grid.into_iter()
            .map(|row| row.into_iter().sum::<u32>())
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
