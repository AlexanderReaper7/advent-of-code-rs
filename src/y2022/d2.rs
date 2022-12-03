//! --- Day 2: Rock Paper Scissors ---
use std::str::FromStr;

/// represents a choice in rock paper scissors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper,
    Scissors,
}
impl RPS {
    /// Returns the outcome of a game of rock paper scissors
    fn evaluate(self: &RPS, other: &RPS) -> Outcome {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) => Outcome::Win,
            (RPS::Paper, RPS::Rock) => Outcome::Win,
            (RPS::Scissors, RPS::Paper) => Outcome::Win,
            (RPS::Rock, RPS::Paper) => Outcome::Lose,
            (RPS::Paper, RPS::Scissors) => Outcome::Lose,
            (RPS::Scissors, RPS::Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
    /// Returns the RPS that would get the desired outcome
    fn reverse_evaluate(self: &RPS, outcome: &Outcome) -> RPS {
        match outcome {
            Outcome::Win => match self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            Outcome::Lose => match self {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            Outcome::Draw => *self,
        }
    }
}
impl FromStr for RPS {
    type Err = ();

    /// parses a string into a RPS enum
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

/// represents an outcome of a rock paper scissors match
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}
impl FromStr for Outcome {
    type Err = ();

    /// parses a string into an Outcome enum
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

pub fn part1(input: String) -> String {
    // step through input line by line
    input
        .trim()
        .lines()
        .map(|line| {
            // split into two players
            let rps = line
                .split_whitespace()
                .map(|s| {
                    s.parse::<RPS>().expect(
                        format!("error while parsing input, {} is not a valid character", s)
                            .as_str(),
                    )
                })
                .collect::<Vec<RPS>>();
            // return score
            rps[1] as isize + rps[1].evaluate(&rps[0]) as isize
        })
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            // split into two, first column is what the opponent will play, second column is what outcome we want
            let rps = line.split_whitespace().collect::<Vec<&str>>();
            // get what the opponent will play
            let opponent = rps[0].parse::<RPS>().expect(
                format!(
                    "error while parsing input, {} is not a valid character",
                    rps[0]
                )
                .as_str(),
            );
            // get the outcome we want
            let outcome = rps[1].parse::<Outcome>().expect(
                format!(
                    "error while parsing input, {} is not a valid character",
                    rps[1]
                )
                .as_str(),
            );
            // calculate what we need to play to get the desired outcome
            let choice = opponent.reverse_evaluate(&outcome);
            // return score
            choice as isize + outcome as isize
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("A Y".to_string()), "8");
        assert_eq!(part1("B X".to_string()), "1");
        assert_eq!(part1("C Z".to_string()), "6");
        assert_eq!(part1("A Y\nB X\nC Z\n".to_string()), "15")
    }
}
