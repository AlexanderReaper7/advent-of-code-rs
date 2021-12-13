//! Advent of code challenge https://adventofcode.com/
use std::process::Command;
use std::io::prelude::*;
mod y2021;

fn main() {
    // get input for year day and part
    let mut stdin = prompt_for_input("Enter year(default 2021):");
    let year = stdin.trim().parse::<u32>().unwrap_or(2021);
    stdin = prompt_for_input("Enter day(default 1):");
    let day = stdin.trim().parse::<u32>().unwrap_or(1);
    stdin = prompt_for_input("Enter part(default 1):");
    let part = stdin.trim().parse::<u32>().unwrap_or(1);
    let input: String = get_input_from_website(year, day);
    let func = select_function(year, day, part);
    let result = func(input);
    println!("Result: {}", result);
    stdin = prompt_for_input("--- press enter to exit or type anything to submit answer ---");
    if stdin.trim().len() > 0 {
        submit_result(result, year, day, part);
    }
}

fn get_input_from_website(year: u32, day: u32) -> String {
    let _ = Command::new("aoc")
    .arg("download")
    .arg("-s").arg(".session")
    .arg("-y").arg(format!("{}", year))
    .arg("-d").arg(format!("{}", day))
    .spawn().unwrap().wait();

    let mut file = std::fs::File::open("input").unwrap();
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    std::fs::remove_file("input").unwrap();
    output
}

fn select_function(year: u32, day: u32, part: u32) -> fn(String) -> String {
    match year {
        2021 => y2021::select_function(day, part),
        _ => panic!("Year not found"),
    }
}

fn prompt_for_input(prompt: &str) -> String {
    let mut stdin = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut stdin).unwrap();
    stdin
}

fn submit_result(result: String, year: u32, day: u32, part: u32) {
    let _ = Command::new("aoc")
    .arg("submit")
    .arg(format!("{}", &part))
    .arg(format!("{}", &result))
    .arg("-s").arg(".session")
    .arg("-y").arg(format!("{}", &year))
    .arg("-d").arg(format!("{}", day))
    .spawn().unwrap().wait();
}

