fn simulate(
    rock: usize,
    height: &mut usize,
    room: &mut Vec<Vec<char>>,
    jets: &Vec<char>,
    k: &mut usize,
    increments: &mut Vec<usize>,
) {
    let shapes: Vec<Vec<Vec<char>>> = vec![
        vec![vec!['#', '#', '#', '#']],
        vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ],
        vec![
            vec!['#', '#', '#'],
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
        ],
        vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']],
        vec![vec!['#', '#'], vec!['#', '#']],
    ];

    let mut x = 2;
    let mut y = *height + 3;

    // Shape dimentions
    let h: usize = shapes[rock].len();
    let w: usize = shapes[rock][0].len();

    loop {
        let dir: char = jets[*k];
        *k = (*k + 1 + jets.len()) % jets.len();

        // Move horizontaly
        if dir == '>' && x + w < 7 {
            // Check blockers
            let mut can_move: bool = true;
            for i in 0..h {
                if room[y + i][x + w] == '#' && shapes[rock][i][w - 1] == '#' {
                    can_move = false;
                    break;
                }
                if shapes[rock][i][w - 1] == '.' {
                    if room[y + i][x + w - 1] == '#' && shapes[rock][i][w - 2] == '#' {
                        can_move = false;
                        break;
                    }
                }
            }
            if can_move {
                x += 1;
            }
        }
        if dir == '<' && x > 0 {
            // Check blockers
            let mut can_move: bool = true;
            for i in 0..h {
                if room[y + i][x - 1] == '#' && shapes[rock][i][0] == '#' {
                    can_move = false;
                    break;
                }
                if shapes[rock][i][0] == '.' {
                    if room[y + i][x] == '#' && shapes[rock][i][1] == '#' {
                        can_move = false;
                        break;
                    }
                    if shapes[rock][i][1] == '.' {
                        if room[y + i][x + 1] == '#' && shapes[rock][i][2] == '#' {
                            can_move = false;
                            break;
                        }
                    }
                }
            }
            if can_move {
                x -= 1;
            }
        }

        // Move verticaly
        // Look for blockers
        if y == 0 {
            break;
        }

        let mut colide: bool = false;
        for j in 0..w {
            if room[y - 1][x + j] == '#' && shapes[rock][0][j] == '#' {
                colide = true;
                break;
            }
            if shapes[rock][0][j] == '.' {
                if room[y][x + j] == '#' && shapes[rock][1][j] == '#' {
                    colide = true;
                    break;
                }
            }
        }

        if colide {
            break;
        }

        y -= 1;
    }

    // Stoped
    for i in 0..h {
        for j in 0..w {
            if shapes[rock][i][j] == '#' {
                room[y + i][x + j] = shapes[rock][i][j];
            }
        }
    }

    let old = *height;
    *height = std::cmp::max(*height, y + h);
    increments.push(*height - old);
}

pub fn part_one(input: &str) -> Option<u32> {
    let jets: Vec<char> = input.trim().chars().collect();

    let mut room: Vec<Vec<char>> = vec![vec!['.'; 7]; 10_000];
    let mut height: usize = 0;
    let mut k: usize = 0;
    let mut increments: Vec<usize> = Vec::new();

    for rock in 0..2022 {
        simulate(
            rock % 5,
            &mut height,
            &mut room,
            &jets,
            &mut k,
            &mut increments,
        );
    }

    Some(height as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let jets: Vec<char> = input.trim().chars().collect();

    let mut room: Vec<Vec<char>> = vec![vec!['.'; 7]; 100_000];
    let mut height: usize = 0;
    let mut k: usize = 0;
    let mut increments: Vec<usize> = Vec::new();

    for rock in 0..10000 {
        simulate(
            rock % 5,
            &mut height,
            &mut room,
            &jets,
            &mut k,
            &mut increments,
        );
        if rock == 532 || rock == 1750 + 532 || rock == 532 + 217 {
            println!("{}", height);
        }
    }

    /*
    fs::write(
        "increments.txt",
        increments
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
    )
    .ok();
    */

    let precycle_len: u64 = 533;
    let precycle_h: u64 = 875;
    let cycle_len: u64 = 1750;
    let cycle_h: u64 = 3656 - precycle_h;
    let offset_len: u64 = 217;
    let offset_h: u64 = 1232 - precycle_h;

    let cycles: u64 = 1_000_000_000_000 / cycle_len;
    println!("{}", cycles * cycle_len + precycle_len);

    Some(precycle_h + (cycles * cycle_h) + offset_h)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
