//! Auto-generated file by build script, do not edit!
pub mod d1;
pub fn select_function(day: u32, part: u32) -> fn(String) -> String {
    match day {
        1 => {
            match part {
                1 => d1::part1,
                2 => d1::part2,
                _ => panic!("Invalid part!"),
            }
        }
        _ => panic!("Invalid day!"),
    }
}