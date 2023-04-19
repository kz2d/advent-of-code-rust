use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks: [Vec<char>; 9] = Default::default();
    let mut it = input.lines();
    loop {
        let i = it.next().unwrap();
        if i.is_empty() {
            break;
        }

        for (index, z) in i
            .chars()
            .enumerate()
            .filter_map(|(index, s)| {
                if index > 0 && (index - 1) % 4 == 0 {
                    Some(s)
                } else {
                    None
                }
            })
            .enumerate()
        {
            if z == ' ' {
                continue;
            }
            stacks[index].push(z);
        }
    }
    for i in &mut stacks {
        i.reverse();
    }

    loop {
        let i = it.next();
        if i.is_none() {
            break;
        }

        let s: Vec<usize> = i
            .unwrap()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let (mv, fr, to) = (s[0], s[1] - 1, s[2] - 1);

        for _ in 0..mv {
            let c = stacks[fr].pop().unwrap();
            stacks[to].push(c);
        }
    }

    stacks
        .iter()
        .map(|s| s.last().unwrap().to_string())
        .reduce(|ac, el| ac + el.as_str())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks: [Vec<char>; 9] = Default::default();
    let mut it = input.lines();
    loop {
        let i = it.next().unwrap();
        if i.is_empty() {
            break;
        }

        for (index, z) in i
            .chars()
            .enumerate()
            .filter_map(|(index, s)| {
                if index > 0 && (index - 1) % 4 == 0 {
                    Some(s)
                } else {
                    None
                }
            })
            .enumerate()
        {
            if z == ' ' {
                continue;
            }
            stacks[index].push(z);
        }
    }
    for i in &mut stacks {
        i.reverse();
    }

    loop {
        let i = it.next();
        if i.is_none() {
            break;
        }

        let s: Vec<usize> = i
            .unwrap()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let (mv, fr, to) = (s[0], s[1] - 1, s[2] - 1);

        let mut q = VecDeque::new();
        for _ in 0..mv {
            let c = stacks[fr].pop().unwrap();
            q.push_back(c);
        }
        for _ in 0..mv {
            stacks[to].push(q.pop_back().unwrap());
        }
    }

    stacks
        .iter()
        .map(|s| s.last().unwrap().to_string())
        .reduce(|ac, el| ac + el.as_str())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("inputs", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
