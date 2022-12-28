fn snafu_to_dec(num: &str) -> i64 {
    let mut mult: i64 = 1;
    let mut res: i64 = 0;

    for n in num.chars().rev() {
        match n {
            '2' => res += 2 * mult,
            '1' => res += 1 * mult,
            '0' => res += 0 * mult,
            '-' => res += (-1) * mult,
            '=' => res += (-2) * mult,
            _ => panic!("Unexpected digit!"),
        }
        mult *= 5;
    }

    return res;
}

fn dec_to_snafu(mut num: i64) -> String {
    let mut res: String = String::new();

    while num > 0 {
        match num.rem_euclid(5) {
            0 => {
                res.push('0');
                num = num / 5;
            }
            1 => {
                res.push('1');
                num = (num - 1) / 5;
            }
            2 => {
                res.push('2');
                num = (num - 2) / 5;
            }
            3 => {
                res.push('=');
                num = (num + 2) / 5;
            }
            4 => {
                res.push('-');
                num = (num + 1) / 5;
            }
            _ => panic!("Error with the reminder"),
        }
    }

    return res.chars().rev().collect();
}

pub fn part_one(input: &str) -> String {
    dec_to_snafu(
        input
            .lines()
            .map(|x| snafu_to_dec(x))
            .fold(0, |acc, x| acc + x),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    println!("{}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), "2=-1=0");
    }
}
