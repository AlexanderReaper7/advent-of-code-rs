#![feature(iter_array_chunks, iter_next_chunk)]
//! Advent of code challenge https://adventofcode.com/
use std::time::Instant;
use time::Month;
mod auto_import;

fn main() {
    // get input for year day and part
    let (year, day, part) = get_year_day_part();
    println!("Creating client from .session file...");
    let client = create_client();
    println!("Getting input for year {} day {}...", year, day);
    let input = client.get_input(year, day).unwrap();
    // get puzzle input
    println!(
        "Getting function for year {} day {} part {}...",
        year, day, part
    );
    // run puzzle solution
    let func = auto_import::select_function(year, day, part).unwrap();
    println!("Running function...");
    let now = Instant::now();
    let result = func(input);
    println!("Completed in: {}ms", now.elapsed().as_secs_f64() * 1000.0);
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
        let res = client.submit_solution(year, day, part, &result).unwrap();
        println!("{}", res);
    }
}

fn create_client() -> aoc_lib::Client {
    let session_token = std::fs::read_to_string(".session").unwrap();
    aoc_lib::Client::new(session_token).unwrap()
}

fn get_year_day_part() -> (u32, u32, u32) {
    let args = std::env::args().collect::<Vec<String>>();
    let current_time = time::OffsetDateTime::now_utc();
    // get year
    let current_year = current_time.year() as u32;
    let prompt = format!("Enter year(default {}):", current_year);
    let year = match args.get(3) {
        Some(arg) => arg
            .parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input(&prompt, current_year)),
        None => prompt_for_input(&prompt, current_year),
    };
    // get day
    let default_day = if current_time.month() == Month::December {
        current_time.day() as u32
    } else {
        1
    };
    let prompt = format!("Enter day(default {}):", default_day);
    let day = if let Some(arg) = args.get(2) {
        arg.parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input(&prompt, default_day))
    } else {
        prompt_for_input(&prompt, default_day)
    };
    // get part
    let part = if let Some(arg) = args.get(1) {
        arg.parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input("Enter part(default 1):", default_day))
    } else {
        prompt_for_input("Enter part(default 1):", 1)
    };
    (year, day, part)
}

fn prompt_for_input<T: std::str::FromStr>(prompt: &str, default: T) -> T {
    let mut stdin = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut stdin).unwrap();
    stdin.trim().parse::<T>().unwrap_or(default)
}

