//! --- Day 4: Camp Cleanup ---

use std::ops::RangeInclusive;

fn parse_into_range_pairs(input: &str) -> Vec<[RangeInclusive<u32>; 2]> {
    input
        .trim()
        .lines()
        .map(|line| {
            // split into pair
            line.split(',')
                .map(|pair| {
                    let bound_pair = pair
                        .split('-') // split into lower and upper bounds
                        .map(|bound| bound.parse::<u32>().unwrap())
                        .next_chunk::<2>()
                        .unwrap();
                    bound_pair[0]..=bound_pair[1]
                })
                .next_chunk::<2>()
                .unwrap()
        })
        .collect::<Vec<[RangeInclusive<u32>; 2]>>()
}

pub fn part1(input: String) -> String {
    // break input into ranges
    let range_pairs = parse_into_range_pairs(&input);
    // check how many of the ranges is completely contained in the other
    range_pairs
        .iter()
        .filter(|range_pair| {
            (range_pair[0].contains(&range_pair[1].start())
                && range_pair[0].contains(&range_pair[1].end()))
                || (range_pair[1].contains(&range_pair[0].start())
                    && range_pair[1].contains(&range_pair[0].end()))
        })
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    // break input into ranges
    let range_pairs = parse_into_range_pairs(&input);
    // check how many of the ranges is partially or completely contained in the other
    range_pairs
        .iter()
        .filter(|range_pair| {
            (range_pair[0].contains(&range_pair[1].start())
                || range_pair[0].contains(&range_pair[1].end()))
                || (range_pair[1].contains(&range_pair[0].start())
                    || range_pair[1].contains(&range_pair[0].end()))
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "2-4,6-8\n\
    2-3,4-5\n\
    5-7,7-9\n\
    2-8,3-7\n\
    6-6,4-6\n\
    2-6,4-8\n";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "2".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "4".to_string());
    }
}
