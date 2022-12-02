pub fn round_score_1(line : Vec<char>) -> Option<u32> {
        let op : u32 = line[0] as u32 - 'A' as u32;
        let you : u32 = line[2] as u32 - 'X' as u32;

        let score_matrix : Vec<Vec<u32>> = vec![vec![3, 6, 0], vec![0, 3, 6], vec![6, 0, 3]];

        return Some(you + 1 + score_matrix[op as usize][you as usize]);
}

pub fn part_one(input: &str) -> Option<u32> {
    let score : u32 = input.lines().fold(0, |acc, line| acc + round_score_1(line.chars().collect::<Vec<char>>()).expect("Error computing round score"));
    Some(score)
}

pub fn round_score_2(line : Vec<char>) -> Option<u32> {
        let op : u32 = line[0] as u32 - 'A' as u32;
        let you : u32 = line[2] as u32 - 'X' as u32;

        let score_matrix : Vec<Vec<u32>> = vec![vec![3, 1, 2], vec![1, 2, 3], vec![2, 3, 1]];

        return Some((you * 3) + score_matrix[op as usize][you as usize]);
}

pub fn part_two(input: &str) -> Option<u32> {
    let score : u32 = input.lines().fold(0, |acc, line| acc + round_score_2(line.chars().collect::<Vec<char>>()).expect("Error computing round score"));
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
