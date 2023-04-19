pub fn part_one(input: &str) -> Option<u32> {
    let mut alphabet = [0u32; 300];
    let mut counter = 0;

    for i in 0..input.len() {
        alphabet[input.as_bytes()[i] as usize] += 1;

        counter += if alphabet[input.as_bytes()[i] as usize] == 1 {
            1
        } else {
            0
        };

        if i > 3 {
            alphabet[input.as_bytes()[i-4] as usize] -= 1;
            counter -= if alphabet[input.as_bytes()[i-4] as usize] == 0 {
            1
        } else {
            0
        };
        }
        if counter == 4 {
            return Some(i as u32 + 1);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut alphabet = [0u32; 300];
    let mut counter = 0;

    for i in 0..input.len() {
        alphabet[input.as_bytes()[i] as usize] += 1;

        counter += if alphabet[input.as_bytes()[i] as usize] == 1 {
            1
        } else {
            0
        };

        if i > 13 {
            alphabet[input.as_bytes()[i-14] as usize] -= 1;
            counter -= if alphabet[input.as_bytes()[i-14] as usize] == 0 {
            1
        } else {
            0
        };
        }
        if counter == 14 {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn est_part_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_two(input), None);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
