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
/// Selects the function for the given day and part
pub fn select_function(day: u32, part: u32) -> Result<fn(String) -> String, String> {
    match day {
        1 => {
            match part {
                1 => Ok(d1::part1),
                2 => Ok(d1::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        2 => {
            match part {
                1 => Ok(d2::part1),
                2 => Ok(d2::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        3 => {
            match part {
                1 => Ok(d3::part1),
                2 => Ok(d3::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        4 => {
            match part {
                1 => Ok(d4::part1),
                2 => Ok(d4::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        5 => {
            match part {
                1 => Ok(d5::part1),
                2 => Ok(d5::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        6 => {
            match part {
                1 => Ok(d6::part1),
                2 => Ok(d6::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        7 => {
            match part {
                1 => Ok(d7::part1),
                2 => Ok(d7::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        8 => {
            match part {
                1 => Ok(d8::part1),
                2 => Ok(d8::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        9 => {
            match part {
                1 => Ok(d9::part1),
                2 => Ok(d9::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        _ => Err("Invalid day!".into()),
    }
}
