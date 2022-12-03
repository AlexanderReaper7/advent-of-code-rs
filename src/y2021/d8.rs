#![allow(dead_code)]
//! 

use std::ops::Index;

// fn parse_input(input: String) ->  {
//     let split = input.trim_end().split(" | ").collect::<Vec<&str>>();
//     let in_values = parse_values(split[0]);
//     let out_values = parse_values(split[1]);
//     (in_values, out_values)
// }

// fn parse_values(input: &str) -> Vec<Result<u8, &str>> {
//     input
//     .split(" ")
//     .map(|s| s.chars().collect::<Vec<char>>())
//     .map(|mut v| {
//         v.sort();
//         v.into_iter().collect::<String>()
//     })
//     .map(|s| parse_7segment_by_length(s.as_str()))
//     .collect::<Vec<Result<u8, &str>>>()
// }

fn chars_to_num(input: &str) -> u8 {
    match input {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("Invalid input"),
    }
}

fn num_to_chars(input: u8) -> &'static str {
    match input {
        0 => "abcefg",
        1 => "cf",
        2 => "acdeg",
        3 => "acdfg",
        4 => "bcdf",
        5 => "abdfg",
        6 => "abdefg",
        7 => "acf",
        8 => "abcdefg",
        9 => "abcdfg",
        _ => panic!("Invalid input"),
    }
}

fn parse_7segment_by_length(input: &str) -> Result<u8, &str> {
    match input.len() {
        2 => Ok(1),
        3 => Ok(7),
        4 => Ok(4),
        7 => Ok(8),
        _ => Err(input),
    }
}

pub fn part1(input: String) -> String {
    let out_values = input
    .trim_end()
    .lines()
    .map(|l| 
        l.split(" | ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| parse_7segment_by_length(s))
        .collect::<Vec<Result<u8, &str>>>())
    .collect::<Vec<Vec<Result<u8, &str>>>>();

    out_values.iter().flatten().filter(|v| v.is_ok()).count().to_string()
}

pub fn part2(input: String) -> String {
    let values = input
    .trim_end()
    .lines()
    .map(|l| l.split(" | ").collect::<Vec<&str>>())
    .map(|s| (s.index(0).split(" ").collect::<Vec<&str>>(), s.index(0).split(" ").collect::<Vec<&str>>()))
    .collect::<Vec<(Vec<&str>, Vec<&str>)>>();
    let mut sum = 0;
    for value in values {
        let out_values = value.1.iter().map(|v| parse_7segment_by_length(v)).collect::<Vec<Result<u8, &str>>>();
        let out_values = out_values.iter().map(|v| v.unwrap_or_else( |input| {
            // find which segments are ambiguous
            match input.len() {
                5 => {
                    // if the length is 5 the possible digits are 2 3 5
                    // the ambiguous segments are b c e f
                    // b: 5, c: 2 3, e: 2, f: 5 3,
                    let fives: Vec<&&str> = value.0.iter().filter(|v| v.len() == 5).collect();
                    // those with length 1 is either 5 or 2, respective unique segments b and e
                    let uniques = fives.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let mut result = String::new();
                        for char in v.chars() {
                            // if this char is not contained in any other char, add it to output and return(each digit has at most 1 unique char)
                            if !fives[(i+1) % 3].contains(char) && !fives[(i+2) % 3].contains(char) {
                                result.push(char);
                                break;
                            }
                        }
                        result
                    })
                    .collect::<Vec<String>>();
                    // 5@ - *4 = 0
                    // if a unique of len 1 becomes len 0 when 4 is removed from unique it is 5
                    let four = value.0.iter().filter(|v| v.len() == 4).next().unwrap();
                    for unique in uniques {
                        if unique.len() == 1 {
                            for char in four.chars() {
                                if unique.contains(char) {
                                    // this is 5
                                    return 5;
                                } else {
                                    // this is 2
                                    return 2;
                                }
                            }
                        } else {
                            return 3;
                        }
                    }
                    panic!("why?")
                }
                6 => {
                    // if the length is 6 the possible digits are 0 6 9
                    // the ambiguous segments are c d e
                    // c: 0 9, d: 6 9, e: 0 6
                    // remove mutual chars among those with len 6
                    let sixes = value.0.iter().filter(|v| v.len() == 6).collect::<Vec<&&str>>();
                    let reduced = input
                    .chars()
                    .filter(|c| !sixes.iter().all(|s| s.contains(*c)))
                    .collect::<String>();
                    // if removing 1(c) from digit results in len 2, the digit is 6
                    let one = value.0.iter().filter(|v| v.len() == 2).next().unwrap();
                    let res = reduced.chars().filter(|c| !one.contains(*c)).collect::<String>();
                    if res.len() == 2 {
                        return 6;
                    }
                    // if removing 4(cd) from digits and len is 0, the digit is 9
                    let four = value.0.iter().filter(|v| v.len() == 4).next().unwrap();
                    let res = reduced.chars().filter(|c| !four.contains(*c)).collect::<String>();
                    if res.len() == 0 {
                        return 9;
                    }
                    return 0
                }
                _ => panic!("Invalid input length"),
            }
        }))
        .collect::<Vec<u8>>();
        // collect into 4 digit number
        sum += out_values.iter().rev().enumerate().map(|(i, d)| (*d as i128) * 10_i128.pow(i as u32)).sum::<i128>();
    }
    sum.to_string()
}
// segment positions top left to bottom right

// finding segments, to find segment subtract digts from the first
// a: 7 - 1
// b: 5 - 2 - 3
// c: 1 - 5
// d: 4 - 1 - 
// e: 8 - 9

// identities:
// 0: abcefg 
// *1: cf
// 2: acdeg
// 3: acdfg
// *4: bcdf
// 5: abdfg
// 6: abdefg
// *7: acf
// *8: abcdefg
// 9: abcdfg
// counts:
// a: 8
// b: 6
// c: 8
// d: 7
// e: 4
// f: 9
// g: 7
// ----
// 0: 6 #
// *1: 2
// 2: 5 @
// 3: 5 @
// *4: 4
// 5: 5 @
// 6: 6 #
// *7: 3
// *8: 7
// 9: 6 #