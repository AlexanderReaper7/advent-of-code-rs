//! container module for solutions of year 2021
pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;

pub fn select_function(day: u32, part: u32) -> fn(String) -> String {
    match day {
        1 => match part {
            1 => d1::part1,
            2 => d1::part2,
            _ => panic!("Part not found"),
        },
        2 => match part {
            1 => d2::part1,
            2 => d2::part2,
            _ => panic!("Part not found"),
        },
        3 => match part {
            1 => d3::part1,
            2 => d3::part2,
            _ => panic!("Part not found"),
        },
        4 => match part {
            1 => d4::part1,
            2 => d4::part2,
            _ => panic!("Part not found"),
        },
        _ => panic!("Day not found"),
    }
}