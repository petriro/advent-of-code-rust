// On each line, the calibration value can be found by combining the first digit
// and the last digit (in that order) to form a single two-digit number.

// Your calculation isn't quite right. It looks like some of the digits are
// actually spelled out with letters: one, two, three, four, five, six,
// seven, eight, and nine also count as valid "digits".

pub fn part_one(input: &str) -> Option<u32> {
    const RADIX: u32 = 10;

    // 1abc2
    // This is a list of chars. Is the input 
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


fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
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
}
