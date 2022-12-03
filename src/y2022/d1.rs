//! --- Day 1: Calorie Counting ---

pub fn part1(input: String) -> String {
    // parse input into vec of total calories per elf
    let elves_totals = input
        .trim()
        .split("\n\n") // split into each elf on empty line
        .map(|split| {
            split
                .lines()
                .map(|line| line.parse::<u32>().unwrap()) // parse each line into u32 of calories
                .sum::<u32>() // sum all lines to get this elf´s total calories
        })
        .collect::<Vec<u32>>();

    // return the highest total calories
    elves_totals.iter().max().unwrap().to_string()
}

pub fn part2(input: String) -> String {
    // parse input into vec of total calories per elf
    let elves_totals = input
        .trim()
        .split("\n\n") // split into each elf on empty line
        .map(|split| {
            split
                .lines()
                .map(|line| line.parse::<u32>().unwrap()) // parse each line into u32 of calories
                .sum::<u32>() // sum all lines to get this elf´s total calories
        })
        .collect::<Vec<u32>>();

    // return the sum of the top 3 highest total calories
    let mut top_three = [0u32; 3];
    for total in elves_totals {
        if total > top_three[0] {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = total; 
        }
        else if total > top_three[1] {
            top_three[2] = top_three[1];
            top_three[1] = total;
        }
        else if total > top_three[2] {
            top_three[2] = total;
        }
    }
    top_three.iter().sum::<u32>().to_string()
}
