use num_complex::Complex;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<i64>, Vec<char>) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut grid: Vec<Vec<char>> = input[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let m: usize = grid[0].len();
    for i in 0..grid.len() {
        grid[i].resize(m, ' ');
    }

    let steps: Vec<i64> = input[1]
        .trim()
        .split(char::is_uppercase)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let rotations: Vec<char> = input[1]
        .trim()
        .matches(char::is_uppercase)
        .map(|x| x.chars().collect::<Vec<char>>()[0])
        .collect();

    return (grid, steps, rotations);
}

fn rotate(rotation: char, dir: Complex<i64>) -> Complex<i64> {
    match rotation {
        'L' => dir * Complex::i(),
        'R' => dir * Complex::i() * Complex::i() * Complex::i(),
        _ => panic!("Invalid rotation"),
    }
}

fn make_move(
    pos: Complex<i64>,
    dir: Complex<i64>,
    grid: &Vec<Vec<char>>,
) -> Result<Complex<i64>, &str> {
    let mut new_pos: Complex<i64> = pos + dir;

    if new_pos.re < 0 {
        new_pos.re = (grid.len() - 1) as i64;
    }
    if new_pos.re >= grid.len() as i64 {
        new_pos.re = 0;
    }
    if new_pos.im < 0 {
        new_pos.im = (grid[0].len() - 1) as i64;
    }
    if new_pos.im >= grid[0].len() as i64 {
        new_pos.im = 0;
    }

    if grid[pos.re as usize][pos.im as usize] == ' '
        && grid[new_pos.re as usize][new_pos.im as usize] == '#'
    {
        return Err("Wall after fall");
    }

    if grid[new_pos.re as usize][new_pos.im as usize] == ' ' {
        return make_move(new_pos, dir, grid);
    }

    if grid[new_pos.re as usize][new_pos.im as usize] == '#' {
        return Ok(pos);
    }

    return Ok(new_pos);
}

fn update_pos(
    steps: i64,
    mut pos: Complex<i64>,
    dir: Complex<i64>,
    grid: &Vec<Vec<char>>,
) -> Complex<i64> {
    for _ in 0..steps {
        pos = match make_move(pos, dir, grid) {
            Err(_) => pos,
            Ok(new_pos) => new_pos,
        };
    }

    pos
}

fn simulate(
    mut pos: Complex<i64>,
    grid: &Vec<Vec<char>>,
    steps: Vec<i64>,
    rotations: Vec<char>,
) -> Option<i64> {
    let mut dir: Complex<i64> = Complex::new(0, 1);

    for k in 0..steps.len() - 1 {
        pos = update_pos(steps[k], pos, dir, grid);
        dir = rotate(rotations[k], dir);
    }
    pos = update_pos(steps[steps.len() - 1], pos, dir, grid);

    let mut dir_val: i64 = 0;
    if dir.re == 1 {
        dir_val = 1;
    }
    if dir.im == -1 {
        dir_val = 2;
    }
    if dir.re == -1 {
        dir_val = 3;
    }

    Some((pos.re + 1) * 1000 + (pos.im + 1) * 4 + dir_val)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (grid, steps, rotations) = parse(input);

    let mut pos: Complex<i64> = Complex::new(0, 0);
    for j in 0..grid[0].len() {
        if grid[0][j] == '.' {
            pos = Complex::new(0, j as i64);
            break;
        }
    }

    simulate(pos, &grid, steps, rotations)
}

fn make_move_2(
    pos: Complex<i64>,
    dir: Complex<i64>,
    grid: &Vec<Vec<char>>,
) -> Result<(Complex<i64>, Complex<i64>), &str> {
    let mut new_pos: Complex<i64> = pos + dir;
    let mut new_dir: Complex<i64> = dir;

    // Wrapping
    if new_pos.re < 0 {
        if new_pos.im < 50 {
            new_pos.re = new_pos.im + 50;
            new_pos.im = 0;
            new_dir = Complex::new(0, 1);
        } else if new_pos.im < 100 {
            new_pos.re = new_pos.im + 100;
            new_pos.im = 0;
            new_dir = Complex::new(0, 1);
        } else {
            new_pos.re = (grid.len() - 1) as i64;
            new_pos.im = new_pos.im - 100;
            new_dir = Complex::new(-1, 0);
        }
    }
    if new_pos.re >= grid.len() as i64 {
        if new_pos.im < 50 {
            new_pos.re = 0;
            new_pos.im = new_pos.im + 100;
            new_dir = Complex::new(1, 0);
        } else if new_pos.im < 100 {
            new_pos.re = new_pos.im + 100;
            new_pos.im = (grid[0].len() - 1) as i64;
            new_dir = Complex::new(0, -1);
        } else {
            new_pos.re = new_pos.im - 50;
            new_pos.im = (grid[0].len() - 1) as i64;
            new_dir = Complex::new(0, -1);
        }
    }
    if new_pos.im < 0 {
        if new_pos.re < 50 {
            new_pos.re = 149 - new_pos.re;
            new_pos.im = 0;
            new_dir = Complex::new(0, 1);
        } else if new_pos.re < 100 {
            new_pos.im = new_pos.re - 50;
            new_pos.re = 0;
            new_dir = Complex::new(1, 0);
        } else if new_pos.re < 150 {
            new_pos.re = 149 - new_pos.re;
            new_pos.im = 0;
            new_dir = Complex::new(0, 1);
        } else {
            new_pos.im = new_pos.re - 100;
            new_pos.re = 0;
            new_dir = Complex::new(1, 0);
        }
    }
    if new_pos.im >= grid[0].len() as i64 {
        if new_pos.re < 50 {
            new_pos.re = 149 - new_pos.re;
            new_pos.im = (grid[0].len() - 1) as i64;
            new_dir = Complex::new(0, -1);
        } else if new_pos.re < 100 {
            new_pos.im = new_pos.re + 50;
            new_pos.re = (grid.len() - 1) as i64;
            new_dir = Complex::new(-1, 0);
        } else if new_pos.re < 150 {
            new_pos.re = 149 - new_pos.re;
            new_pos.im = (grid[0].len() - 1) as i64;
            new_dir = Complex::new(0, -1);
        } else {
            new_pos.im = new_pos.re - 100;
            new_pos.re = (grid.len() - 1) as i64;
            new_dir = Complex::new(-1, 0);
        }
    }

    if grid[pos.re as usize][pos.im as usize] == ' '
        && grid[new_pos.re as usize][new_pos.im as usize] == '#'
    {
        return Err("Wall after fall");
    }

    if grid[new_pos.re as usize][new_pos.im as usize] == ' ' {
        return make_move_2(new_pos, new_dir, grid);
    }

    if grid[new_pos.re as usize][new_pos.im as usize] == '#' {
        return Ok((pos, dir));
    }

    return Ok((new_pos, new_dir));
}

fn update_pos_2(
    steps: i64,
    mut pos: Complex<i64>,
    mut dir: Complex<i64>,
    grid: &Vec<Vec<char>>,
) -> (Complex<i64>, Complex<i64>) {
    for _ in 0..steps {
        (pos, dir) = match make_move_2(pos, dir, grid) {
            Err(_) => (pos, dir),
            Ok((new_pos, new_dir)) => (new_pos, new_dir),
        };
    }

    (pos, dir)
}

fn simulate_2(
    mut pos: Complex<i64>,
    grid: &Vec<Vec<char>>,
    steps: Vec<i64>,
    rotations: Vec<char>,
) -> Option<i64> {
    let mut dir: Complex<i64> = Complex::new(0, 1);

    for k in 0..steps.len() - 1 {
        (pos, dir) = update_pos_2(steps[k], pos, dir, grid);
        dir = rotate(rotations[k], dir);
    }
    (pos, dir) = update_pos_2(steps[steps.len() - 1], pos, dir, grid);

    // Face
    let mut dir_val: i64 = 0;
    if dir.re == 1 {
        dir_val = 1;
    }
    if dir.im == -1 {
        dir_val = 2;
    }
    if dir.re == -1 {
        dir_val = 3;
    }

    Some((pos.re + 1) * 1000 + (pos.im + 1) * 4 + dir_val)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (grid, steps, rotations) = parse(input);

    let mut pos: Complex<i64> = Complex::new(0, 0);
    for j in 0..grid[0].len() {
        if grid[0][j] == '.' {
            pos = Complex::new(0, j as i64);
            break;
        }
    }

    simulate_2(pos, &grid, steps, rotations)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("inputs", 22);
        assert_eq!(part_two(&input), Some(37415));
    }
}
