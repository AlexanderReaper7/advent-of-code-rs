//! --- Day 1: Sonar Sweep ---

fn parse_input(input: String) -> Vec<i32> {
    input.split('\n')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

/// count the number of times the next number is greater than the last
pub fn part1(input: String) -> String {
    // parse input into vector of ints
    let values = parse_input(input);
    if values.len() < 2 {
        panic!("input too short, need at least 2 numbers");
    }
    let mut result = 0;
    // count number of times the new number is larger than the previous
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            result += 1;
        }
    }
    // return result
    result.to_string()
}

/// count the number of times the next number is greater than the last in a 3 wide window
pub fn part2(input: String) -> String {
    // parse input into vector of ints
    let values = parse_input(input);
    if values.len() < 4 {
        panic!("input too short, need at least 4 numbers");
    }
    let mut new_values = Vec::with_capacity(values.len() - 2);
    // make a 3 wide window of values
    for i in 2..values.len() {
        new_values.push(values[i - 2] + values[i - 1] + values[i]);
    }
    let mut result = 0;
    // count number of times the new number is larger than the previous
    for i in 1..new_values.len() {
        if new_values[i] > new_values[i - 1] {
            result += 1;
        }
    }
    // return result
    result.to_string()
}
