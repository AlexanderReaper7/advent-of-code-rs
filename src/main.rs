//! Advent of code challenge https://adventofcode.com/
use std::io::prelude::*;
use std::process::Command;
use std::time::Instant;
mod auto_import;

fn main() {
    // get input for year day and part
    let args = std::env::args().collect::<Vec<String>>();
    // get year
    let year = match args.get(3) {
        Some(arg) => arg
            .parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input("Enter year(default 2022):", 1)),
        None => prompt_for_input("Enter year(default 2022):", 2022),
    };
    // get day
    let day = if let Some(arg) = args.get(2) {
        arg.parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input("Enter day(default 1):", 1))
    } else {
        prompt_for_input("Enter day(default 1):", 1)
    };
    // get part
    let part = if let Some(arg) = args.get(1) {
        arg.parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input("Enter part(default 1):", 1))
    } else {
        prompt_for_input("Enter part(default 1):", 1)
    };
    // get puzzle input
    let input: String = get_input_from_website(year, day);
    println!(
        "getting function for year {} day {} part {}...",
        year, day, part
    );
    let func = auto_import::select_function(year, day, part);
    // run puzzle solution
    println!("running function...");
    let now = Instant::now();
    let result = func(input);
    println!("completed in: {}ms", now.elapsed().as_secs_f64() * 1000.0);
    // print result
    println!("Result: {}", result);
    // submit result?
    if prompt_for_input(
        "Press enter to exit or type anything to submit answer",
        "".to_string(),
    )
    .len()
        > 0
    {
        submit_result(result, year, day, part);
    }
}

fn get_input_from_website(year: u32, day: u32) -> String {
    let _ = Command::new("aoc")
        .arg("download")
        .arg("-s")
        .arg(".session")
        .arg("-y")
        .arg(format!("{}", year))
        .arg("-d")
        .arg(format!("{}", day))
        .spawn()
        .unwrap()
        .wait();

    let mut file = std::fs::File::open("input").unwrap();
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    std::fs::remove_file("input").unwrap();
    output
}

fn prompt_for_input<T: std::str::FromStr>(prompt: &str, default: T) -> T {
    let mut stdin = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut stdin).unwrap();
    stdin.trim().parse::<T>().unwrap_or(default)
}

fn submit_result(result: String, year: u32, day: u32, part: u32) {
    let _ = Command::new("aoc")
        .arg("submit")
        .arg(&part.to_string())
        .arg(&result)
        .arg("-s")
        .arg(".session")
        .arg("-y")
        .arg(format!("{}", &year))
        .arg("-d")
        .arg(format!("{}", day))
        .spawn()
        .unwrap()
        .wait();
}
