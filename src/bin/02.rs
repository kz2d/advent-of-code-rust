use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PAY: HashMap<&'static str, u32> = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6)
    ]);
}

lazy_static! {
    static ref PAY2: HashMap<&'static str, u32> = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7)
    ]);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0u32;
    for line in input.lines() {
        score += PAY.get(line).unwrap();
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0u32;
    for line in input.lines() {
        score += PAY2.get(line).unwrap();
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15523));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(15702));
    }
}
