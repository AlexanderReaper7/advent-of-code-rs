//! Auto-generated file by build script, do not edit!
pub mod d1;
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
        _ => Err("Invalid day!".into()),
    }
}
