use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};

use reqwest::header::COOKIE;

use crate::Result;

/// Advent of Code client
/// 
/// This client is used to get input from the Advent of Code website and to submit solutions.
/// 
/// # Example
/// 
/// ```rust
/// use aoc_lib::Client;
/// 
/// fn main() {
///   let session_token = std::fs::read_to_string(".session").unwrap();
///   let client = Client::new(session_token).unwrap();
///   
///   let input = client.get_input(2019, 1).unwrap();
///   let result = client.submit_solution(2019, 1, 1, "solution").unwrap();
///   println!("{}", result);
/// }
pub struct Client {
    session_token: String,
    client: reqwest::blocking::Client,
    cache_dir: std::path::PathBuf,
}

impl Client {
    pub fn new(session_token: String) -> Result<Self>
    {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "This OS is not supported"))?
            .join("advent_of_code/");

        fs::create_dir_all(&cache_dir).map_err(|err| {
            eprintln!(
                "Failed to create cache dir \"{}\": {}",
                cache_dir.display(),
                err
            );
            err
        })?;

        Ok(Self {
            cache_dir,
            session_token,
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn get_input(&self, year: u32, day: u32) -> Result<String> {
        if let Ok(input) = self.get_cached_input(year, day) {
            return Ok(input);
        }

        let input = self.download_input(year, day)?;
        self.cache_input(year, day, &input)?;

        Ok(input)
    }

    fn get_cached_input(&self, year: u32, day: u32) -> Result<String> {
        let path = self.cache_dir.join(format!("y{}/d{}.txt", year, day));
        Ok(std::fs::read_to_string(path)?)
    }

    fn cache_input(&self, year: u32, day: u32, input: &str) -> Result<()> {
        let path = self.cache_dir.join(format!("y{}/d{}.txt", year, day));
        // create the year folder if it doesn't exist
        fs::create_dir_all(path.parent().unwrap())?;

        let mut file = File::create(path)?;

        file.write_all(input.as_bytes())?;

        Ok(())
    }

    fn download_input(&self, year: u32, day: u32) -> Result<String> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let cookie = format!("session={}", self.session_token);
        let input = self
            .client
            .get(&url)
            .header(COOKIE, cookie)
            .send()?
            .error_for_status()?
            .text()?;

        Ok(input)
    }

    pub fn submit_solution(&self, year: u32, day: u32, part: u32, solution: &str) -> Result<String> {
        use select::document::Document;
        use select::predicate::Name;

        let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
        let cookie = format!("session={}", self.session_token);

        let mut params = HashMap::new();
        params.insert("level", part.to_string());
        params.insert("answer", solution.into());

        let response = self
            .client
            .post(&url)
            .header(COOKIE, cookie)
            .form(&params)
            .send()?
            .error_for_status()?
            .text()?;

        let doc = Document::from(response.as_str());
        let node = doc.find(Name("main")).next().unwrap();
        let text = node.text();
        // let text = text.trim().split(".  ").next().unwrap_or("");
        let text = format!("{}.", text.trim());

        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        // WARN: this is a relative path to the .session file, you may need to change this
        let session_token = std::fs::read_to_string("../.session").unwrap();
        let client = Client::new(session_token).unwrap();
        let _ = client.get_input(2021, 1).unwrap();
    }

    #[test]
    fn test_caching() {
        let year = 1000;
        let day = 100;
        let client = Client::new("no session".into()).unwrap();
        client.cache_input(year, day, "test").unwrap();
        let res = client.get_cached_input(year, day).unwrap();
        assert_eq!(res, "test");
    }
}