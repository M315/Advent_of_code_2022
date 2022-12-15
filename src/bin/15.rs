use std::cmp::max;

fn merge_intervals(intervals: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    //Sort the intervals by the start point
    intervals.sort();

    let mut result: Vec<(i64, i64)> = Vec::new();
    let mut curr: (i64, i64) = intervals[0];

    for i in 1..intervals.len() {
        if intervals[i].0 <= curr.1 {
            curr.1 = max(curr.1, intervals[i].1);
        } else {
            result.push(curr);
            curr = intervals[i];
        }
    }

    result.push(curr);

    return result;
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let y: i64 = lines[0].split('=').collect::<Vec<&str>>()[1]
        .parse::<i64>()
        .unwrap();

    let mut intervals: Vec<(i64, i64)> = Vec::new();

    for k in 1..lines.len() {
        let line: Vec<Vec<i64>> = lines[k]
            .split(':')
            .map(|y| {
                y.split(',')
                    .map(|x| {
                        x.split('=').collect::<Vec<&str>>()[1]
                            .parse::<i64>()
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        let radius: i64 = (line[0][0] - line[1][0]).abs() + (line[0][1] - line[1][1]).abs();
        let dist: i64 = (line[0][1] - y).abs();

        if radius >= dist {
            intervals.push((line[0][0] - (radius - dist), line[0][0] + (radius - dist)));
        }
    }

    intervals = merge_intervals(&mut intervals);

    let mut sum: u64 = 0;
    for interval in intervals {
        sum += (interval.1 - interval.0) as u64;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut y: i64 = lines[0].split('=').collect::<Vec<&str>>()[1]
        .parse::<i64>()
        .unwrap();

    let mut values: Vec<Vec<Vec<i64>>> = Vec::new();

    for k in 1..lines.len() {
        values.push(
            lines[k]
                .split(':')
                .map(|y| {
                    y.split(',')
                        .map(|x| {
                            x.split('=').collect::<Vec<&str>>()[1]
                                .parse::<i64>()
                                .unwrap()
                        })
                        .collect()
                })
                .collect::<Vec<Vec<i64>>>(),
        );
    }

    while y < 4_000_000 {
        let mut intervals: Vec<(i64, i64)> = Vec::new();
        for value in values.iter() {
            let radius: i64 = (value[0][0] - value[1][0]).abs() + (value[0][1] - value[1][1]).abs();
            let dist: i64 = (value[0][1] - y).abs();

            if radius >= dist {
                intervals.push((value[0][0] - (radius - dist), value[0][0] + (radius - dist)));
            }
        }
        intervals = merge_intervals(&mut intervals);

        if intervals.len() > 1 {
            return Some((4_000_000 * (intervals[0].1 as u64 + 1)) + y as u64);
        }

        y += 1;
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
