//! --- Day 3: Binary Diagnostic ---

pub fn part1(input: String) -> String {
    // initialize the length of the binary numbers
    let mut digits: Vec<u32> =
        vec![0; input.lines().map(|l| l.chars().count()).max().unwrap() as usize];
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
    let chars_len = input.lines().map(|l| l.chars().count()).max().unwrap() as usize;
    let mut valid_lines: Vec<&str> = input.lines().clone().collect();
    let mut j = 0;
    while valid_lines.len() > 1 && j < chars_len {
        // find mask
        let zeros = valid_lines
            .iter()
            .map(|l| l.chars().nth(j).unwrap())
            .filter(|c| *c == '0')
            .count();
        let ones = valid_lines.len() - zeros;
        let mask;
        if zeros >= ones {
            mask = '0'
        } else {
            mask = '1'
        }
        // remove lines whose j:th char don't match the mask
        let mut i: i32 = (valid_lines.len() - 1).try_into().unwrap();
        while i > -1 {
            if valid_lines[i as usize].chars().nth(j).unwrap() != mask {
                valid_lines.remove(i as usize);
            }
            i -= 1;
        }
        j += 1;
    }
    let oxygen_rating = u32::from_str_radix(valid_lines[0], 2).unwrap();

    valid_lines = input.lines().clone().collect();
    j = 0;
    while valid_lines.len() > 1 && j < chars_len {
        // find mask
        let zeros = valid_lines
            .iter()
            .map(|l| l.chars().nth(j).unwrap())
            .filter(|c| *c == '0')
            .count();
        let ones = valid_lines.len() - zeros;
        let mask;
        if zeros <= ones {
            mask = '0'
        } else {
            mask = '1'
        }
        // remove lines whose j:th char don't match the mask
        let mut i: i32 = (valid_lines.len() - 1).try_into().unwrap();
        while i > -1 {
            if valid_lines[i as usize].chars().nth(j).unwrap() != mask {
                valid_lines.remove(i as usize);
            }
            i -= 1;
        }
        j += 1;
    }
    let co2_rating = u32::from_str_radix(valid_lines[0], 2).unwrap();
    (oxygen_rating * co2_rating).to_string()
}

#[test]
fn test_part2() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(part2(input.to_string()), "230");
}
