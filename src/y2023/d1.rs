//! --- Day 1: Trebuchet?! ---

pub fn part1(input: String) -> String {
    input.lines().map(|l| parse_line(l)).sum::<u32>().to_string()
}

/// Gets the first and last digit in a string and combines them into a u32
fn parse_line(s: &str) -> u32 {
    let digits = s.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let first = digits.chars().next().expect("No digits found in string");
    let last = digits.chars().last().expect("No digits found in string");
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

pub fn part2(input: String) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    static RESULT1: &str = "142";
    static RESULT2: &str = "142";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), RESULT1.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), RESULT2.to_string());
    }
}
