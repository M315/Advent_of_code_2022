#[derive(Debug)]
struct Point(i32, i32, i32);

fn add_points(a: &Point, b: &Point) -> Point {
    Point(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn DFS(i: usize, j: usize, k: usize, block: &mut Vec<Vec<Vec<u32>>>) {
    if i >= block.len() || j >= block.len() || k >= block.len() {
        return;
    }

    if block[i][j][k] == 0 {
        block[i][j][k] = 2;
    } else {
        return;
    }

    DFS(i + 1, j, k, block);
    DFS(i, j + 1, k, block);
    DFS(i, j, k + 1, block);
    if i > 0 {
        DFS(i - 1, j, k, block);
    }
    if j > 0 {
        DFS(i, j - 1, k, block);
    }
    if k > 0 {
        DFS(i, j, k - 1, block);
    }
}

fn surface_point(pos: Point, block: &Vec<Vec<Vec<u32>>>, air: u32) -> u32 {
    if block[pos.0 as usize][pos.1 as usize][pos.2 as usize] != 1 {
        return 0;
    }

    let directions: Vec<Point> = vec![
        Point(1, 0, 0),
        Point(-1, 0, 0),
        Point(0, 1, 0),
        Point(0, -1, 0),
        Point(0, 0, 1),
        Point(0, 0, -1),
    ];

    let mut surface: u32 = 0;
    for dir in directions {
        let p: Point = add_points(&pos, &dir);
        if p.0 >= 0
            && p.0 < block.len() as i32
            && p.1 >= 0
            && p.1 < block.len() as i32
            && p.2 >= 0
            && p.2 < block.len() as i32
        {
            if block[p.0 as usize][p.1 as usize][p.2 as usize] == air {
                surface += 1;
            }
        } else {
            surface += 1;
        }
    }

    return surface;
}

pub fn part_one(input: &str) -> Option<u32> {
    let n: usize = 30;
    let mut block: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; n]; n]; n];

    for line in input.lines() {
        let pos: Vec<usize> = line
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        block[pos[0]][pos[1]][pos[2]] = 1;
    }

    let mut surface: u32 = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                surface += surface_point(Point(i as i32, j as i32, k as i32), &block, 0);
            }
        }
    }

    Some(surface)
}

pub fn part_two(input: &str) -> Option<u32> {
    let n: usize = 35;
    let mut block: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; n]; n]; n];

    for line in input.lines() {
        let pos: Vec<usize> = line
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        block[pos[0] + 5][pos[1] + 5][pos[2] + 5] = 1;
    }

    DFS(0, 0, 0, &mut block);

    let mut surface: u32 = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                surface += surface_point(Point(i as i32, j as i32, k as i32), &block, 2);
            }
        }
    }

    Some(surface)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
