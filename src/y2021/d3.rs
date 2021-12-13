//! --- Day 3: Binary Diagnostic ---

pub fn part1(input: String) -> String {
    // initialize the length of the binary numbers
    let mut digits: Vec<u32> = vec![0; input.lines().map(|l| l.chars().count()).max().unwrap() as usize];
    // count the ones
    for line in input.lines() {
        for (i, digit) in line.chars().enumerate() {
            match digit {
                '0' => (),
                '1' => digits[i] += 1,
                _ => panic!("Invalid input"),
            }
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let len = input.lines().count();
    for digit in digits {
        if digit > len as u32 / 2 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();
    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    (gamma_rate * epsilon_rate).to_string()
}


pub fn part2(input: String) -> String {
    // initialize the length of the binary numbers
    let mut digits: Vec<u32> = vec![0; input.lines().map(|l| l.chars().count()).max().unwrap() as usize];
    // count the ones
    for line in input.lines() {
        for (i, digit) in line.chars().enumerate() {
            match digit {
                '0' => (),
                '1' => digits[i] += 1,
                _ => panic!("Invalid input"),
            }
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let len = input.lines().count();
    for digit in digits {
        if digit > len as u32 / 2 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();
    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    (gamma_rate * epsilon_rate).to_string()
}