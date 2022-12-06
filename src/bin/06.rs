fn solver(input: &str, n: usize) -> Option<u32> {
    let input: Vec<char> = input.chars().collect();

    for i in 0..input.len() - n - 1 {
        let mut curr: Vec<bool> = vec![true; 26];
        let mut sol: bool = true;
        for j in 0..n {
            if curr[input[i + j] as usize - 97] {
                curr[input[i + j] as usize - 97] = false;
            } else {
                sol = false;
                break;
            }
        }

        if sol {
            return Some(i as u32 + n as u32);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solver(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solver(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
