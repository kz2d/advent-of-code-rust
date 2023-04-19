use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for i in input.lines() {
        let ((st1, en1), (st2, en2)): ((u32, u32), (u32, u32)) = i
            .split(",")
            .take(2)
            .collect_vec()
            .iter()
            .map(|s| {
                s.split("-")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        if st1 <= st2 && en1 >= en2 {
            sum += 1;
        } else if st1 >= st2 && en1 <= en2 {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for i in input.lines() {
        let ((st1, en1), (st2, en2)): ((u32, u32), (u32, u32)) = i
            .split(",")
            .take(2)
            .collect_vec()
            .iter()
            .map(|s| {
                s.split("-")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        if st1 <= st2 && en1 >= st2 {
            sum += 1;
        } else if st1 <= en2 && en1 >= en2 {
            sum += 1;
        } else if st1 <= st2 && en1 >= en2 {
            sum += 1;
        } else if st1 >= st2 && en1 <= en2 {
            sum += 1;
        }
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn est_part_one() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
