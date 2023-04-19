use itertools::{max, sorted};

pub fn part_one(input: &str) -> Option<u32> {
    let mut cur: u32 = 0;
    let mut elfs = vec![];
    for i in input.lines() {
        if i.is_empty() {
            elfs.push(cur);
            cur = 0;
        } else {
            cur += i.parse::<u32>().unwrap();
        }
    }

    max(elfs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cur: u32 = 0;
    let mut elfs = vec![];
    for i in input.lines() {
        if i.is_empty() {
            elfs.push(cur);
            cur = 0;
        } else {
            cur += i.parse::<u32>().unwrap();
        }
    }

    Some(sorted(elfs).rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(72478));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(210367));
    }
}
