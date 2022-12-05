fn parse(input: &str) -> (Vec<Vec<char>>, Vec<&str>) {
    let mut lines: Vec<&str> = input.lines().collect();
    let n: usize = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n];

    lines.reverse();

    loop {
        let line = lines.pop().unwrap().chars().collect::<Vec<char>>();

        if line[1].is_digit(10) {
            break;
        }

        for j in 0..n {
            if line[1 + 4 * j] != ' ' {
                stacks[j].push(line[1 + 4 * j]);
            }
        }
    }

    lines.pop();
    lines.reverse();

    for k in 0..n {
        stacks[k].reverse();
    }

    return (stacks, lines);
}

pub fn part_one<'a>(stacks: &'a Vec<Vec<char>>, lines: &'a Vec<&str>) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = stacks.to_vec();
    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();

        for _ in 0..line[1].parse::<usize>().unwrap() {
            let c: char = stacks[line[3].parse::<usize>().unwrap() - 1]
                .pop()
                .expect("Stack is empty");
            stacks[line[5].parse::<usize>().unwrap() - 1].push(c);
        }
    }

    let mut ret: Vec<char> = Vec::new();
    for mut stack in stacks {
        ret.push(stack.pop().unwrap());
    }

    return Some(ret.into_iter().collect::<String>());
}

pub fn part_two<'b>(stacks: &'b Vec<Vec<char>>, lines: &'b Vec<&str>) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = stacks.to_vec();
    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();

        let mut aux_stack: Vec<char> = Vec::new();
        for _ in 0..line[1].parse::<usize>().unwrap() {
            aux_stack.push(
                stacks[line[3].parse::<usize>().unwrap() - 1]
                    .pop()
                    .expect("Stack is empty"),
            );
        }
        aux_stack.reverse();
        stacks[line[5].parse::<usize>().unwrap() - 1].append(&mut aux_stack);
    }

    let mut ret: Vec<char> = Vec::new();
    for mut stack in stacks {
        ret.push(stack.pop().unwrap());
    }

    return Some(ret.into_iter().collect::<String>());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    let input: (Vec<Vec<char>>, Vec<&str>) = parse(&input);

    use std::time::Instant;

    let timer = Instant::now();
    let result = part_one(&input.0, &input.1);
    let elapsed = timer.elapsed();
    println!(
        "Part one:\n{} (elapsed: {:.2?})",
        result.expect("Error while solving"),
        elapsed
    );

    let timer = Instant::now();
    let result = part_two(&input.0, &input.1);
    let elapsed = timer.elapsed();
    println!(
        "Part two:\n{} (elapsed: {:.2?})",
        result.expect("Error while solving"),
        elapsed
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        let input: (Vec<Vec<char>>, Vec<&str>) = parse(&input);
        assert_eq!(part_one(&input.0, &input.1), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        let input: (Vec<Vec<char>>, Vec<&str>) = parse(&input);
        assert_eq!(part_two(&input.0, &input.1), Some(String::from("MCD")));
    }
}
