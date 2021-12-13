//! --- Day 2: Dive! ---

enum Direction {
    Up,
    Down,
    Forward,
}
impl std::str::FromStr for Direction {

    type Err = &'static str;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err("Failed to parse Direction"),
        }
    }
}

struct Command {
    direction: Direction,
    steps: i32,
}

pub fn part1(input: String) -> String {
    // parse input
    let mut commands: Vec<Command> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        commands.push(Command { direction: parts[0].parse::<Direction>().unwrap(), steps: parts[1].parse::<i32>().unwrap() });
    }
    // execute commands
    let mut depth = 0;
    let mut horizontal = 0;
    for command in commands {
        match command.direction {
            Direction::Up => {
                depth -= command.steps;
            }
            Direction::Down => {
                depth += command.steps;
            }
            Direction::Forward => {
                horizontal += command.steps;
            }
        }
    }
    // return result
    let result = (depth*horizontal).to_string();
    result
}

pub fn part2(input: String) -> String {
    // parse input
    let mut commands: Vec<Command> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        commands.push(Command { direction: parts[0].parse::<Direction>().unwrap(), steps: parts[1].parse::<i32>().unwrap() });
    }
    // execute commands
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for command in commands {
        match command.direction {
            Direction::Up => {
                aim -= command.steps;
            }
            Direction::Down => {
                aim += command.steps;
            }
            Direction::Forward => {
                horizontal += command.steps;
                depth += aim*command.steps;
            }
        }
    }
    // return result
    let result = (depth*horizontal).to_string();
    result
}