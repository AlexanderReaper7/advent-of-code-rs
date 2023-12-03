//! Auto-generated file by build script, do not edit!
pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;
pub fn select_function(day: u32, part: u32) -> fn(String) -> String {
    match day {
        1 => {
            match part {
                1 => d1::part1,
                2 => d1::part2,
                _ => panic!("Invalid part!"),
            }
        }
        2 => {
            match part {
                1 => d2::part1,
                2 => d2::part2,
                _ => panic!("Invalid part!"),
            }
        }
        3 => {
            match part {
                1 => d3::part1,
                2 => d3::part2,
                _ => panic!("Invalid part!"),
            }
        }
        4 => {
            match part {
                1 => d4::part1,
                2 => d4::part2,
                _ => panic!("Invalid part!"),
            }
        }
        5 => {
            match part {
                1 => d5::part1,
                2 => d5::part2,
                _ => panic!("Invalid part!"),
            }
        }
        6 => {
            match part {
                1 => d6::part1,
                2 => d6::part2,
                _ => panic!("Invalid part!"),
            }
        }
        7 => {
            match part {
                1 => d7::part1,
                2 => d7::part2,
                _ => panic!("Invalid part!"),
            }
        }
        8 => {
            match part {
                1 => d8::part1,
                2 => d8::part2,
                _ => panic!("Invalid part!"),
            }
        }
        9 => {
            match part {
                1 => d9::part1,
                2 => d9::part2,
                _ => panic!("Invalid part!"),
            }
        }
        _ => panic!("Invalid day!"),
    }
}
