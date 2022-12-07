//! --- Day 5: Supply Stacks ---

pub fn part1(input: String) -> String {
    let mut state = parse_initial_state(&input);
    let moves = parse_moves(&input);

    // perform the moves
    for (quantity, source, destination) in moves {
        let source = source - 1;
        let destination = destination - 1;
        let height = state[source].len();
        let mut to_move = state[source]
            .drain(height - quantity..)
            .collect::<Vec<char>>();
        to_move.reverse();
        state[destination].append(&mut to_move);
    }

    // return the top layer of each stack as a string
    state
        .iter()
        .map(|stack| stack.last().expect("should not be empty"))
        .collect::<String>()
}

pub fn part2(input: String) -> String {
    let mut state = parse_initial_state(&input);
    let moves = parse_moves(&input);

    // perform the moves
    for (quantity, source, destination) in moves {
        let source = source - 1;
        let destination = destination - 1;
        let height = state[source].len();
        let mut to_move = state[source]
            .drain(height - quantity..)
            .collect::<Vec<char>>();
        state[destination].append(&mut to_move);
    }

    // return the top layer of each stack as a string
    state
        .iter()
        .map(|stack| stack.last().expect("should not be empty"))
        .collect::<String>()
}

/// Parse the initial state of the stacks from the input string
fn parse_initial_state(input: &str) -> Vec<Vec<char>> {
    // count the number of lines until the index number is hit in the second index of the line
    let crate_lines = input
        .lines()
        .take_while(|line| line.chars().nth(1).expect("should not be empty") != '1')
        .collect::<Vec<&str>>();
    let height = crate_lines.len();
    const STEPS_BETWEEN_CRATES: usize = 4;
    let width = (crate_lines[0].len() / STEPS_BETWEEN_CRATES) + 1;
    // [width][height]
    let mut state: Vec<Vec<char>> = vec![Vec::with_capacity(height); width];
    for line in crate_lines.iter().rev() {
        line.chars()
            .skip(1)
            .step_by(STEPS_BETWEEN_CRATES)
            .enumerate()
            .for_each(|(w, c)| {
                if c != ' ' {
                    state[w].push(c)
                }
            });
    }
    state
}

/// Parse the moves from the input string
/// # Returns
/// A vector of tuples containing the quantity to move, the source stack and the destination stack
fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let mut moves = Vec::new();

    // skip all lines until the first empty line (the beginning of the moves)
    for line in input.split("\n\n").nth(1).unwrap().lines() {
        let mut parts = line.split_whitespace();
        let quantity = parts
            .nth(1)
            .expect("should not be empty")
            .parse::<usize>()
            .expect("should be a positive number");
        let source = parts
            .nth(1)
            .expect("should not be empty")
            .parse::<usize>()
            .expect("should be a positive number");
        let destination = parts
            .nth(1)
            .expect("should not be empty")
            .parse::<usize>()
            .expect("should be a positive number");
        moves.push((quantity, source, destination));
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn test_parse_initial_state() {
        let state = parse_initial_state(INPUT);
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(state, expected);
    }

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves(INPUT);
        let expected = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "MCD".to_string());
    }
}
