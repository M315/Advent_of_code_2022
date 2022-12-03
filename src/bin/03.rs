pub fn get_score_1(line: Vec<char>) -> Option<u32> {
    let mut first: Vec<bool> = vec![false; 60];
    let n: usize = line.len() / 2;

    for i in 0..n {
        first[line[i] as usize - 65] = true;
    }

    for i in n..2 * n {
        if first[line[i] as usize - 65] {
            if line[i].is_lowercase() {
                return Some(line[i] as u32 - 96);
            } else {
                return Some(line[i] as u32 - 38);
            }
        }
    }

    return None;
}

pub fn get_score_2(lines: Vec<Vec<char>>) -> Option<u32> {
    let mut contains: Vec<Vec<bool>> = vec![vec![false; 130]; 3];

    for i in 0..3 {
        for j in 0..lines[i].len() {
            contains[i][lines[i][j] as usize] = true;
        }
    }

    for j in 0..contains[0].len() {
        if contains[0][j] && contains[1][j] && contains[2][j] {
            if j > 96 {
                return Some(j as u32 - 96);
            } else {
                return Some(j as u32 - 38);
            }
        }
    }

    return None;
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(input.lines().fold(0, |acc, line| {
        acc + get_score_1(line.chars().collect::<Vec<char>>())
            .expect("Error while computing the score")
    }));
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    let mut i: usize = 0;

    while i < input.len() {
        let mut v: Vec<Vec<char>> = Vec::new();
        for _ in 0..3 {
            v.push(input[i].chars().collect());
            i += 1;
        }

        sum += get_score_2(v).expect("Error computing the score");
    }

    println!("{}", sum);
    return Some(sum);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
