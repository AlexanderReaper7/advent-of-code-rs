//! Auto-generated file by build script, do not edit!
#[path = "y2021/mod.rs"]
pub mod y2021;

pub fn select_function(year: u32, day: u32, part: u32) -> fn(String) -> String {
	match year {
		2021 => y2021::select_function(day, part),
		_ => panic!("Invalid year!"),
	}
}
