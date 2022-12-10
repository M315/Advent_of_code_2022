fn parse_input(input: &str) -> Vec<i32> {
    let mut x: Vec<i32> = vec![1, 1];

    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let state: i32 = *x.last().expect("X is empty!");
        if line.len() == 1 {
            x.push(state);
        } else {
            x.push(state);
            x.push(state + line[1].parse::<i32>().unwrap());
        }
    }

    return x;
}

pub fn part_one(input: &str) -> Option<i32> {
    let x: Vec<i32> = parse_input(input);
    let mut sum: i32 = 0;
    let mut i: usize = 20;

    while i <= 220 {
        sum += i as i32 * x[i];
        i += 40;
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<i32> {
    let x: Vec<i32> = parse_input(input);
    let mut ctr: Vec<Vec<char>> = vec![vec![' '; 40]; 6];

    for i in 0..6 {
        for j in 0..40 {
            let pos: i32 = x[i * 40 + j + 1];
            if (pos - j as i32).abs() <= 1 {
                ctr[i][j] = '@';
            }
        }
    }

    for row in ctr {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    return Some(1);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(1));
    }
}
