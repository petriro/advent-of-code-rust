// On each line, the calibration value can be found by combining the first digit
// and the last digit (in that order) to form a single two-digit number.

// Your calculation isn't quite right. It looks like some of the digits are
// actually spelled out with letters: one, two, three, four, five, six,
// seven, eight, and nine also count as valid "digits".
const RADIX: u32 = 10;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let iter = line.chars();
        let mut created_number = (None, None);
        for num in iter {
            if num.is_numeric() {
                if created_number.0.is_none() {
                    created_number.0 = Some(num);
                    created_number.1 = Some(num);
                } else {
                    created_number.1 = Some(num);
                }
            }
        }

        if created_number.1.is_none() {
            sum = sum + created_number.0.unwrap().to_digit(RADIX).unwrap();
        } else {

        }
        sum = sum + created_number.0.unwrap().to_digit(RADIX).unwrap() * 10 + created_number.1.unwrap().to_digit(RADIX).unwrap();
    }

    return Some(sum);
}

pub fn get_number(char_index: usize, line: &str) -> Option<u32>{
    // 3, 4, 5 chars
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5), 
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0)
    ];

    let substring = &line[char_index..];
    for (num_str, num) in  numbers {
        if substring.starts_with(num_str) {
            println!("{}", substring);
            return Some(num);
        }
    }
    
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let iter = line.char_indices();
        let mut created_number = (None, None);
        for (char_index, num) in iter {
            if num.is_numeric() {
                if created_number.0.is_none() {
                    created_number.0 = num.to_digit(RADIX);
                    created_number.1 = num.to_digit(RADIX);
                } else {
                    created_number.1 = num.to_digit(RADIX);
                }
            } else {
                let alpha_number = get_number(char_index, line);
                if alpha_number.is_some() {
                    if created_number.0.is_none() {
                        created_number.0 = alpha_number;
                        created_number.1 = alpha_number;
                    } else {
                        created_number.1 = alpha_number;
                    }
                }
            }
        }

        if created_number.1.is_none() {
            sum = sum + created_number.0.unwrap();
        } else {

        }
        sum = sum + created_number.0.unwrap() * 10 + created_number.1.unwrap();
    }

    return Some(sum);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(1, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        let output = Some(142);
        assert_eq!(part_one(&input), output);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples_pt_2", 1);
        let output = Some(281);
        assert_eq!(part_two(&input), output);
    }
}
