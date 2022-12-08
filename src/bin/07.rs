use indextree::{Arena, NodeId};

#[derive(Clone, Debug)]
pub struct Entry<'a> {
    name: &'a str,
    is_dir: bool,
    size: u64,
}

type Input<'a> = Option<Arena<Entry<'a>>>;

pub fn parse(input: &str) -> Input {
    let mut arena = Arena::new();
    let mut curr_dir = arena.new_node(Entry {
        name: "/",
        is_dir: true,
        size: 0,
    });

    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        if line[0] == "$" && line[1] == "cd" {
            match line[2] {
                ".." => {
                    curr_dir = arena.get(curr_dir)?.parent()?;
                }
                "/" => {
                    continue;
                }
                dir_name => {
                    curr_dir = curr_dir
                        .children(&arena)
                        .find(|&id| arena.get(id).unwrap().get().name == dir_name)?;
                }
            }
        }

        if line[0] == "$" {
            continue;
        }

        if line[0] == "dir" {
            let new_entry: NodeId = arena.new_node(Entry {
                name: line[1],
                is_dir: true,
                size: 0,
            });
            curr_dir.append(new_entry, &mut arena);
            continue;
        }
        let val = line[0].parse::<u64>().unwrap();
        let new_entry: NodeId = arena.new_node(Entry {
            name: line[1],
            is_dir: false,
            size: val,
        });
        curr_dir.append(new_entry, &mut arena);

        // Add size to parents
        let parents: Vec<NodeId> = curr_dir.ancestors(&arena).collect();
        for parent_id in parents.into_iter() {
            arena.get_mut(parent_id).unwrap().get_mut().size += val;
        }
    }

    return Some(arena);
}

pub fn part_one(input: &str) -> Option<u64> {
    let tree = parse(input).expect("Error creating the tree");
    let mut res = 0;
    for entry in tree.iter() {
        if entry.get().is_dir && entry.get().size < 100_000 {
            res += entry.get().size;
        }
    }

    return Some(res);
}

pub fn part_two(input: &str) -> Option<u64> {
    let tree = parse(input).expect("Error creating the tree");
    let mut sizes: Vec<u64> = Vec::new();
    for entry in tree.iter() {
        if entry.get().is_dir {
            sizes.push(entry.get().size);
        }
    }
    sizes.sort();
    let unused_space: u64 = 70_000_000 - sizes.last().unwrap();

    for size in sizes {
        if unused_space + size >= 30_000_000 {
            return Some(size);
        }
    }

    return None;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
