use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op_value: String,
    op_type: char,
    test_val: u64,
    test_true: usize,
    test_false: usize,
    count: u64,
}

impl Monkey {
    fn operation(&self, val: u64, module: u64, div: bool) -> u64 {
        let mut value: u64 = val;
        let value_op: u64 = self.op_value.parse::<u64>().unwrap_or(val);

        if self.op_type == '*' {
            value *= value_op;
        } else if self.op_type == '+' {
            value += value_op;
        } else {
            panic!("Operation not defined");
        }
        if div {
            value /= 3;
        }
        value % module
    }

    fn throw(&self, val: u64) -> usize {
        if val % self.test_val == 0 {
            return self.test_true;
        }
        return self.test_false;
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_info in input.split("\n\n") {
        let lines: Vec<&str> = monkey_info.lines().collect();

        // Get stack
        let items: VecDeque<u64> = lines[1].split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();

        // Get operation value and type
        let mut line: Vec<&str> = lines[2].split(' ').collect();
        let op_value: String = line.pop().unwrap().trim().to_string();
        let op_type: char = line.pop().unwrap().trim().chars().collect::<Vec<char>>()[0];

        // Get test
        let test_val: u64 = lines[3]
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();
        let test_true: usize = lines[4]
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let test_false: usize = lines[5]
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey {
            items,
            op_value,
            op_type,
            test_val,
            test_true,
            test_false,
            count: 0,
        });
    }

    return monkeys;
}

fn turn(monkeys: &mut Vec<Monkey>, i: usize, module: u64, div: bool) {
    while !monkeys[i].items.is_empty() {
        monkeys[i].count += 1;
        let mut item: u64 = monkeys[i].items.pop_front().unwrap();
        item = monkeys[i].operation(item, module, div);
        let target: usize = monkeys[i].throw(item);
        monkeys[target].items.push_back(item);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = parse_input(input);
    let mut module: u64 = 1;
    for monkey in monkeys.iter() {
        module *= monkey.test_val;
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            turn(&mut monkeys, i, module, true);
        }
    }

    let mut counts: Vec<u64> = Vec::new();
    for monkey in monkeys {
        counts.push(monkey.count);
    }

    counts.sort();

    return Some(counts[counts.len() - 1] * counts[counts.len() - 2]);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = parse_input(input);
    let mut module: u64 = 1;
    for monkey in monkeys.iter() {
        module *= monkey.test_val;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            turn(&mut monkeys, i, module, false);
        }
    }

    let mut counts: Vec<u64> = Vec::new();
    for monkey in monkeys {
        counts.push(monkey.count);
    }

    counts.sort();

    return Some(counts[counts.len() - 1] * counts[counts.len() - 2]);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
