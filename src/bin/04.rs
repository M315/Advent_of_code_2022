type Input = Vec<Vec<u32>>;

fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let mut ret: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let mut pair: Vec<u32> = Vec::new();
        let s: Vec<&str> = line.split(',').collect();
        for e in s {
            let sub_split: Vec<&str> = e.split('-').collect();
            for n in sub_split {
                pair.push(n.parse::<u32>().unwrap());
            }
        }
        ret.push(pair);
    }
    return ret;
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut sum = 0;
    for pair in input {
        if pair[0] <= pair[2] && pair[1] >= pair[3] {
            sum += 1;
        } else if pair[2] <= pair[0] && pair[3] >= pair[1] {
            sum += 1;
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut sum = 0;
    for pair in input {
        if pair[0] >= pair[2] && pair[0] <= pair[3] {
            sum += 1;
        } else if pair[2] >= pair[0] && pair[2] <= pair[1] {
            sum += 1;
        } else if pair[1] >= pair[2] && pair[1] <= pair[3] {
            sum += 1;
        } else if pair[3] >= pair[0] && pair[3] <= pair[1] {
            sum += 1;
        }
    }
    return Some(sum);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
