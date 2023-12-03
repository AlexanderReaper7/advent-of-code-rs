//! Advent of Code library
//! 
//! Simplified copy of https://github.com/panicbit/aoc
//! 
//! This library provides functions to get input from the Advent of Code website and to submit solutions.

mod client;
pub use client::Client;
pub(crate) type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;