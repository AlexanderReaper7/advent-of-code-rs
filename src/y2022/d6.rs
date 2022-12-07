//! --- Day 6: Tuning Trouble ---

pub fn part1(input: String) -> String {
    // move a sliding window of size 4 over the input string until all characters are different, return the number of processed chars as a string
    const WINDOW_SIZE: usize = 4;
    (input.chars().collect::<Vec<char>>().windows(WINDOW_SIZE).position(|window| window.iter().all(|c| window.iter().filter(|other| c == *other).count() < 2)).unwrap() + WINDOW_SIZE).to_string()
}

pub fn part2(input: String) -> String {
    const WINDOW_SIZE: usize = 14;
    (input.chars().collect::<Vec<char>>().windows(WINDOW_SIZE).position(|window| window.iter().all(|c| window.iter().filter(|other| c == *other).count() < 2)).unwrap() + WINDOW_SIZE).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "7".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "19".to_string());
    }
}
