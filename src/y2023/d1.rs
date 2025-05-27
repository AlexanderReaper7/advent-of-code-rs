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
    input.lines().map(|l| parse_line_2(l)).sum::<u32>().to_string()
}

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn parse_line_2(s: &str) -> u32 {
    let s = s.to_string();
    let mut first = 0;
    let mut last = 0;
    let mut i = 0;
    while i < s.chars().count() {
        // if the character is a digit
        if let Some(d) = s.chars().nth(i).unwrap().to_digit(10) {
            if first == 0 {
                first = d;
            } else {
                last = d;
            }
        }
        // if the next characters are a digit as a word
        else {
            for (j, d) in DIGITS.iter().enumerate() {
                if s[i..].starts_with(d) {
                    // remove the digit word from the string so we don't find overlapping words like "twone"
                    //s.replace_range(i..i+d.chars().count(), "");
                    if first == 0 {
                        first = j as u32 + 1;
                    } else {
                        last = j as u32 + 1;
                    }
                    // because we have modified the string, we need to start over without incrementing i
                    //continue 'outer;
                }
            }
        }
        i += 1;
    }
    (first*10 + last) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const RESULT1: &str = "142";
    const INPUT2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    const RESULT2: &str = "281";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT1.to_string()), RESULT1.to_string());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT2.to_string()), RESULT2.to_string());
    }

    const RESULT2PARTS: [&str; 7] = ["29", "83", "13", "24", "42", "14", "76"];
    #[test]
    fn part2_parts() {
        INPUT2.lines().enumerate().for_each(|l| assert_eq!(RESULT2PARTS[l.0], parse_line_2(l.1).to_string()));
    }

    #[test]
    fn part2_merged_words() {
        assert_eq!(parse_line_2("eighthree"), 83);
        assert_eq!(parse_line_2("sevenine"), 79);
        assert_eq!(parse_line_2("twone"), 21);
    }
    
    #[test]
    fn test_part2_manual() {
        assert_eq!(super::part2("x1vzgnpdjtwonert".to_string()), "11".to_string());
    }
}
