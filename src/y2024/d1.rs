//! --- Day 1: Historian Hysteria ---
pub fn part1(input: String) -> String {
    let (mut left_column, mut right_column) = parse_input(&input);
    
    left_column.sort();
    right_column.sort();
    
    let mut total_distance = 0;
    for (right, left) in right_column.iter().zip(left_column.iter()) {
        total_distance += (left - right).abs();
    }
    total_distance.to_string()
}

pub fn part2(input: String) -> String {
    let (mut left_column, mut right_column) = parse_input(&input);
    
    // Sort both columns
    left_column.sort();
    right_column.sort();
    
    // Calculate frequencies of values in the right column once
    let mut right_frequencies = Vec::new();
    let mut i = 0;
    while i < right_column.len() {
        let value = right_column[i];
        let mut count = 1;
        i += 1;
        
        while i < right_column.len() && right_column[i] == value {
            count += 1;
            i += 1;
        }
        
        right_frequencies.push((value, count));
    }
    
    // Calculate frequencies of values in the left column
    let mut left_frequencies = Vec::new();
    let mut i = 0;
    while i < left_column.len() {
        let value = left_column[i];
        let mut count = 1;
        i += 1;
        
        while i < left_column.len() && left_column[i] == value {
            count += 1;
            i += 1;
        }
        
        left_frequencies.push((value, count));
    }
    
    // Calculate total with each unique left value
    let mut total = 0;
    for (left_value, left_count) in left_frequencies {
        // Use standard library binary search
        if let Ok(idx) = right_frequencies.binary_search_by_key(&left_value, |&(val, _)| val) {
            let (_, right_count) = right_frequencies[idx];
            total += left_value * left_count * right_count;
        }
        // If the value wasn't found in right_frequencies, no need to add anything to total since it would be multiplied by 0.
    }
    
    total.to_string()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let left_value = parts[0].parse::<i32>().unwrap();
        let right_value = parts[1].parse::<i32>().unwrap();
        
        left_column.push(left_value);
        right_column.push(right_value);
    }
    (left_column, right_column)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    static RESULT1: &str = "11";
    static RESULT2: &str = "31";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT1.to_string()), RESULT1.to_string());
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT1.to_string()), RESULT2.to_string());
    }
}
