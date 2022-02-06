//! --- Day 7: The Treachery of Whales ---

pub fn part1(input: String) -> String {
    let crabs: Vec<i32> = input
    .trim()
    .split(",")
    .map(|e| e.parse::<i32>().unwrap())
    .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    let mut fuel_usages = vec![0; (max - min + 1) as usize];
    for position in *min..=*max {
        let mut fuel_needed = 0;
        for crab in crabs.iter() {
            fuel_needed += (*crab - position).abs();
        }
        fuel_usages[(position - min) as usize] = fuel_needed;
    }
    fuel_usages.iter().min().unwrap().to_string()
}

pub fn part2(input: String) -> String {
    let crabs: Vec<i32> = input
    .trim()
    .split(",")
    .map(|e| e.parse::<i32>().unwrap())
    .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    let mut fuel_usages = vec![0; (max - min + 1) as usize];
    for position in *min..=*max {
        let mut fuel_needed = 0;
        for crab in crabs.iter() {
            let distance = (*crab - position).abs();
            fuel_needed += (distance*distance+distance)/2;
        }
        fuel_usages[(position - min) as usize] = fuel_needed;
    }
    fuel_usages.iter().min().unwrap().to_string()
}