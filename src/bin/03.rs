use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for i in input.lines() {
        let (s1, s2) = i.split_at(i.len() / 2);
        let z = HashSet::<_>::from_iter(s1.chars())
            .intersection(&HashSet::from_iter(s2.chars()))
            .take(1)
            .copied()
            .collect_vec()[0] as u32;
        if z > 'a' as u32 {
            sum += z - 'a' as u32 + 1;
        } else {
            sum += z - 'A' as u32 + 27;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut it = input.lines();
    loop {
        let s1 = it.next();
        if s1.is_none() {
            break;
        }
        let s1 = HashSet::<_>::from_iter(s1.unwrap().chars());
        let s2 = HashSet::<_>::from_iter(it.next().unwrap().chars());
        let s3 = HashSet::<_>::from_iter(it.next().unwrap().chars());

        let z = s1
            .intersection(&s2)
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&s3)
            .take(1)
            .copied()
            .collect_vec()[0] as u32;
        if z > 'a' as u32 {
            sum += z - 'a' as u32 + 1;
        } else {
            sum += z - 'A' as u32 + 27;
        }
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn est_part_one() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        println!("{input}");
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
