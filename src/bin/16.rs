use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input<'a>(
    input: &'a str,
    graph: &mut HashMap<&'a str, HashMap<&'a str, i64>>,
    flow: &mut HashMap<&'a str, i64>,
) -> HashMap<&'a str, usize> {
    let mut g: HashMap<&str, Vec<&str>> = HashMap::new();

    let re = Regex::new(r"\d+").unwrap();
    let re2 = Regex::new(r"[[:upper:]]{2}").unwrap();
    for line in input.lines() {
        if line == "" {
            break;
        }
        let f = re.find(line).unwrap().as_str().parse::<i64>().unwrap();
        let valves = re2
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();
        g.insert(valves[0], valves[1..].to_vec());
        flow.insert(valves[0], f);
    }

    let mut non_empty: Vec<&str> = Vec::new();

    for node in g.keys() {
        if *node != "AA" && *flow.get(node).unwrap() == 0 {
            continue;
        }

        if *node != "AA" {
            non_empty.push(node);
        }

        graph.insert(node, HashMap::new());

        graph.get_mut(node).unwrap().insert(node, 0);
        graph.get_mut(node).unwrap().insert("AA", 0);

        let mut q: VecDeque<(i64, &str)> = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();

        visited.insert(node);

        q.push_back((0, node));

        while let Some((dist, pos)) = q.pop_front() {
            for neighbor in g.get(pos).unwrap() {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                if *flow.get(neighbor).unwrap() != 0 {
                    graph.get_mut(node).unwrap().insert(neighbor, dist + 1);
                }
                q.push_back((dist + 1, neighbor));
            }
        }

        graph.get_mut(node).unwrap().remove(node);
        graph.get_mut(node).unwrap().remove("AA");
    }

    let mut indices: HashMap<&str, usize> = HashMap::new();
    for i in 0..non_empty.len() {
        indices.insert(non_empty[i], i);
    }

    indices
}

fn dfs(
    node: &str,
    time: i64,
    bitmask: u64,
    graph: &HashMap<&str, HashMap<&str, i64>>,
    flow: &HashMap<&str, i64>,
    cache: &mut HashMap<(String, u64, i64), i64>,
    indices: &HashMap<&str, usize>,
) -> Option<i64> {
    if let Some(&ans) = cache.get(&(node.to_string(), bitmask, time)) {
        return Some(ans);
    }
    let mut best = 0;
    for (neighbor, travel_time) in graph.get(node).unwrap() {
        let bit: u64 = 1 << indices[neighbor];
        if bitmask & bit != 0 {
            continue;
        }

        let remaining: i64 = time - travel_time - 1;
        if remaining <= 0 {
            continue;
        }
        let sub_result = dfs(
            neighbor,
            remaining,
            bitmask | bit,
            graph,
            flow,
            cache,
            indices,
        )?;
        best = best.max(sub_result + *flow.get(neighbor).unwrap() * remaining);
    }
    cache.insert((node.to_string(), bitmask, time), best);
    Some(best)
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut graph: HashMap<&str, HashMap<&str, i64>> = HashMap::new();
    let mut flow: HashMap<&str, i64> = HashMap::new();

    let indices = parse_input(input, &mut graph, &mut flow);

    let mut cache = HashMap::new();

    dfs("AA", 30, 0, &graph, &flow, &mut cache, &indices)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut graph: HashMap<&str, HashMap<&str, i64>> = HashMap::new();
    let mut flow: HashMap<&str, i64> = HashMap::new();

    let indices = parse_input(input, &mut graph, &mut flow);

    let mut cache = HashMap::new();
    let b = (1 << indices.len()) - 1;
    let mut m: i64 = 0;

    for i in 0..((b + 1) / 2) {
        m = m.max(
            dfs("AA", 26, i, &graph, &flow, &mut cache, &indices).unwrap()
                + dfs("AA", 26, b ^ i, &graph, &flow, &mut cache, &indices).unwrap(),
        );
    }

    Some(m)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
