//! --- Day 2: Red-Nosed Reports ---
pub fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: String) -> String {
    let reports = parse_input(input);
    let mut safe_count = 0;

    for report in reports {
        if report.is_empty() {
            panic!("Unexpected empty report {:?}", report); // Skip empty reports
        }
        let mut increasing = true;
        let mut decreasing = true;
        let mut safe = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i] - report[i + 1]).abs();
            if diff < 1 || diff > 3 {
                safe = false; // Not safe if adjacent levels differ by less than 1 or more than 3
                break;
            }
            if report[i] < report[i + 1] {
                decreasing = false;
            } else if report[i] > report[i + 1] {
                increasing = false;
            }
        }

        safe = (increasing || decreasing) && safe; // Safe if the levels are all increasing or all decreasing

        if safe {
            safe_count += 1;
        }
    }
    safe_count.to_string()
}

/// A report only counts as safe if both of the following are true:
///  - The levels are either all increasing or all decreasing.
///  - Any two adjacent levels differ by at least one and at most three.
fn is_safe(report: &[i32]) -> bool {
    if report.is_empty() {
        return false; // An empty report is not safe
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = (report[i] - report[i + 1]).abs();
        if diff < 1 || diff > 3 {
            return false; // Not safe if adjacent levels differ by less than 1 or more than 3
        }
        if report[i] < report[i + 1] {
            decreasing = false;
        } else if report[i] > report[i + 1] {
            increasing = false;
        }
    }

    increasing || decreasing // Safe if the levels are all increasing or all decreasing
}

pub fn part2(input: String) -> String {
    unimplemented!()
}
#[cfg(test)]
mod tests {
    use super::*;
    // The unusual data (your puzzle input) consists of many reports, one report per line.
    // Each report is a list of numbers called levels that are separated by spaces. For example:
    static INPUT1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    /*
    A report only counts as safe if both of the following are true:
     - The levels are either all increasing or all decreasing.
     - Any two adjacent levels differ by at least one and at most three.

    In the example above, the reports can be found safe or unsafe by checking those rules:

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

    So, in this example, 2 reports are safe.

    Analyze the unusual data from the engineers. How many reports are safe?
    */
    static RESULT1: &str = "2";
    static INPUT2: &str = "";
    static RESULT2: &str = "";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT1.to_string()), RESULT1.to_string());
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT2.to_string()), RESULT2.to_string());
    }
}
