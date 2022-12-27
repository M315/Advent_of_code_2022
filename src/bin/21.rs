use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Monkey<'a> {
    value: Option<i64>,
    left: Option<&'a str>,
    right: Option<&'a str>,
    operation: Option<&'a str>,
}

fn parse(input: &str) -> HashMap<&str, Monkey> {
    let mut map: HashMap<&str, Monkey> = HashMap::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let pred: Vec<&str> = line[1].trim().split(' ').collect();
        match pred.len() {
            1 => {
                map.insert(
                    line[0].trim(),
                    Monkey {
                        value: Some(pred[0].trim().parse::<i64>().unwrap()),
                        left: None,
                        right: None,
                        operation: None,
                    },
                );
            }
            3 => {
                map.insert(
                    line[0].trim(),
                    Monkey {
                        value: None,
                        left: Some(pred[0].trim()),
                        right: Some(pred[2].trim()),
                        operation: Some(pred[1].trim()),
                    },
                );
            }
            _ => continue,
        }
    }

    return map;
}

fn get_value(monkey_name: &str, monkeys: &mut HashMap<&str, Monkey>) -> Option<i64> {
    let mut monkey: Monkey = *monkeys.get_mut(monkey_name).expect("Monkey doesn't exist");
    if monkey.value.is_some() {
        return monkey.value;
    }

    let left_value: i64 = get_value(monkey.left.unwrap(), monkeys)?;
    let right_value: i64 = get_value(monkey.right.unwrap(), monkeys)?;
    match monkey.operation.unwrap() {
        "+" => monkey.value = Some(left_value + right_value),
        "-" => monkey.value = Some(left_value - right_value),
        "*" => monkey.value = Some(left_value * right_value),
        "/" => monkey.value = Some(left_value / right_value),
        _ => panic!("Invalid operation"),
    }

    monkey.value
}

fn dependency(monkey_name: &str, monkeys: &mut HashMap<&str, Monkey>) -> bool {
    let monkey: Monkey = *monkeys.get_mut(monkey_name).expect("Monkey doesn't exist");

    if monkey_name == "humn" {
        return true;
    }

    if monkey.value.is_some() {
        return false;
    }

    return dependency(monkey.left.unwrap(), monkeys) || dependency(monkey.right.unwrap(), monkeys);
}

fn find_value(target: i64, monkey_name: &str, monkeys: &mut HashMap<&str, Monkey>) -> Option<i64> {
    let mut monkey: Monkey = *monkeys.get_mut(monkey_name).expect("Monkey doesn't exist");

    if !dependency(monkey_name, monkeys) {
        println!("{}", monkey_name);
        return None;
    }

    if monkey_name == "humn" {
        println!("Target: {}", target);
        return Some(target);
    }

    let mut return_value: Option<i64> = None;
    if !dependency(monkey.left.unwrap(), monkeys) {
        let value = get_value(monkey.left.unwrap(), monkeys).unwrap();
        match monkey.operation.unwrap() {
            "+" => {
                return_value = find_value(target - value, monkey.right.unwrap(), monkeys);
            }
            "-" => {
                return_value = find_value(value - target, monkey.right.unwrap(), monkeys);
            }
            "*" => {
                return_value = find_value(target / value, monkey.right.unwrap(), monkeys);
            }
            "/" => {
                return_value = find_value(value / target, monkey.right.unwrap(), monkeys);
            }
            _ => panic!("Invalid operation"),
        }
    } else {
        let value = get_value(monkey.right.unwrap(), monkeys).unwrap();
        match monkey.operation.unwrap() {
            "+" => {
                return_value = find_value(target - value, monkey.left.unwrap(), monkeys);
            }
            "-" => {
                return_value = find_value(value + target, monkey.left.unwrap(), monkeys);
            }
            "*" => {
                return_value = find_value(target / value, monkey.left.unwrap(), monkeys);
            }
            "/" => {
                return_value = find_value(target * value, monkey.left.unwrap(), monkeys);
            }
            _ => panic!("Invalid operation"),
        }
    }

    return_value
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys: HashMap<&str, Monkey> = parse(input);
    get_value("root", &mut monkeys)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys: HashMap<&str, Monkey> = parse(input);

    let root: Monkey = *monkeys.get("root").unwrap();
    let mut target: i64 = 0;
    if !dependency(root.left.unwrap(), &mut monkeys) {
        target = get_value(root.left.unwrap(), &mut monkeys)?;
    } else {
        target = get_value(root.right.unwrap(), &mut monkeys)?;
    }

    find_value(2 * target, "root", &mut monkeys)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
