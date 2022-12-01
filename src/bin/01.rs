use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let input : Vec<&str> = input.split('\n').map(|x: &str| x.trim()).collect();

    let mut curr : u32 = 0;
    let mut maximum : u32 = 0;
    
    for num in input {
        let val : Result<u32, std::num::ParseIntError> = num.parse::<u32>();
        if val.is_ok() {
            curr += val.unwrap();
        } else {
            maximum = max(maximum, curr);
            curr = 0;
        }
    }

    return Some(maximum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let input : Vec<&str> = input.split('\n').map(|x: &str| x.trim()).collect();

    let mut food : Vec<u32> = Vec::new();
    let mut curr : u32 = 0;
    
    for num in input {
        let val : Result<u32, std::num::ParseIntError> = num.parse::<u32>();
        if val.is_ok() {
            curr += val.unwrap();
        } else {
            food.push(curr);
            curr = 0;
        }
    }

    food.sort();
    food.reverse();

    let ans : u32 = food.into_iter().take(3).sum();

    return Some(ans);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 01);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(71924));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(210406));
    }
}
