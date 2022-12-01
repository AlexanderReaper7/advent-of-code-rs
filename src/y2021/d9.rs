//! --- Day 9: Smoke Basin ---

use std::{vec, str::FromStr};

pub fn part1(input: String) -> String {
    // convert string into 2d array of digits
    let input = input.trim().lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    // map which digits should be checked for smaller neighbors
    let mut lowest_points: Vec<(usize, usize)> = Vec::new();
    // check for neighbors with lesser digit
    for y in 0..input.len() {
        'x: for x in 0..input[y].len() {
            for (dy, dx) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                // check if neighbor exists
                if y as isize + dy < 0 || y as isize + dy >= input.len() as isize || x as isize + dx < 0 || x as isize + dx >= input[y].len() as isize {
                    continue;
                }
                // check if neighbor is lesser
                if input[(y as isize + dy) as usize][(x as isize + dx) as usize] < input[y][x] {
                    // skip to next digit if this digit has a smaller neighbor
                    continue 'x;
                }
            }
            // else add point to lowest_points
            lowest_points.push((y, x));
        }
    }
    println!("{:?}", lowest_points);
    // add up the risk level
    lowest_points.iter().map(|(y, x)| (input[*y][*x] + 1) as u64).sum::<u64>().to_string()
}

#[test]
fn test_part1() {
    let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
    assert_eq!(part1(String::from(input)), String::from("15"));
}

pub fn part2(input: String) -> String {
    unimplemented!()
}