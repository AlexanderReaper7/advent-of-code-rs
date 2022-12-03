
//! --- Day 3: Rucksack Reorganization ---
// Each rucksack has two large compartments.
// All items of a given type are meant to go into exactly one of the two compartments.
// the first half of a rucksack is the first compartment, the second half is the second compartment
// each rucksack has one error

pub fn part1(input: String) -> String {
    input.trim().lines().map(|rucksack| {
        // split rucksack in half into compartments
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        // get the duplicate across compartments
        let duplicate = first
            .chars()
            .filter(|c| second.contains(*c))
            .collect::<String>();
        // return the priority for the duplicate
        duplicate[0..1].to_string()
            .chars()
            .map(|c| if c.is_ascii_uppercase() {c as u32 - 64+26} else {c as u32 - 96})
            .sum::<u32>()
    }).sum::<u32>().to_string()
}
// every three rucksacks is a group
// each rucksack in a group contains one common item
// the common item is the badge
pub fn part2(input: String) -> String {
    // split into groups
    input.trim().lines().array_chunks::<3>().map(|group| {
        // get the common item
        let common = group[0].chars().filter(|c| group[1].contains(*c) && group[2].contains(*c)).collect::<String>();
        // return the priority for the common item
        common[0..1].to_string()
            .chars()
            .map(|c| if c.is_ascii_uppercase() {c as u32 - 64+26} else {c as u32 - 96})
            .sum::<u32>()
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "157".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "70".to_string());
    }
}
