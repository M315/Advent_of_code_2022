#![feature(iter_array_chunks)]
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Bytes;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Packet {
    List(Vec<Packet>),
    Num(u8),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let slice_cmp = <[Packet]>::cmp;
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => slice_cmp(l, r),
            (Packet::List(l), &Packet::Num(r)) => slice_cmp(l, &[Packet::Num(r)]),
            (&Packet::Num(l), Packet::List(r)) => slice_cmp(&[Packet::Num(l)], r),
        }
    }
}

impl Packet {
    fn parse(s: &str) -> Self {
        parse_impl(&mut s[1..].bytes().peekable())
    }
}

fn parse_impl(it: &mut Peekable<Bytes<'_>>) -> Packet {
    let mut list = Vec::new();
    while let Some(b) = it.next() {
        match b {
            // Encountered a number, make sure to include any following digits
            // and append the result to our current list.
            b'0'..=b'9' => {
                let mut num = b - b'0';
                while let Some(b'0'..=b'9') = it.peek() {
                    num = 10 * num + (it.next().unwrap() - b'0');
                }
                list.push(Packet::Num(num));
            }
            // Encountered a nested list, recurse to parse it and append the
            // result to our current list.
            b'[' => list.push(parse_impl(it)),
            // Finished parsing the list, so return it
            b']' => return Packet::List(list),
            // Just continue to the next number or list
            b',' => continue,
            // The end of the input, return the single element in the list
            b => panic!("unexpected byte `{b:?}`"),
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> Vec<Packet> {
    input
        .split_whitespace()
        .map(Packet::parse)
        .collect::<Vec<Packet>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let packets: Vec<Packet> = parse_input(input);

    Some(
        packets
            .into_iter()
            .array_chunks()
            .enumerate()
            .map(|(i, [left, right])| match left.cmp(&right) {
                Ordering::Less => i as u32 + 1,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets: Vec<Packet> = parse_input(input);

    packets.sort();
    let i = packets.binary_search(&Packet::parse("[[2]]")).unwrap_err() + 1;
    let j = packets.binary_search(&Packet::parse("[[6]]")).unwrap_err() + 2;
    Some(i * j)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
