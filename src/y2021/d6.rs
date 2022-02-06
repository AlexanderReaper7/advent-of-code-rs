//! --- Day 6: Lanternfish ---

use rayon::prelude::*;
pub fn part1(input: String) -> String {
    // parse input
    let mut lanternfish: Vec<u8> = input
        .trim()
        .split(",")
        .map(|e| e.parse::<u8>().unwrap())
        .collect();
    let iterations = 80;
    let start_days = 8u8;
    let reset_days = 6u8;

    for iteration in 0..iterations {
        let new_count: usize = lanternfish
            .par_iter_mut()
            .map(|fish| {
                if *fish == 0 {
                    *fish = reset_days as u8;
                    return 1;
                }
                *fish -= 1;
                0
            })
            .sum();
        lanternfish.append(&mut vec![start_days; new_count]);
        println!(
            "Iteration: {}, added {} new lanternfish.",
            iteration, new_count
        );
    }
    lanternfish.len().to_string()
}

pub fn part2(input: String) -> String {
    //parse
    let lanternfish: Vec<u8> = input
        .trim()
        .split(",")
        .map(|e| e.parse::<u8>().unwrap())
        .collect();
    let iterations = 256;
    let start_days = 8u8;
    let reset_days = 6u8;

    // initialize
    let mut fish_in_days_left: Vec<u128> = vec![0; (start_days + 1) as usize];
    for fish in lanternfish {
        fish_in_days_left[fish as usize] += 1;
    }

    // simulate
    for iteration in 0..iterations {
        let new_count = fish_in_days_left[0];
        fish_in_days_left.rotate_left(1);
        fish_in_days_left[start_days as usize] = new_count;
        fish_in_days_left[reset_days as usize] += new_count;
        println!("Iteration: {}, fish: {:?}", iteration, fish_in_days_left);
    }
    fish_in_days_left.iter().sum::<u128>().to_string()
}
